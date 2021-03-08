use std::cell::RefCell;
use std::cmp::min;
use std::collections::VecDeque;
use std::error::Error;
use std::fs::File;
use std::io::Read;
use std::mem;
use std::os::unix::io::RawFd;
use std::path::Path;
use std::ptr;
use std::rc::Rc;
use std::thread;
use std::time::Duration;

use crate::constants::*;
use crate::memory::*;
use crate::vfio::*;

use crate::pci::pci_map_resource;
use crate::vfio::VFIO_PCI_BAR0_REGION_INDEX;
use crate::DeviceStats;
use crate::IxyDevice;

const DRIVER_NAME: &str = "ixy-ixgbevf";

const MAX_QUEUES: u16 = 8;

const PKT_BUF_ENTRY_SIZE: usize = 2048;
const MIN_MEMPOOL_SIZE: usize = 4096;

const NUM_RX_QUEUE_ENTRIES: usize = 512;
const NUM_TX_QUEUE_ENTRIES: usize = 512;
const TX_CLEAN_BATCH: usize = 32;

fn wrap_ring(index: usize, ring_size: usize) -> usize {
    (index + 1) & (ring_size - 1)
}

pub struct Mailbox {
    api_version: ixgbe_pfvf_api_rev,

    timeout: u32,
    usec_delay: u32,
    v2p_mailbox: u32,
    size: u16,

    // stats
    msgs_tx: u32,
    msgs_rx: u32,
    reqs: u32,
    acks: u32,
    rsts: u32,
}

impl Mailbox {
    fn init() -> Self {
        Mailbox {
            api_version: ixgbe_pfvf_api_rev::ixgbe_mbox_api_10,

            timeout: IXGBE_VF_MBX_INIT_TIMEOUT,
            usec_delay: IXGBE_VF_MBX_INIT_DELAY,
            v2p_mailbox: 0,
            size: IXGBE_VFMAILBOX_SIZE,

            // stats
            msgs_tx: 0,
            msgs_rx: 0,
            reqs: 0,
            acks: 0,
            rsts: 0,
        }
    }
}

pub struct IxgbeVFDevice {
    pci_addr: String,
    addr: *mut u8,
    len: usize,
    num_rx_queues: u16,
    num_tx_queues: u16,
    rx_queues: Vec<IxgbeRxQueue>,
    tx_queues: Vec<IxgbeTxQueue>,
    mbx: RefCell<Mailbox>,
    mac: RefCell<[u8; 6]>,
    stats: RefCell<DeviceStats>,
    vfio: bool,
    vfio_fd: RawFd,
}

struct IxgbeRxQueue {
    descriptors: *mut ixgbe_adv_rx_desc,
    num_descriptors: usize,
    pool: Rc<Mempool>,
    bufs_in_use: Vec<usize>,
    rx_index: usize,
}

struct IxgbeTxQueue {
    descriptors: *mut ixgbe_adv_tx_desc,
    num_descriptors: usize,
    pool: Option<Rc<Mempool>>,
    bufs_in_use: VecDeque<usize>,
    clean_index: usize,
    tx_index: usize,
}

impl IxyDevice for IxgbeVFDevice {
    /// Returns the driver's name of this device.
    fn get_driver_name(&self) -> &str {
        DRIVER_NAME
    }

    /// Returns the card's iommu capability.
    fn is_card_iommu_capable(&self) -> bool {
        self.vfio
    }

    /// Returns VFIO container file descriptor or [`None`] if IOMMU is not available.
    fn get_vfio_container(&self) -> Option<RawFd> {
        if self.vfio {
            Some(self.vfio_fd)
        } else {
            None
        }
    }

    /// Returns the pci address of this device.
    fn get_pci_addr(&self) -> &str {
        &self.pci_addr
    }

    /// Returns the mac address of this device.
    fn get_mac_addr(&self) -> [u8; 6] {
        *self.mac.borrow()
    }

    /// Sets the mac address of this device.
    fn set_mac_addr(&self, mac: [u8; 6]) {
        let mut msg = [IXGBE_VF_SET_MAC_ADDR, 0, 0];

        msg[1] = u32::from(mac[0])
            + (u32::from(mac[1]) << 8)
            + (u32::from(mac[2]) << 16)
            + (u32::from(mac[3]) << 24);
        msg[2] = u32::from(mac[4]) + (u32::from(mac[5]) << 8);

        self.wait_write_read_msg_mbx(&mut msg).unwrap();

        msg[0] &= !IXGBE_VT_MSGTYPE_CTS;

        if msg[0] == (IXGBE_VF_SET_MAC_ADDR | IXGBE_VT_MSGTYPE_NACK) {
            warn!("mac address rejected by device");
        }
    }

    /// Pushes up to `num_packets` received `Packet`s onto `buffer`.
    fn rx_batch(
        &mut self,
        queue_id: u16,
        buffer: &mut VecDeque<Packet>,
        num_packets: usize,
    ) -> usize {
        let mut rx_index;
        let mut last_rx_index;
        let mut received_packets = 0;

        {
            let queue = self
                .rx_queues
                .get_mut(queue_id as usize)
                .expect("invalid rx queue id");

            rx_index = queue.rx_index;
            last_rx_index = queue.rx_index;

            for i in 0..num_packets {
                let desc = unsafe { queue.descriptors.add(rx_index) as *mut ixgbe_adv_rx_desc };
                let status =
                    unsafe { ptr::read_volatile(&mut (*desc).wb.upper.status_error as *mut u32) };

                if (status & IXGBE_RXDADV_STAT_DD) == 0 {
                    break;
                }

                if (status & IXGBE_RXDADV_STAT_EOP) == 0 {
                    panic!("increase buffer size or decrease MTU")
                }

                let pool = &queue.pool;

                // get a free buffer from the mempool
                if let Some(buf) = pool.alloc_buf() {
                    // replace currently used buffer with new buffer
                    let buf = mem::replace(&mut queue.bufs_in_use[rx_index], buf);

                    let p = Packet {
                        addr_virt: pool.get_virt_addr(buf),
                        addr_phys: pool.get_phys_addr(buf),
                        len: unsafe {
                            ptr::read_volatile(&(*desc).wb.upper.length as *const u16) as usize
                        },
                        pool: pool.clone(),
                        pool_entry: buf,
                    };

                    #[cfg(all(
                        any(target_arch = "x86", target_arch = "x86_64"),
                        target_feature = "sse"
                    ))]
                    p.prefetch(Prefetch::Time1);

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
                    // break if there was no free buffer
                    break;
                }
            }
        }

        if rx_index != last_rx_index {
            self.set_reg32(IXGBE_VFRDT(u32::from(queue_id)), last_rx_index as u32);
            self.rx_queues[queue_id as usize].rx_index = rx_index;
        }

        received_packets
    }

    /// Pops as many packets as possible from `packets` to put them into the device`s tx queue.
    fn tx_batch(&mut self, queue_id: u16, buffer: &mut VecDeque<Packet>) -> usize {
        let mut sent = 0;

        {
            let mut queue = self
                .tx_queues
                .get_mut(queue_id as usize)
                .expect("invalid tx queue id");

            let mut cur_index = queue.tx_index;
            let clean_index = clean_tx_queue(&mut queue);

            if queue.pool.is_none() {
                if let Some(packet) = buffer.get(0) {
                    queue.pool = Some(packet.pool.clone());
                }
            }

            while let Some(packet) = buffer.pop_front() {
                assert!(
                    Rc::ptr_eq(queue.pool.as_ref().unwrap(), &packet.pool),
                    "distinct memory pools for a single tx queue are not supported yet"
                );

                let next_index = wrap_ring(cur_index, queue.num_descriptors);

                if clean_index == next_index {
                    // tx queue of device is full, push packet back onto the
                    // queue of to-be-sent packets
                    buffer.push_front(packet);
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
            IXGBE_VFTDT(u32::from(queue_id)),
            self.tx_queues[queue_id as usize].tx_index as u32,
        );

        sent
    }

    /// Reads the stats of this device into `stats`.
    fn read_stats(&self, stats: &mut DeviceStats) {
        let mut dev_stats = self.stats.borrow_mut();

        let rx_pkts = u64::from(self.get_reg32(IXGBE_VFGPRC));
        let tx_pkts = u64::from(self.get_reg32(IXGBE_VFGPTC));
        let rx_bytes = u64::from(self.get_reg32(IXGBE_VFGORC_LSB))
            + (u64::from(self.get_reg32(IXGBE_VFGORC_MSB)) << 32);
        let tx_bytes = u64::from(self.get_reg32(IXGBE_VFGOTC_LSB))
            + (u64::from(self.get_reg32(IXGBE_VFGOTC_MSB)) << 32);

        // stat registers wrap around, pkts have a 32 bit and bytes a 36 bit counter
        stats.rx_pkts += rx_pkts.wrapping_sub(dev_stats.rx_pkts) & ((1 << 32) - 1);
        stats.tx_pkts += tx_pkts.wrapping_sub(dev_stats.tx_pkts) & ((1 << 32) - 1);
        stats.rx_bytes += rx_bytes.wrapping_sub(dev_stats.rx_bytes) & ((1 << 36) - 1);
        stats.tx_bytes += tx_bytes.wrapping_sub(dev_stats.tx_bytes) & ((1 << 36) - 1);

        dev_stats.rx_pkts = rx_pkts;
        dev_stats.tx_pkts = tx_pkts;
        dev_stats.rx_bytes = rx_bytes;
        dev_stats.tx_bytes = tx_bytes;
    }

    /// Resets the stats of this device.
    fn reset_stats(&mut self) {
        let mut dev_stats = self.stats.borrow_mut();

        dev_stats.rx_pkts = u64::from(self.get_reg32(IXGBE_VFGPRC));
        dev_stats.tx_pkts = u64::from(self.get_reg32(IXGBE_VFGPTC));
        dev_stats.rx_bytes = u64::from(self.get_reg32(IXGBE_VFGORC_LSB))
            + (u64::from(self.get_reg32(IXGBE_VFGORC_MSB)) << 32);
        dev_stats.tx_bytes = u64::from(self.get_reg32(IXGBE_VFGOTC_LSB))
            + (u64::from(self.get_reg32(IXGBE_VFGOTC_MSB)) << 32);
    }

    /// Returns the link speed of this device.
    fn get_link_speed(&self) -> u16 {
        let speed = self.get_reg32(IXGBE_VFLINKS);
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

impl IxgbeVFDevice {
    /// Returns an initialized `IxgbeVFDevice` on success.
    ///
    /// # Panics
    /// Panics if `num_rx_queues` or `num_tx_queues` exceeds `MAX_QUEUES`.
    pub fn init(
        pci_addr: &str,
        num_rx_queues: u16,
        num_tx_queues: u16,
    ) -> Result<IxgbeVFDevice, Box<dyn Error>> {
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

        // Check if the NIC is IOMMU enabled...
        let vfio = Path::new(&format!("/sys/bus/pci/devices/{}/iommu_group", pci_addr)).exists();

        let (addr, len) = if vfio {
            let device_fd = vfio_init(pci_addr)?;
            vfio_map_region(device_fd, VFIO_PCI_BAR0_REGION_INDEX)?
        } else {
            if unsafe { libc::getuid() } != 0 {
                warn!("not running as root, this will probably fail");
            }

            pci_map_resource(pci_addr)?
        };

        // initialize RX and TX queue
        let rx_queues = Vec::with_capacity(num_rx_queues as usize);
        let tx_queues = Vec::with_capacity(num_tx_queues as usize);

        let mbx = RefCell::new(Mailbox::init());
        let mac = RefCell::new([0; 6]);
        let stats = RefCell::new(DeviceStats::default());

        // create the IxyDevice
        let mut dev = IxgbeVFDevice {
            pci_addr: pci_addr.to_string(),
            addr,
            len,
            num_rx_queues,
            num_tx_queues,
            rx_queues,
            tx_queues,
            mbx,
            mac,
            stats,
            vfio,
            vfio_fd: unsafe { VFIO_CONTAINER_FILE_DESCRIPTOR },
        };

        dev.reset_and_init(pci_addr)?;

        Ok(dev)
    }

    /// Resets and initializes this device.
    fn reset_and_init(&mut self, pci_addr: &str) -> Result<(), Box<dyn Error>> {
        info!("resetting device {}", pci_addr);

        // disable all interrupts
        self.disable_interrupts();

        // reset VF
        self.set_reg32(IXGBE_VFCTRL, IXGBE_CTRL_RST);
        self.get_reg32(IXGBE_STATUS);
        thread::sleep(Duration::from_millis(50));

        // cannot reset while the RSTI / RSTD bits are asserted
        self.wait_check_for_rst()?;

        // DPDK does this, probably something we could leave out
        self.reset_vf_registers();

        self.wait_write_msg_to_mbx(&[IXGBE_VF_RESET])?;
        thread::sleep(Duration::from_millis(10));

        self.init_mac_addr()?;

        let mac = self.get_mac_addr();
        info!("initializing device {}", pci_addr);
        info!(
            "mac address: {:02x}:{:02x}:{:02x}:{:02x}:{:02x}:{:02x}",
            mac[0], mac[1], mac[2], mac[3], mac[4], mac[5]
        );

        self.negotiate_api()?;

        self.init_tx()?;

        self.init_rx()?;

        for i in 0..self.num_tx_queues {
            self.start_tx_queue(i)?;
        }

        for i in 0..self.num_rx_queues {
            self.start_rx_queue(i)?;
        }

        // setup done, what is our link speed?
        info!("link speed is {} Mbit/s", self.get_link_speed());

        Ok(())
    }

    /// Resets the VF registers.
    fn reset_vf_registers(&mut self) {
        // VRSRRCTL default values (BSIZEPACKET = 2048, BSIZEHEADER = 256)
        let mut vfsrrctl = 0x100 << IXGBE_SRRCTL_BSIZEHDRSIZE_SHIFT;
        vfsrrctl |= 0x800 >> IXGBE_SRRCTL_BSIZEPKT_SHIFT;

        // DCA_RXCTRL default value
        let vfdca_rxctrl = IXGBE_DCA_RXCTRL_DESC_RRO_EN
            | IXGBE_DCA_RXCTRL_DATA_WRO_EN
            | IXGBE_DCA_RXCTRL_HEAD_WRO_EN;

        // DCA_TXCTRL default value
        let vfdca_txctrl = IXGBE_DCA_TXCTRL_DESC_RRO_EN
            | IXGBE_DCA_TXCTRL_DESC_WRO_EN
            | IXGBE_DCA_TXCTRL_DATA_RRO_EN;

        self.set_reg32(IXGBE_VFPSRTYPE, 0);

        for i in 0..(MAX_QUEUES as u32) {
            self.set_reg32(IXGBE_VFRDH(i), 0);
            self.set_reg32(IXGBE_VFRDT(i), 0);
            self.set_reg32(IXGBE_VFRXDCTL(i), 0);
            self.set_reg32(IXGBE_VFSRRCTL(i), vfsrrctl);
            self.set_reg32(IXGBE_VFTDH(i), 0);
            self.set_reg32(IXGBE_VFTDT(i), 0);
            self.set_reg32(IXGBE_VFTXDCTL(i), 0);
            self.set_reg32(IXGBE_VFTDWBAH(i), 0);
            self.set_reg32(IXGBE_VFTDWBAL(i), 0);
            self.set_reg32(IXGBE_VFDCA_RXCTRL(i), vfdca_rxctrl);
            self.set_reg32(IXGBE_VFDCA_TXCTRL(i), vfdca_txctrl);
        }

        self.get_reg32(IXGBE_STATUS);
    }

    /// Negotiates the mailbox API version.
    fn negotiate_api(&mut self) -> Result<(), Box<dyn Error>> {
        let api_versions = [
            ixgbe_pfvf_api_rev::ixgbe_mbox_api_13,
            ixgbe_pfvf_api_rev::ixgbe_mbox_api_12,
            ixgbe_pfvf_api_rev::ixgbe_mbox_api_11,
            ixgbe_pfvf_api_rev::ixgbe_mbox_api_10,
        ];

        for api_version in &api_versions {
            let mut msg = [IXGBE_VF_API_NEGOTIATE, *api_version as u32, 0];

            self.wait_write_read_msg_mbx(&mut msg)?;

            msg[0] &= !IXGBE_VT_MSGTYPE_CTS;

            if msg[0] == (IXGBE_VF_API_NEGOTIATE | IXGBE_VT_MSGTYPE_ACK) {
                self.mbx.borrow_mut().api_version = *api_version;
                break;
            }
        }

        Ok(())
    }

    /// Initializes the mac address of this device appropriately, i.e. by
    /// using the PF set mac address or generating a new one.
    fn init_mac_addr(&mut self) -> Result<(), Box<dyn Error>> {
        // permanent address
        let mut msg_buf = [3; IXGBE_VF_PERMADDR_MSG_LEN as usize];
        self.wait_read_msg_from_mbx(&mut msg_buf)?;

        if msg_buf[0] != (IXGBE_VF_RESET | IXGBE_VT_MSGTYPE_ACK)
            && msg_buf[0] != (IXGBE_VF_RESET | IXGBE_VT_MSGTYPE_NACK)
        {
            return Err("invalid mac address".into());
        }

        if msg_buf[0] == (IXGBE_VF_RESET | IXGBE_VT_MSGTYPE_ACK) {
            let mut mac = self.mac.borrow_mut();

            mac[0] = (msg_buf[1] >> 24) as u8;
            mac[1] = (msg_buf[1] >> 16 & 0xff) as u8;
            mac[2] = (msg_buf[1] >> 8 & 0xff) as u8;
            mac[3] = (msg_buf[1] & 0xff) as u8;
            mac[4] = (msg_buf[2] >> 8 & 0xff) as u8;
            mac[5] = (msg_buf[2] & 0xff) as u8;

            info!(
                "received mac address from PF driver: {:02x}:{:02x}:{:02x}:{:02x}:{:02x}:{:02x}",
                mac[0], mac[1], mac[2], mac[3], mac[4], mac[5]
            );
        } else {
            let mut mac = self.mac.borrow_mut();

            mac[0] = 0x02; // indicates locally assigned mac address
            mac[1] = 0x09;
            mac[2] = 0xC0;

            // generate last 3 bytes of mac address randomly
            File::open("/dev/urandom")?.read_exact(&mut mac[3..])?;

            info!(
                "generated mac address: {:02x}:{:02x}:{:02x}:{:02x}:{:02x}:{:02x}",
                mac[0], mac[1], mac[2], mac[3], mac[4], mac[5]
            );

            self.set_mac_addr(*mac);
        }

        Ok(())
    }

    // sections 4.6.7
    /// Initializes the rx queues of this device.
    fn init_rx(&mut self) -> Result<(), Box<dyn Error>> {
        // configure queues, same for all queues
        for i in 0..self.num_rx_queues {
            debug!("initializing rx queue {}", i);
            // enable advanced rx descriptors
            self.set_reg32(
                IXGBE_VFSRRCTL(u32::from(i)),
                (self.get_reg32(IXGBE_VFSRRCTL(u32::from(i))) & !IXGBE_SRRCTL_DESCTYPE_MASK)
                    | IXGBE_SRRCTL_DESCTYPE_ADV_ONEBUF,
            );
            // let nic drop packets if no rx descriptor is available instead of buffering them
            self.set_flags32(IXGBE_VFSRRCTL(u32::from(i)), IXGBE_SRRCTL_DROP_EN);

            // section 7.1.9 - setup descriptor ring
            let ring_size_bytes =
                (NUM_RX_QUEUE_ENTRIES) as usize * mem::size_of::<ixgbe_adv_rx_desc>();

            let dma: Dma<ixgbe_adv_rx_desc> = Dma::allocate(ring_size_bytes, true)?;

            // initialize to 0xff to prevent rogue memory accesses on premature dma activation
            unsafe {
                memset(dma.virt as *mut u8, ring_size_bytes, 0xff);
            }

            self.set_reg32(
                IXGBE_VFRDBAL(u32::from(i)),
                (dma.phys as u64 & 0xffff_ffff) as u32,
            );
            self.set_reg32(IXGBE_VFRDBAH(u32::from(i)), (dma.phys as u64 >> 32) as u32);
            self.set_reg32(IXGBE_VFRDLEN(u32::from(i)), ring_size_bytes as u32);

            debug!("rx ring {} phys addr: {:#x}", i, dma.phys);
            debug!("rx ring {} virt addr: {:p}", i, dma.virt);

            // set ring to empty at start
            self.set_reg32(IXGBE_VFRDH(u32::from(i)), 0);
            self.set_reg32(IXGBE_VFRDT(u32::from(i)), 0);

            let mempool_size = if NUM_RX_QUEUE_ENTRIES + NUM_TX_QUEUE_ENTRIES < MIN_MEMPOOL_SIZE {
                MIN_MEMPOOL_SIZE
            } else {
                NUM_RX_QUEUE_ENTRIES + NUM_TX_QUEUE_ENTRIES
            };

            let mempool = Mempool::allocate(mempool_size as usize, PKT_BUF_ENTRY_SIZE).unwrap();

            let rx_queue = IxgbeRxQueue {
                descriptors: dma.virt,
                pool: mempool,
                num_descriptors: NUM_RX_QUEUE_ENTRIES,
                rx_index: 0,
                bufs_in_use: Vec::with_capacity(NUM_RX_QUEUE_ENTRIES),
            };

            self.rx_queues.push(rx_queue);
        }

        // probably a broken feature, this flag is initialized with 1 but has to be set to 0
        for i in 0..self.num_rx_queues {
            self.clear_flags32(IXGBE_VFDCA_RXCTRL(u32::from(i)), 1 << 12);
        }

        Ok(())
    }

    // section 4.6.8
    /// Initializes the tx queues of this device.
    fn init_tx(&mut self) -> Result<(), Box<dyn Error>> {
        // configure queues
        for i in 0..self.num_tx_queues {
            debug!("initializing tx queue {}", i);
            // section 7.1.9 - setup descriptor ring
            let ring_size_bytes =
                NUM_TX_QUEUE_ENTRIES as usize * mem::size_of::<ixgbe_adv_tx_desc>();

            let dma: Dma<ixgbe_adv_tx_desc> = Dma::allocate(ring_size_bytes, true)?;
            unsafe {
                memset(dma.virt as *mut u8, ring_size_bytes, 0xff);
            }

            self.set_reg32(
                IXGBE_VFTDBAL(u32::from(i)),
                (dma.phys as u64 & 0xffff_ffff) as u32,
            );
            self.set_reg32(IXGBE_VFTDBAH(u32::from(i)), (dma.phys as u64 >> 32) as u32);
            self.set_reg32(IXGBE_VFTDLEN(u32::from(i)), ring_size_bytes as u32);

            debug!("tx ring {} phys addr: {:#x}", i, dma.phys);
            debug!("tx ring {} virt addr: {:p}", i, dma.virt);

            // descriptor writeback magic values, important to get good performance and low PCIe overhead
            // see 7.2.3.4.1 and 7.2.3.5 for an explanation of these values and how to find good ones
            // we just use the defaults from DPDK here, but this is a potentially interesting point for optimizations
            let mut txdctl = self.get_reg32(IXGBE_VFTXDCTL(u32::from(i)));
            // there are no defines for this in constants.rs for some reason
            // pthresh: 6:0, hthresh: 14:8, wthresh: 22:16
            txdctl &= !(0x7F | (0x7F << 8) | (0x7F << 16));
            txdctl |= 36 | (8 << 8) | (4 << 16);

            self.set_reg32(IXGBE_VFTXDCTL(u32::from(i)), txdctl);

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

        Ok(())
    }

    /// Sets the rx queues` descriptors and enables the queues.
    fn start_rx_queue(&mut self, queue_id: u16) -> Result<(), Box<dyn Error>> {
        debug!("starting rx queue {}", queue_id);

        let queue = &mut self.rx_queues[queue_id as usize];

        if queue.num_descriptors & (queue.num_descriptors - 1) != 0 {
            return Err("number of queue entries must be a power of 2".into());
        }

        for i in 0..queue.num_descriptors {
            let pool = &queue.pool;

            let buf = match pool.alloc_buf() {
                Some(x) => x,
                None => return Err("failed to allocate rx descriptor".into()),
            };

            unsafe {
                ptr::write_volatile(
                    &mut (*queue.descriptors.add(i)).read.pkt_addr as *mut u64,
                    pool.get_phys_addr(buf) as u64,
                );

                ptr::write_volatile(
                    &mut (*queue.descriptors.add(i)).read.hdr_addr as *mut u64,
                    0,
                );
            }

            // we need to remember which descriptor entry belongs to which mempool entry
            queue.bufs_in_use.push(buf);
        }

        let queue = &self.rx_queues[queue_id as usize];

        // enable queue and wait if necessary
        self.set_flags32(IXGBE_VFRXDCTL(u32::from(queue_id)), IXGBE_RXDCTL_ENABLE);
        self.wait_set_reg32(IXGBE_VFRXDCTL(u32::from(queue_id)), IXGBE_RXDCTL_ENABLE);

        // rx queue starts out full
        self.set_reg32(IXGBE_VFRDH(u32::from(queue_id)), 0);

        // was set to 0 before in the init function
        self.set_reg32(
            IXGBE_VFRDT(u32::from(queue_id)),
            (queue.num_descriptors - 1) as u32,
        );

        Ok(())
    }

    /// Enables the tx queues.
    fn start_tx_queue(&mut self, queue_id: u16) -> Result<(), Box<dyn Error>> {
        debug!("starting tx queue {}", queue_id);

        let queue = &mut self.tx_queues[queue_id as usize];

        if queue.num_descriptors & (queue.num_descriptors - 1) != 0 {
            return Err("number of queue entries must be a power of 2".into());
        }

        // tx queue starts out empty
        self.set_reg32(IXGBE_VFTDH(u32::from(queue_id)), 0);
        self.set_reg32(IXGBE_VFTDT(u32::from(queue_id)), 0);

        // enable queue and wait if necessary
        self.set_flags32(IXGBE_VFTXDCTL(u32::from(queue_id)), IXGBE_TXDCTL_ENABLE);
        self.wait_set_reg32(IXGBE_VFTXDCTL(u32::from(queue_id)), IXGBE_TXDCTL_ENABLE);

        Ok(())
    }

    /// Enables or disables promiscuous mode of this device.
    #[allow(dead_code)]
    fn set_promisc(&self, _enabled: bool) {
        unimplemented!("PF driver do not support promiscuous mode for VFs yet, see chapter 7.1 in the Intel 82599 SR-IOV driver companion guide");
    }

    /// Returns the register at `self.addr` + `reg`.
    ///
    /// # Panics
    ///
    /// Panics if `self.addr` + `reg` does not belong to the mapped memory of the pci device.
    fn get_reg32(&self, reg: u32) -> u32 {
        assert!(reg as usize <= self.len - 4, "memory access out of bounds");

        unsafe { ptr::read_volatile((self.addr as usize + reg as usize) as *mut u32) }
    }

    /// Sets the register at `self.addr` + `reg` to `value`.
    ///
    /// # Panics
    ///
    /// Panics if `self.addr` + `reg` does not belong to the mapped memory of the pci device.
    fn set_reg32(&self, reg: u32, value: u32) {
        assert!(reg as usize <= self.len - 4, "memory access out of bounds");

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
    #[allow(dead_code)]
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

    /// Returns the register at `self.addr` + `reg` + (`index` * 4).
    ///
    /// # Panics
    ///
    /// Panics if `self.addr` + `reg` + (`index` * 4) does not belong to the mapped memory of the pci device.
    fn get_reg32_array(&self, reg: u32, index: u32) -> u32 {
        let idx = reg + (index << 2);

        assert!(idx as usize <= self.len - 4, "memory access out of bounds");

        unsafe { ptr::read_volatile((self.addr as usize + idx as usize) as *mut u32) }
    }

    /// Sets the register at `self.addr` + `reg` + (`index` * 4) to `value`.
    ///
    /// # Panics
    ///
    /// Panics if `self.addr` + `reg` + (`index` * 4) does not belong to the mapped memory of the pci device.
    fn set_reg32_array(&self, reg: u32, index: u32, value: u32) {
        let idx = reg + (index << 2);

        assert!(idx as usize <= self.len - 4, "memory access out of bounds");

        unsafe {
            ptr::write_volatile((self.addr as usize + idx as usize) as *mut u32, value);
        }
    }

    /// Clears all interrupt masks for all queues.
    fn clear_interrupts(&self) {
        // Clear interrupt mask
        self.set_reg32(IXGBE_VTEIMC, IXGBE_VF_IRQ_CLEAR_MASK);

        // Clear any pending interrupts
        self.get_reg32(IXGBE_VTEICR);
    }

    /// Disables all interrupts for all queues.
    fn disable_interrupts(&self) {
        self.clear_interrupts();
    }

    /// Waits for reset from PF.
    fn wait_check_for_rst(&mut self) -> Result<(), Box<dyn Error>> {
        let mut countdown = self.mbx.borrow().timeout;

        while countdown > 0 && !self.check_for_rst() {
            countdown -= 1;
            thread::sleep(Duration::from_micros(5));
        }

        if countdown == 0 {
            Err("timeout while checking for reset".into())
        } else {
            Ok(())
        }
    }

    /// Checks if the PF has sent a message.
    fn check_for_msg(&self) -> bool {
        if !self.check_for_bit(IXGBE_VFMAILBOX_PFSTS) {
            self.mbx.borrow_mut().reqs += 1;
            true
        } else {
            false
        }
    }

    /// Checks if the PF has sent ack.
    fn check_for_ack(&self) -> bool {
        if !self.check_for_bit(IXGBE_VFMAILBOX_PFACK) {
            self.mbx.borrow_mut().acks += 1;
            true
        } else {
            false
        }
    }

    /// Checks if the PF has sent reset.
    fn check_for_rst(&self) -> bool {
        if !self.check_for_bit(IXGBE_VFMAILBOX_RSTD | IXGBE_VFMAILBOX_RSTI) {
            self.mbx.borrow_mut().rsts += 1;
            true
        } else {
            false
        }
    }

    /// Checks for the read to clear bits within the V2P mailbox.
    fn check_for_bit(&self, mask: u32) -> bool {
        let v2p_mailbox = self.read_v2p_mbx();

        self.mbx.borrow_mut().v2p_mailbox &= !mask;

        (v2p_mailbox & mask) != 0x0
    }

    /// Reads the v2p mailbox without losing the read to clear status bits.
    fn read_v2p_mbx(&self) -> u32 {
        let mut v2p_mailbox = self.get_reg32(IXGBE_VFMAILBOX);

        v2p_mailbox |= self.mbx.borrow().v2p_mailbox;
        self.mbx.borrow_mut().v2p_mailbox |= v2p_mailbox & IXGBE_VFMAILBOX_R2C_BITS;

        v2p_mailbox
    }

    /// Writes a message to the mailbox, waits for ack, reads a message from the mailbox.
    fn wait_write_read_msg_mbx(&self, msg: &mut [u32]) -> Result<(), Box<dyn Error>> {
        self.wait_write_msg_to_mbx(msg)?;
        self.wait_read_msg_from_mbx(msg)?;

        Ok(())
    }

    /// Writes a message to the mailbox, waits for ack.
    fn wait_write_msg_to_mbx(&self, msg: &[u32]) -> Result<(), Box<dyn Error>> {
        self.write_msg_to_mbx(msg)?;
        self.wait_for_ack()?;

        Ok(())
    }

    /// Waits for ack from PF.
    fn wait_for_ack(&self) -> Result<(), Box<dyn Error>> {
        let mut countdown = self.mbx.borrow().timeout;

        while countdown > 0 && self.check_for_ack() {
            countdown -= 1;
            thread::sleep(Duration::from_micros(self.mbx.borrow().usec_delay as u64));
        }

        if countdown == 0 {
            Err("timeout while polling for ack".into())
        } else {
            Ok(())
        }
    }

    /// Waits for message from PF.
    fn wait_for_msg(&self) -> Result<(), Box<dyn Error>> {
        let mut countdown = self.mbx.borrow().timeout;

        while countdown > 0 && self.check_for_msg() {
            countdown -= 1;
            thread::sleep(Duration::from_micros(self.mbx.borrow().usec_delay as u64));
        }

        if countdown == 0 {
            Err("timeout while polling for message".into())
        } else {
            Ok(())
        }
    }

    /// Writes a message to the mailbox.
    fn write_msg_to_mbx(&self, msg: &[u32]) -> Result<(), Box<dyn Error>> {
        assert!(
            msg.len() <= self.mbx.borrow().size as usize,
            "invalid mailbox message size"
        );

        // lock mailbox to prevent pf/vf race condition
        self.obtain_mbx_lock()?;

        // flush msg and acks as we are overwriting the message buffer
        self.check_for_msg();
        self.check_for_ack();

        // copy message to mailbox memory buffer
        for (idx, el) in msg.iter().enumerate() {
            self.set_reg32_array(IXGBE_VFMBMEM, idx as u32, *el);
        }

        // update stats
        self.mbx.borrow_mut().msgs_tx += 1;

        // Drop VFU and interrupt the PF to tell it a message has been sent
        self.set_reg32(IXGBE_VFMAILBOX, IXGBE_VFMAILBOX_REQ);

        Ok(())
    }

    /// Receives (and waits for) a message from the mailbox.
    fn wait_read_msg_from_mbx(&self, msg: &mut [u32]) -> Result<(), Box<dyn Error>> {
        self.wait_for_msg()?;
        self.read_msg_from_mbx(msg)?;

        Ok(())
    }

    /// Reads a message from the mailbox.
    fn read_msg_from_mbx(&self, msg: &mut [u32]) -> Result<(), Box<dyn Error>> {
        let len = min(msg.len(), self.mbx.borrow().size as usize);

        // lock mailbox to prevent pf/vf race condition
        self.obtain_mbx_lock()?;

        // copy message from mailbox memory buffer
        for (idx, el) in msg[0..len].iter_mut().enumerate() {
            *el = self.get_reg32_array(IXGBE_VFMBMEM, idx as u32);
        }

        // Acknowledge receipt and release mailbox, then we're done
        self.set_reg32(IXGBE_VFMAILBOX, IXGBE_VFMAILBOX_ACK);

        // update stats
        self.mbx.borrow_mut().msgs_rx += 1;

        Ok(())
    }

    /// Obtains the mailbox lock.
    fn obtain_mbx_lock(&self) -> Result<(), Box<dyn Error>> {
        // take ownership of the buffer
        self.set_reg32(IXGBE_VFMAILBOX, IXGBE_VFMAILBOX_VFU);

        // reserve mailbox for vf use
        if (self.read_v2p_mbx() & IXGBE_VFMAILBOX_VFU) != 0x0 {
            Ok(())
        } else {
            Err("failed to obtain mailbox lock".into())
        }
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
                    p.free_stack
                        .borrow_mut()
                        .extend(queue.bufs_in_use.drain(..))
                } else {
                    p.free_stack
                        .borrow_mut()
                        .extend(queue.bufs_in_use.drain(..TX_CLEAN_BATCH))
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
