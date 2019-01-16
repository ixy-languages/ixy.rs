use std::cell::RefCell;
use std::collections::VecDeque;
use std::error::Error;
use std::fs;
use std::fs::{File, OpenOptions};
use std::io::{Seek, SeekFrom};
use std::mem;
use std::os::unix::io::{AsRawFd, FromRawFd};
use std::os::unix::io::RawFd;
use std::path::Path;
use std::ptr;
use std::rc::Rc;
use std::thread;
use std::time::{Duration, Instant};

use byteorder::{NativeEndian, ReadBytesExt, WriteBytesExt};

use constants::*;
use memory::*;
use pci::*;

use libc;
use DeviceStats;
use IxyDevice;
use MAX_QUEUES;

const DRIVER_NAME: &str = "ixy-ixgbe";

const NUM_RX_QUEUE_ENTRIES: usize = 512;
const NUM_TX_QUEUE_ENTRIES: usize = 512;
const TX_CLEAN_BATCH: usize = 32;

/* constants needed for IOMMU. Grabbed from linux/vfio.h */
const VFIO_GET_API_VERSION: u64 = 15204;
const VFIO_CHECK_EXTENSION: u64 = 15205;
const VFIO_SET_IOMMU: u64 = 15206;
const VFIO_GROUP_GET_STATUS: u64 = 15207;
const VFIO_GROUP_SET_CONTAINER: u64 = 15208;
const VFIO_GROUP_GET_DEVICE_FD:u64 = 15210;
const VFIO_DEVICE_GET_REGION_INFO:u64 = 15212;
//const VFIO_IOMMU_GET_INFO: u64 = 15216;

const VFIO_API_VERSION: i32 = 0;
const VFIO_TYPE1_IOMMU: u64 = 1;
const VFIO_GROUP_FLAGS_VIABLE: u32 = 1;
//const VFIO_GROUP_FLAGS_CONTAINER_SET: u32 = 2;
const VFIO_PCI_CONFIG_REGION_INDEX: u32 = 7;
const VFIO_PCI_BAR0_REGION_INDEX: u32 = 0;

/* struct vfio_group_status, grabbed from linux/vfio.h */
struct vfio_group_status {
    argsz: u32,
    flags: u32,
}

/* struct vfio_region_info, grabbed from linux/vfio.h */
#[repr(C)]
struct vfio_region_info {
    argsz: u32,
    flags: u32,
    index: u32,
    cap_offset: u32,
    size: u64,
    offset: u64,
}

/* struct vfio_iommu_type1_info, grabbed from linux/vfio.h */
// struct vfio_iommu_type1_info {
//     argsz: u32,
//     flags: u32,
//     iova_pgsizes: u64,
// }

/* struct vfio_device_info, grabbed from linux/vfio.h */
// struct vfio_device_info {
//     argsz: u32,
//     flags: u32,
//     num_regions: u32,
//     num_irqs: u32,
// }

fn wrap_ring(index: usize, ring_size: usize) -> usize {
    (index + 1) & (ring_size - 1)
}

pub struct IxgbeDevice {
    pci_addr: String,
    addr: *mut u8,
    len: usize,
    num_rx_queues: u16,
    num_tx_queues: u16,
    rx_queues: Vec<IxgbeRxQueue>,
    tx_queues: Vec<IxgbeTxQueue>,
    pub iommu: bool,
    vfio_device_file_descriptor: RawFd,
    vfio_group_file: Option<File>,
    pub gfd: RawFd,
    pub vfio_container_file: Option<File>,
    pub cfd: RawFd,
}

struct IxgbeRxQueue {
    descriptors: *mut ixgbe_adv_rx_desc,
    num_descriptors: usize,
    pool: Rc<RefCell<Mempool>>,
    bufs_in_use: Vec<usize>,
    rx_index: usize,
}

struct IxgbeTxQueue {
    descriptors: *mut ixgbe_adv_tx_desc,
    num_descriptors: usize,
    pool: Option<Rc<RefCell<Mempool>>>,
    bufs_in_use: VecDeque<usize>,
    clean_index: usize,
    tx_index: usize,
}

impl IxyDevice for IxgbeDevice {
    /// Returns an initialized `IxgbeDevice` on success.
    /// 
    /// # Panics
    /// Panics if `num_rx_queues` or `num_tx_queues` exceeds `MAX_QUEUES`.
    fn init(
        pci_addr: &str,
        num_rx_queues: u16,
        num_tx_queues: u16,
    ) -> Result<IxgbeDevice, Box<Error>> {
        if unsafe { libc::getuid() } != 0 {
            warn!("not running as root, this will probably fail");
        }

        assert!(
            num_rx_queues <= MAX_QUEUES,
            "cannot configure {} rx queues: limit is {}",
            num_rx_queues,
            MAX_QUEUES
        );
        assert!(
            num_tx_queues <= MAX_QUEUES,
            "cannot configure {} tx queues: limit is {}",
            num_tx_queues,
            MAX_QUEUES
        );

        // check if iommu is activated
        // iommu is activated if there is a iommu_group symlink in /sys/bus/pci/devices/$pci_addr
        let iommu = Path::new(&format!("/sys/bus/pci/devices/{}/iommu_group", pci_addr)).exists();
        // ToDo (stefan.huber@stusta.de): unload ixgbe driver, load vfio driver (nicetohave)
        let device_file_descriptor: RawFd;
        let group_file: Option<File>;
        let gfd: RawFd;
        let container_file: Option<File>;
        let cfd: RawFd;
        let addr: *mut u8;
        let len: usize;
        if iommu {
            /* we also have to build these vfio structs... */
            let group_status: vfio_group_status = vfio_group_status { argsz: mem::size_of::<vfio_group_status> as u32, flags: 0, };
            //let mut iommu_info: vfio_iommu_type1_info = vfio_iommu_type1_info { argsz: mem::size_of::<vfio_iommu_type1_info> as u32, flags: 0, iova_pgsizes: 0, };
            //let mut device_info: vfio_device_info = vfio_device_info { argsz: mem::size_of::<vfio_device_info> as u32, flags: 0, num_irqs: 0, num_regions: 0, };

            /* Open new VFIO Container */
            /* Caveat: OpenOptions(...).open(...).as_raw_fd() closes the file again instantly, staling the file descriptor! */
            container_file = Some(OpenOptions::new()
                .read(true)
                .write(true)
                .open("/dev/vfio/vfio")?);

            /* find vfio group for device */
            let link = fs::read_link(format!("/sys/bus/pci/devices/{}/iommu_group", pci_addr))?;
            let group = link.file_name().unwrap().to_str().unwrap().parse::<i32>().unwrap();
            unsafe {
                cfd = get_raw_fd(&container_file);
                /* check IOMMU API version */
                if libc::ioctl(cfd, VFIO_GET_API_VERSION) != VFIO_API_VERSION {
                    info!("Unknown VFIO API Version. Application will probably die soon(ish).");
                }

                /* check if device supports Type1 IOMMU */
                if libc::ioctl(cfd, VFIO_CHECK_EXTENSION, VFIO_TYPE1_IOMMU) != 1 {
                    info!("Device doesn't support Type1 IOMMU. Application will probably crash soon(ish).");
                }

                /* open the devices' group */
                group_file = Some(OpenOptions::new()
                    .read(true)
                    .write(true)
                    .open(format!("/dev/vfio/{}",group))?);
                gfd = get_raw_fd(&group_file);

                /* Test the group is viable and available */
                if libc::ioctl(gfd, VFIO_GROUP_GET_STATUS, &group_status) == -1 {
                    eprintln!("[ERROR]Could not VFIO_GROUP_GET_STATUS. Errno: {}", *libc::__errno_location());
                }
                if (group_status.flags & VFIO_GROUP_FLAGS_VIABLE) != 1 {
                    info!("Group is not viable (ie, not all devices bound for vfio). Application will probably crash soon(ish).");
                }

                /* Add the group to the container */
                if libc::ioctl(gfd, VFIO_GROUP_SET_CONTAINER, &cfd) == -1 {
                    eprintln!("[ERROR]Could not VFIO_GROUP_SET_CONTAINER. Errno: {}", *libc::__errno_location());
                }

                /* Enable the IOMMU model we want */
                if libc::ioctl(cfd, VFIO_SET_IOMMU, VFIO_TYPE1_IOMMU) == -1 {
                    eprintln!("[ERROR]Could not VFIO_SET_IOMMU to VFIO_TYPE1_IOMMU. Errno: {}", *libc::__errno_location());
                }

                /* Get addition IOMMU info */
                //libc::iocfiletl(vfio_cfd, VFIO_IOMMU_GET_INFO, &iommu_info);

                /* Get a file descriptor for the device */
                device_file_descriptor = libc::ioctl(gfd, VFIO_GROUP_GET_DEVICE_FD, pci_addr);
                if device_file_descriptor == -1 {
                    eprintln!("[ERROR]Could not VFIO_GROUP_GET_DEVICE_FD. Errno: {}", *libc::__errno_location());
                }

                /* write to the command register (offset 4) in the PCIe config space */
                let command_register_offset = 4;
                /* bit 2 is "bus master enable", see PCIe 3.0 specification section 7.5.1.1 */
                let bus_master_enable_bit = 2;
                
                /* map config space */
                /* Get region info for config region */
                let conf_reg: vfio_region_info = vfio_region_info {
                    argsz: mem::size_of::<vfio_region_info> as u32,
                    flags: 0,
                    index: VFIO_PCI_CONFIG_REGION_INDEX,
                    cap_offset: 0,
                    size: 0,
                    offset: 0,
                };
                if libc::ioctl(device_file_descriptor, VFIO_DEVICE_GET_REGION_INFO, &conf_reg) == -1 {
                    eprintln!("[ERROR]Could not VFIO_DEVICE_GET_REGION_INFO for index VFIO_PCI_CONFIG_REGION_INDEX. Errno: {}", *libc::__errno_location());
                }

                /* set DMA bit */
                let mut devicefile = File::from_raw_fd(device_file_descriptor);

                assert_eq!(devicefile.seek(SeekFrom::Start(conf_reg.offset + command_register_offset))?, conf_reg.offset + command_register_offset);
                let mut dma = devicefile.read_u16::<NativeEndian>()?;

                dma |= 1 << bus_master_enable_bit;

                assert_eq!(devicefile.seek(SeekFrom::Start(conf_reg.offset + command_register_offset))?, conf_reg.offset + command_register_offset);
                devicefile.write_u16::<NativeEndian>(dma)?;

                /* map BAR0 space */
                let bar0_reg: vfio_region_info = vfio_region_info {
                    argsz: mem::size_of::<vfio_region_info> as u32,
                    flags: 0,
                    index: VFIO_PCI_BAR0_REGION_INDEX,
                    cap_offset: 0,
                    size: 0,
                    offset: 0,
                };
                if libc::ioctl(device_file_descriptor, VFIO_DEVICE_GET_REGION_INFO, &bar0_reg) == -1 {
                    eprintln!("[ERROR]Could not VFIO_DEVICE_GET_REGION_INFO for index VFIO_PCI_BAR0_REGION_INDEX. Errno: {}", *libc::__errno_location());
                }

                len = bar0_reg.size as usize;

                let ptr = libc::mmap(
                        ptr::null_mut(),
                        len,
                        libc::PROT_READ | libc::PROT_WRITE,
                        libc::MAP_SHARED,
                        devicefile.as_raw_fd(),
                        bar0_reg.offset as i64,
                    ) as *mut u8;
                addr = ptr;
            }
        } else {
            device_file_descriptor = -1;
            group_file = None;
            gfd = -1;
            container_file = None;
            cfd = -1;
            let (addrtemp, lentemp) = pci_map_resource(pci_addr)?;
            addr = addrtemp;
            len = lentemp;
        }

        let rx_queues = Vec::with_capacity(num_rx_queues as usize);
        let tx_queues = Vec::with_capacity(num_tx_queues as usize);
        let mut dev = IxgbeDevice {
            pci_addr: pci_addr.to_string(),
            addr: addr,
            len: len,
            num_rx_queues: num_rx_queues,
            num_tx_queues: num_tx_queues,
            rx_queues: rx_queues,
            tx_queues: tx_queues,
            iommu: iommu,
            vfio_device_file_descriptor: device_file_descriptor,
            vfio_group_file: group_file,
            gfd: gfd,
            vfio_container_file: container_file,
            cfd: cfd,
        };

        dev.reset_and_init(pci_addr)?;

        Ok(dev)
    }

    /// Returns the driver's name of this device.
    fn get_driver_name(&self) -> &str {
        DRIVER_NAME
    }

    /// Returns the pci address of this device.
    fn get_pci_addr(&self) -> &str {
        &self.pci_addr
    }

    /// Pushes up to `num_packets` received `Packet`s onto `buffer`.
    fn rx_batch(
        &mut self,
        queue_id: u32,
        buffer: &mut VecDeque<Packet>,
        num_packets: usize,
    ) -> usize {
        let mut rx_index;
        let mut last_rx_index;
        let mut received_packets = 0;

        {
            let queue = &mut self.rx_queues[queue_id as usize];

            rx_index = queue.rx_index;
            last_rx_index = queue.rx_index;

            for i in 0..num_packets {
                let desc = unsafe { queue.descriptors.add(rx_index) as *mut ixgbe_adv_rx_desc };
                let status =
                    unsafe { ptr::read_volatile(&mut (*desc).wb.upper.status_error as *mut u32) };

                if (status & IXGBE_RXDADV_STAT_DD) != 0 {
                    if (status & IXGBE_RXDADV_STAT_EOP) == 0 {
                        panic!("increase buffer size or decrease MTU")
                    }

                    let mut pool = queue.pool.borrow_mut();

                    // get a free buffer from the mempool
                    let buf = pool.alloc_buf().expect("no buffer available");

                    // replace currently used buffer with new buffer
                    let mut buf = mem::replace(&mut queue.bufs_in_use[rx_index], buf);

                    let p = unsafe {
                        Packet {
                            addr_virt: pool.get_virt_addr(buf),
                            addr_phys: pool.get_phys_addr(buf),
                            len: ptr::read_volatile(&(*desc).wb.upper.length as *const u16)
                                as usize,
                            pool: queue.pool.clone(),
                            pool_entry: buf,
                        }
                    };

                    buffer.push_back(p);

                    unsafe {
                        ptr::write_volatile(
                            &mut (*desc).read.pkt_addr as *mut u64,
                            pool.get_phys_addr(queue.bufs_in_use[rx_index]) as u64,
                        );
                        ptr::write_volatile(&mut (*desc).read.hdr_addr as *mut u64, 0);
                    }

                    last_rx_index = rx_index;
                    rx_index = wrap_ring(rx_index, queue.num_descriptors);
                    received_packets = i + 1;
                } else {
                    break;
                }
            }
        }

        if rx_index != last_rx_index {
            self.set_reg32(IXGBE_RDT(queue_id), last_rx_index as u32);
            self.rx_queues[queue_id as usize].rx_index = rx_index;
        }

        received_packets
    }

    /// Pops as many packets as possible from `packets` to put them into the device`s tx queue.
    fn tx_batch(&mut self, queue_id: u32, packets: &mut VecDeque<Packet>) -> usize {
        let mut sent = 0;

        {
            let mut queue = &mut self.tx_queues[queue_id as usize];

            let mut cur_index = queue.tx_index;
            let clean_index = clean_tx_queue(&mut queue);

            if queue.pool.is_none() {
                if let Some(packet) = packets.get(0) {
                    queue.pool = Some(packet.pool.clone());
                }
            }

            while let Some(packet) = packets.pop_front() {
                assert_eq!(
                    queue.pool.as_ref().unwrap().as_ptr(),
                    packet.pool.as_ptr(),
                    "distinct memory pools for a single tx queue are not supported yet"
                );

                let next_index = wrap_ring(cur_index, queue.num_descriptors);

                if clean_index == next_index {
                    break;
                }

                queue.tx_index = wrap_ring(queue.tx_index, queue.num_descriptors);

                unsafe {
                    ptr::write_volatile(
                        &mut (*queue.descriptors.add(cur_index)).read.buffer_addr as *mut u64,
                        packet.get_phys_addr() as u64,
                    );
                    ptr::write_volatile(
                        &mut (*queue.descriptors.add(cur_index)).read.cmd_type_len as *mut u32,
                        IXGBE_ADVTXD_DCMD_EOP
                            | IXGBE_ADVTXD_DCMD_RS
                            | IXGBE_ADVTXD_DCMD_IFCS
                            | IXGBE_ADVTXD_DCMD_DEXT
                            | IXGBE_ADVTXD_DTYP_DATA
                            | packet.len() as u32,
                    );
                    ptr::write_volatile(
                        &mut (*queue.descriptors.add(cur_index)).read.olinfo_status as *mut u32,
                        (packet.len() as u32) << IXGBE_ADVTXD_PAYLEN_SHIFT,
                    );
                }

                queue.bufs_in_use.push_back(packet.pool_entry);
                mem::forget(packet);

                cur_index = next_index;
                sent += 1;
            }
        }

        self.set_reg32(
            IXGBE_TDT(queue_id),
            self.tx_queues[queue_id as usize].tx_index as u32,
        );

        sent
    }

    /// Reads the stats of this device into `stats`.
    fn read_stats(&self, stats: &mut DeviceStats) {
        let rx_pkts = u64::from(self.get_reg32(IXGBE_GPRC));
        let tx_pkts = u64::from(self.get_reg32(IXGBE_GPTC));
        let rx_bytes =
            u64::from(self.get_reg32(IXGBE_GORCL)) + (u64::from(self.get_reg32(IXGBE_GORCH)) << 32);
        let tx_bytes =
            u64::from(self.get_reg32(IXGBE_GOTCL)) + (u64::from(self.get_reg32(IXGBE_GOTCH)) << 32);

        stats.rx_pkts += rx_pkts;
        stats.tx_pkts += tx_pkts;
        stats.rx_bytes += rx_bytes;
        stats.tx_bytes += tx_bytes;
    }

    /// Resets the stats of this device.
    fn reset_stats(&self) {
        self.get_reg32(IXGBE_GPRC);
        self.get_reg32(IXGBE_GPTC);
        self.get_reg32(IXGBE_GORCL);
        self.get_reg32(IXGBE_GORCH);
        self.get_reg32(IXGBE_GOTCL);
        self.get_reg32(IXGBE_GOTCH);
    }

    /// Returns the link speed of this device.
    fn get_link_speed(&self) -> u16 {
        let speed = self.get_reg32(IXGBE_LINKS);
        if (speed & IXGBE_LINKS_UP) == 0 {
            return 0;
        }
        match speed & IXGBE_LINKS_SPEED_82599 {
            IXGBE_LINKS_SPEED_100_82599 => 100,
            IXGBE_LINKS_SPEED_1G_82599 => 1000,
            IXGBE_LINKS_SPEED_10G_82599 => 10000,
            _ => 0,
        }
    }
}

impl IxgbeDevice {
    /// Resets and initializes this device.
    fn reset_and_init(&mut self, pci_addr: &str) -> Result<(), Box<Error>> {
        info!("resetting device {}", pci_addr);
        // section 4.6.3.1 - disable all interrupts
        self.set_reg32(IXGBE_EIMC, 0x7fff_ffff);

        // section 4.6.3.2
        self.set_reg32(IXGBE_CTRL, IXGBE_CTRL_RST_MASK);
        self.wait_clear_reg32(IXGBE_CTRL, IXGBE_CTRL_RST_MASK);
        thread::sleep(Duration::from_millis(10));

        // section 4.6.3.1 - disable interrupts again after reset
        self.set_reg32(IXGBE_EIMC, 0x7fff_ffff);

        info!("initializing device {}", pci_addr);

        // section 4.6.3 - wait for EEPROM auto read completion
        self.wait_set_reg32(IXGBE_EEC, IXGBE_EEC_ARD);

        // section 4.6.3 - wait for dma initialization done
        self.wait_set_reg32(IXGBE_RDRXCTL, IXGBE_RDRXCTL_DMAIDONE);

        // skip last step from 4.6.3 - we don't want interrupts

        // section 4.6.4 - initialize link (auto negotiation)
        self.init_link();

        // section 4.6.5 - statistical counters
        // reset-on-read registers, just read them once
        self.reset_stats();

        // section 4.6.7 - init rx
        self.init_rx()?;

        // section 4.6.8 - init tx
        self.init_tx()?;

        for i in 0..self.num_rx_queues {
            self.start_rx_queue(i)?;
        }

        for i in 0..self.num_tx_queues {
            self.start_tx_queue(i)?;
        }

        // enable promisc mode by default to make testing easier
        self.set_promisc(true);

        // wait some time for the link to come up
        self.wait_for_link();

        Ok(())
    }

    // sections 4.6.7
    /// Initializes the rx queues of this device.
    fn init_rx(&mut self) -> Result<(), Box<Error>> {
        // disable rx while re-configuring it
        self.clear_flags32(IXGBE_RXCTRL, IXGBE_RXCTRL_RXEN);

        // section 4.6.11.3.4 - allocate all queues and traffic to PB0
        self.set_reg32(IXGBE_RXPBSIZE(0), IXGBE_RXPBSIZE_128KB);
        for i in 1..8 {
            self.set_reg32(IXGBE_RXPBSIZE(i), 0);
        }

        // enable CRC offloading
        self.set_flags32(IXGBE_HLREG0, IXGBE_HLREG0_RXCRCSTRP);
        self.set_flags32(IXGBE_RDRXCTL, IXGBE_RDRXCTL_CRCSTRIP);

        // accept broadcast packets
        self.set_flags32(IXGBE_FCTRL, IXGBE_FCTRL_BAM);

        // configure queues, same for all queues
        for i in 0..self.num_rx_queues {
            debug!("initializing rx queue {}", i);
            // enable advanced rx descriptors
            self.set_reg32(
                IXGBE_SRRCTL(u32::from(i)),
                (self.get_reg32(IXGBE_SRRCTL(u32::from(i))) & !IXGBE_SRRCTL_DESCTYPE_MASK)
                    | IXGBE_SRRCTL_DESCTYPE_ADV_ONEBUF,
            );
            // let nic drop packets if no rx descriptor is available instead of buffering them
            self.set_flags32(IXGBE_SRRCTL(u32::from(i)), IXGBE_SRRCTL_DROP_EN);

            // section 7.1.9 - setup descriptor ring
            let ring_size_bytes =
                (NUM_RX_QUEUE_ENTRIES) as usize * mem::size_of::<ixgbe_adv_rx_desc>();

            let dma: Dma<ixgbe_adv_rx_desc> = Dma::allocate(ring_size_bytes, true, self)?;

            // initialize to 0xff to prevent rogue memory accesses on premature dma activation
            unsafe {
                memset(dma.virt as *mut u8, ring_size_bytes, 0xff);
            }

            self.set_reg32(
                IXGBE_RDBAL(u32::from(i)),
                (dma.phys as u64 & 0xffff_ffff) as u32,
            );
            self.set_reg32(IXGBE_RDBAH(u32::from(i)), (dma.phys as u64 >> 32) as u32);
            self.set_reg32(IXGBE_RDLEN(u32::from(i)), ring_size_bytes as u32);

            debug!("rx ring {} phys addr: {:#x}", i, dma.phys);
            debug!("rx ring {} virt addr: {:p}", i, dma.virt);

            // set ring to empty at start
            self.set_reg32(IXGBE_RDH(u32::from(i)), 0);
            self.set_reg32(IXGBE_RDT(u32::from(i)), 0);

            let mempool_size = if NUM_RX_QUEUE_ENTRIES + NUM_TX_QUEUE_ENTRIES < 4096 {
                4096
            } else {
                NUM_RX_QUEUE_ENTRIES + NUM_TX_QUEUE_ENTRIES
            };

            let mempool = Mempool::allocate(mempool_size as usize, 2048, self).unwrap();

            let rx_queue = IxgbeRxQueue {
                descriptors: dma.virt,
                pool: mempool,
                num_descriptors: NUM_RX_QUEUE_ENTRIES,
                rx_index: 0,
                bufs_in_use: Vec::with_capacity(NUM_RX_QUEUE_ENTRIES),
            };

            self.rx_queues.push(rx_queue);
        }

        // last sentence of section 4.6.7 - set some magic bits
        self.set_flags32(IXGBE_CTRL_EXT, IXGBE_CTRL_EXT_NS_DIS);

        // probably a broken feature, this flag is initialized with 1 but has to be set to 0
        for i in 0..self.num_rx_queues {
            self.clear_flags32(IXGBE_DCA_RXCTRL(u32::from(i)), 1 << 12);
        }

        // start rx
        self.set_flags32(IXGBE_RXCTRL, IXGBE_RXCTRL_RXEN);

        Ok(())
    }

    // section 4.6.8
    /// Initializes the tx queues of this device.
    fn init_tx(&mut self) -> Result<(), Box<Error>> {
        // crc offload and small packet padding
        self.set_flags32(IXGBE_HLREG0, IXGBE_HLREG0_TXCRCEN | IXGBE_HLREG0_TXPADEN);

        // section 4.6.11.3.4 - set default buffer size allocations
        self.set_reg32(IXGBE_TXPBSIZE(0), IXGBE_TXPBSIZE_40KB);
        for i in 1..8 {
            self.set_reg32(IXGBE_TXPBSIZE(i), 0);
        }

        // required when not using DCB/VTd
        self.set_reg32(IXGBE_DTXMXSZRQ, 0xffff);
        self.clear_flags32(IXGBE_RTTDCS, IXGBE_RTTDCS_ARBDIS);

        // configure queues
        for i in 0..self.num_tx_queues {
            debug!("initializing tx queue {}", i);
            // section 7.1.9 - setup descriptor ring
            let ring_size_bytes =
                NUM_TX_QUEUE_ENTRIES as usize * mem::size_of::<ixgbe_adv_tx_desc>();

            let dma: Dma<ixgbe_adv_tx_desc> = Dma::allocate(ring_size_bytes, true, self)?;
            unsafe {
                memset(dma.virt as *mut u8, ring_size_bytes, 0xff);
            }

            self.set_reg32(
                IXGBE_TDBAL(u32::from(i)),
                (dma.phys as u64 & 0xffff_ffff) as u32,
            );
            self.set_reg32(IXGBE_TDBAH(u32::from(i)), (dma.phys as u64 >> 32) as u32);
            self.set_reg32(IXGBE_TDLEN(u32::from(i)), ring_size_bytes as u32);

            debug!("tx ring {} phys addr: {:#x}", i, dma.phys);
            debug!("tx ring {} virt addr: {:p}", i, dma.virt);

            // descriptor writeback magic values, important to get good performance and low PCIe overhead
            // see 7.2.3.4.1 and 7.2.3.5 for an explanation of these values and how to find good ones
            // we just use the defaults from DPDK here, but this is a potentially interesting point for optimizations
            let mut txdctl = self.get_reg32(IXGBE_TXDCTL(u32::from(i)));
            // there are no defines for this in constants.rs for some reason
            // pthresh: 6:0, hthresh: 14:8, wthresh: 22:16
            txdctl &= !(0x3F | (0x3F << 8) | (0x3F << 16));
            txdctl |= 36 | (8 << 8) | (4 << 16);

            self.set_reg32(IXGBE_TXDCTL(u32::from(i)), txdctl);

            let tx_queue = IxgbeTxQueue {
                descriptors: dma.virt,
                bufs_in_use: VecDeque::with_capacity(NUM_TX_QUEUE_ENTRIES),
                pool: None,
                num_descriptors: NUM_TX_QUEUE_ENTRIES,
                clean_index: 0,
                tx_index: 0,
            };

            self.tx_queues.push(tx_queue);
        }

        // final step: enable DMA
        self.set_reg32(IXGBE_DMATXCTL, IXGBE_DMATXCTL_TE);

        Ok(())
    }

    /// Sets the rx queues` descriptors and enablesIxgbeDevice the queues.
    fn start_rx_queue(&mut self, queue_id: u16) -> Result<(), Box<Error>> {
        debug!("starting rx queue {}", queue_id);

        {
            let queue = &mut self.rx_queues[queue_id as usize];

            if queue.num_descriptors & (queue.num_descriptors - 1) != 0 {
                return Err("number of queue entries must be a power of 2".into());
            }

            for i in 0..queue.num_descriptors {
                let pool = &queue.pool;

                let mut buf = match pool.borrow_mut().alloc_buf() {
                    Some(x) => x,
                    None => return Err("failed to allocate rx descriptor".into()),
                };

                unsafe {
                    ptr::write_volatile(
                        &mut (*queue.descriptors.add(i)).read.pkt_addr as *mut u64,
                        pool.borrow_mut().get_phys_addr(buf) as u64,
                    );

                    ptr::write_volatile(
                        &mut (*queue.descriptors.add(i)).read.hdr_addr as *mut u64,
                        0,
                    );
                }

                // we need to remember which descriptor entry belongs to which mempool entry
                queue.bufs_in_use.push(buf);
            }
        }

        let queue = &self.rx_queues[queue_id as usize];

        // enable queue and wait if necessary
        self.set_flags32(IXGBE_RXDCTL(u32::from(queue_id)), IXGBE_RXDCTL_ENABLE);
        self.wait_set_reg32(IXGBE_RXDCTL(u32::from(queue_id)), IXGBE_RXDCTL_ENABLE);

        // rx queue starts out full
        self.set_reg32(IXGBE_RDH(u32::from(queue_id)), 0);

        // was set to 0 before in the init function
        self.set_reg32(
            IXGBE_RDT(u32::from(queue_id)),
            (queue.num_descriptors - 1) as u32,
        );

        Ok(())
    }

    /// Enables the tx queues.
    fn start_tx_queue(&mut self, queue_id: u16) -> Result<(), Box<Error>> {
        debug!("starting tx queue {}", queue_id);

        {
            let queue = &mut self.tx_queues[queue_id as usize];

            if queue.num_descriptors & (queue.num_descriptors - 1) != 0 {
                return Err("number of queue entries must be a power of 2".into());
            }
        }

        // tx queue starts out empty
        self.set_reg32(IXGBE_TDH(u32::from(queue_id)), 0);
        self.set_reg32(IXGBE_TDT(u32::from(queue_id)), 0);

        // enable queue and wait if necessary
        self.set_flags32(IXGBE_TXDCTL(u32::from(queue_id)), IXGBE_TXDCTL_ENABLE);
        self.wait_set_reg32(IXGBE_TXDCTL(u32::from(queue_id)), IXGBE_TXDCTL_ENABLE);

        Ok(())
    }

    // see section 4.6.4
    /// Initializes the link of this device.
    fn init_link(&self) {
        // link auto-configuration register should already be set correctly, we're resetting it anyway
        self.set_reg32(
            IXGBE_AUTOC,
            (self.get_reg32(IXGBE_AUTOC) & !IXGBE_AUTOC_LMS_MASK) | IXGBE_AUTOC_LMS_10G_SERIAL,
        );
        self.set_reg32(
            IXGBE_AUTOC,
            (self.get_reg32(IXGBE_AUTOC) & !IXGBE_AUTOC_10G_PMA_PMD_MASK) | IXGBE_AUTOC_10G_XAUI,
        );
        // negotiate link
        self.set_flags32(IXGBE_AUTOC, IXGBE_AUTOC_AN_RESTART);
        // datasheet wants us to wait for the link here, but we can continue and wait afterwards
    }

    /// Waits for the link to come up.
    fn wait_for_link(&self) {
        info!("waiting for link");
        let time = Instant::now();
        let mut speed = self.get_link_speed();
        while speed == 0 && time.elapsed().as_secs() < 10 {
            thread::sleep(Duration::from_millis(100));
            speed = self.get_link_speed();
        }
        info!("link speed is {} Mbit/s", self.get_link_speed());
    }

    /// Enables or disables promisc mode of this device.
    fn set_promisc(&self, enabled: bool) {
        if enabled {
            info!("enabling promisc mode");
            self.set_flags32(IXGBE_FCTRL, IXGBE_FCTRL_MPE | IXGBE_FCTRL_UPE);
        } else {
            info!("disabling promisc mode");
            self.clear_flags32(IXGBE_FCTRL, IXGBE_FCTRL_MPE | IXGBE_FCTRL_UPE);
        }
    }

    /// Returns the register at `self.addr` + `reg`.
    ///
    /// # Panics
    ///
    /// Panics if `self.addr` + `reg` does not belong to the mapped memory of the pci device.
    fn get_reg32(&self, reg: u32) -> u32 {
        assert!(
            reg as usize <= self.len - 4 as usize,
            "memory access out of bounds"
        );

        unsafe { ptr::read_volatile((self.addr as usize + reg as usize) as *mut u32) }
    }

    /// Sets the register at `self.addr` + `reg` to `value`.
    ///
    /// # Panics
    ///
    /// Panics if `self.addr` + `reg` does not belong to the mapped memory of the pci device.
    fn set_reg32(&self, reg: u32, value: u32) {
        assert!(
            reg as usize <= self.len - 4 as usize,
            "memory access out of bounds"
        );

        unsafe {
            ptr::write_volatile((self.addr as usize + reg as usize) as *mut u32, value);
        }
    }

    /// Sets the `flags` at `self.addr` + `reg`.
    fn set_flags32(&self, reg: u32, flags: u32) {
        self.set_reg32(reg, self.get_reg32(reg) | flags);
    }

    /// Clears the `flags` at `self.addr` + `reg`.
    fn clear_flags32(&self, reg: u32, flags: u32) {
        self.set_reg32(reg, self.get_reg32(reg) & !flags);
    }

    /// Waits for `self.addr` + `reg` to clear `value`.
    fn wait_clear_reg32(&self, reg: u32, value: u32) {
        loop {
            let current = self.get_reg32(reg);
            if (current & value) == 0 {
                break;
            }
            thread::sleep(Duration::from_millis(100));
        }
    }

    /// Waits for `self.addr` + `reg` to set `value`.
    fn wait_set_reg32(&self, reg: u32, value: u32) {
        loop {
            let current = self.get_reg32(reg);
            if (current & value) == value {
                break;
            }
            thread::sleep(Duration::from_millis(100));
        }
    }

    pub fn is_vfio_device(&self) -> bool {
        return self.iommu;
    }
}

/// Removes multiples of `TX_CLEAN_BATCH` packets from `queue`.
fn clean_tx_queue(queue: &mut IxgbeTxQueue) -> usize {
    let mut clean_index = queue.clean_index;
    let cur_index = queue.tx_index;

    loop {
        let mut cleanable = cur_index as i32 - clean_index as i32;

        if cleanable < 0 {
            cleanable += queue.num_descriptors as i32;
        }

        if cleanable < TX_CLEAN_BATCH as i32 {
            break;
        }

        let mut cleanup_to = clean_index + TX_CLEAN_BATCH - 1;

        if cleanup_to >= queue.num_descriptors {
            cleanup_to -= queue.num_descriptors;
        }

        let status = unsafe {
            ptr::read_volatile(&(*queue.descriptors.add(cleanup_to)).wb.status as *const u32)
        };

        if (status & IXGBE_ADVTXD_STAT_DD) != 0 {
            if let Some(ref p) = queue.pool {
                if TX_CLEAN_BATCH as usize >= queue.bufs_in_use.len() {
                    p.borrow_mut().free_stack.append(
                        &mut queue
                            .bufs_in_use
                            .drain(..)
                            .collect::<Vec<usize>>()
                    )
                } else {
                    p.borrow_mut().free_stack.append(
                        &mut queue
                            .bufs_in_use
                            .drain(..TX_CLEAN_BATCH)
                            .collect::<Vec<usize>>(),
                    )
                }
            }

            clean_index = wrap_ring(cleanup_to, queue.num_descriptors);
        } else {
            break;
        }
    }

    queue.clean_index = clean_index;

    clean_index
}

fn get_raw_fd(f: &Option<File>) -> RawFd {
    match f{
        &Some(ref x) => return x.as_raw_fd(),
        &None => return -1,
    }
}