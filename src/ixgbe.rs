use std::collections::VecDeque;
use std::error::Error;
use std::mem;
use std::os::unix::io::RawFd;
use std::ptr;
use std::rc::Rc;
use std::thread;
use std::time::{Duration, Instant};

use crate::constants::*;
use crate::memory::*;

use crate::pci::pci_map_resource;
use crate::DeviceStats;
use crate::IxyDevice;
use crate::MAX_QUEUES;
use libc;

const DRIVER_NAME: &str = "ixy-ixgbe";

const NUM_RX_QUEUE_ENTRIES: usize = 512;
const NUM_TX_QUEUE_ENTRIES: usize = 512;
const TX_CLEAN_BATCH: usize = 32;

fn wrap_ring(index: usize, ring_size: usize) -> usize {
    (index + 1) & (ring_size - 1)
}

pub struct IxgbeDevice {
    pub(crate) pci_addr: String,
    pub(crate) addr: *mut u8,
    pub(crate) len: usize,
    pub(crate) num_rx_queues: u16,
    pub(crate) num_tx_queues: u16,
    pub(crate) rx_queues: Vec<IxgbeRxQueue>,
    pub(crate) tx_queues: Vec<IxgbeTxQueue>,
    pub(crate) iommu: bool,
    pub(crate) vfio_container: RawFd,
}

pub struct IxgbeRxQueue {
    descriptors: *mut ixgbe_adv_rx_desc,
    num_descriptors: usize,
    pool: Rc<Mempool>,
    bufs_in_use: Vec<usize>,
    rx_index: usize,
}

pub struct IxgbeTxQueue {
    descriptors: *mut ixgbe_adv_tx_desc,
    num_descriptors: usize,
    pool: Option<Rc<Mempool>>,
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

        let (addr, len) = pci_map_resource(pci_addr)?;
        let rx_queues = Vec::with_capacity(num_rx_queues as usize);
        let tx_queues = Vec::with_capacity(num_tx_queues as usize);
        let mut dev = IxgbeDevice {
            pci_addr: pci_addr.to_string(),
            addr,
            len,
            num_rx_queues,
            num_tx_queues,
            rx_queues,
            tx_queues,
            iommu: false,
            vfio_container: 0,
        };

        dev.reset_and_init(pci_addr)?;

        Ok(dev)
    }

    /// Returns the driver's name of this device.
    fn get_driver_name(&self) -> &str {
        DRIVER_NAME
    }

    /// Returns the driver's iommu capability.
    fn is_card_iommu_capable(&self) -> bool {
        self.iommu
    }

    /// Returns the VFIO container file descriptor.
    /// When implementing non-VFIO / IOMMU devices, just return 0.
    fn get_vfio_container(&self) -> Option<RawFd> {
        if self.iommu {
            Some(self.vfio_container)
        } else {
            None
        }
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

                    let pool = &queue.pool;

                    // get a free buffer from the mempool
                    let buf = pool.alloc_buf().expect("no buffer available");

                    // replace currently used buffer with new buffer
                    let buf = mem::replace(&mut queue.bufs_in_use[rx_index], buf);

                    let p = unsafe {
                        Packet {
                            addr_virt: pool.get_virt_addr(buf),
                            addr_phys: pool.get_phys_addr(buf),
                            len: ptr::read_volatile(&(*desc).wb.upper.length as *const u16)
                                as usize,
                            pool: pool.clone(),
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
                assert!(
                    Rc::ptr_eq(queue.pool.as_ref().unwrap(), &packet.pool),
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
    pub(crate) fn reset_and_init(&mut self, pci_addr: &str) -> Result<(), Box<Error>> {
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
