use std;
use std::thread;
use std::time::Duration;
use std::time::Instant;
use std::ptr;
use std::error::Error;
use std::io::ErrorKind;

use constants::*;
use pci::*;
use memory::*;

use std::rc::Rc;
use std::cell::RefCell;

use std::collections::VecDeque;
use std::mem;

use IxyDriver;
use MAX_QUEUES;
use DeviceStats;
use libc;

use log::*;

const DRIVER_NAME: &str = "ixy-ixgbe";

const MAX_RX_QUEUE_ENTRIES: u32 = 4096;
//const MAX_TX_QUEUE_ENTRIES: u32 = 4096;

const NUM_RX_QUEUE_ENTRIES: u32 = 512;
const NUM_TX_QUEUE_ENTRIES: u32 = 512;

const TX_CLEAN_BATCH: u32 = 32;

const fn wrap_ring(index: u32, ring_size: u32) -> u32 {
    (index + 1) & (ring_size - 1)
}


pub struct IxgbeDevice {
    addr: *mut u8,
    len: usize,
    num_rx_queues: u16,
    num_tx_queues: u16,
    rx_queues: Vec<IxgbeRxQueue>,
    tx_queues: Vec<IxgbeTxQueue>,
}

struct IxgbeRxQueue {
    descriptors: *mut ixgbe_adv_rx_desc,
    mempool: Rc<RefCell<Mempool>>,
    num_entries: u32,
    rx_index: u32,
    mempool_entries: Vec<u32>,
}

struct IxgbeTxQueue {
    descriptors: *mut ixgbe_adv_tx_desc,
    queue: VecDeque<Packet>,
    num_entries: u32,
    clean_index: u32,
    tx_index: u32,
}

fn reset_and_init(ixgbe: &mut IxgbeDevice, pci_addr: &str) -> Result<(), Box<Error>> {
    info!("resetting device {}", pci_addr);
    // section 4.6.3.1 - disable all interrupts
    ixgbe.set_reg32(IXGBE_EIMC, 0x7FFFFFFF);

    // section 4.6.3.2
    ixgbe.set_reg32(IXGBE_CTRL, IXGBE_CTRL_RST_MASK);
    ixgbe.wait_clear_reg32(IXGBE_CTRL, IXGBE_CTRL_RST_MASK);
    thread::sleep(Duration::from_millis(10));

    // section 4.6.3.1 - disable interrupts again after reset
    ixgbe.set_reg32(IXGBE_EIMC, 0x7FFFFFFF);

    info!("initializing device {}", pci_addr);

    // section 4.6.3 - wait for EEPROM auto read completion
    ixgbe.wait_set_reg32(IXGBE_EEC, IXGBE_EEC_ARD);

    // section 4.6.3 - wait for dma initialization done
    ixgbe.wait_set_reg32(IXGBE_RDRXCTL, IXGBE_RDRXCTL_DMAIDONE);

    // skip last step from 4.6.3 - we don't want interrups

    // section 4.6.4 - initialize link (auto negotiation)
    init_link(ixgbe);

    // section 4.6.5 - statistical counters
    // reset-on-read registers, just read them once
    ixgbe.reset_stats();

    // section 4.6.7 - init rx
    init_rx(ixgbe)?;

    // section 4.6.8 - init tx
    init_tx(ixgbe)?;

    for i in 0..ixgbe.num_rx_queues {
        start_rx_queue(ixgbe, i)?;
    }

    for i in 0..ixgbe.num_tx_queues {
        start_tx_queue(ixgbe, i)?;
    }

    // enable promisc mode by default to make testing easier
    ixgbe.set_promisc(true);

    // wait some time for the link to come up
    wait_for_link(ixgbe);

    Ok(())
}

// sections 4.6.7
fn init_rx(ixgbe: &mut IxgbeDevice) -> Result<(), Box<Error>> {
    // disable rx while re-configuring it
    ixgbe.clear_flags32(IXGBE_RXCTRL, IXGBE_RXCTRL_RXEN);

    // section 4.6.11.3.4 - allocate all queues and traffic to PB0
    ixgbe.set_reg32(IXGBE_RXPBSIZE(0), IXGBE_RXPBSIZE_128KB);
    for i in 1..8 {
        ixgbe.set_reg32(IXGBE_RXPBSIZE(i), 0);
    }

    // enable CRC offloading
    ixgbe.set_flags32(IXGBE_HLREG0, IXGBE_HLREG0_RXCRCSTRP);
    ixgbe.set_flags32(IXGBE_RDRXCTL, IXGBE_RDRXCTL_CRCSTRIP);

    // accept broadcast packets
    ixgbe.set_flags32(IXGBE_FCTRL, IXGBE_FCTRL_BAM);

    // configure queues, same for all queues
    for i in 0..ixgbe.num_rx_queues {
        debug!("initializing rx queue {}", i);
        // enable advanced rx descriptors
        ixgbe.set_reg32(IXGBE_SRRCTL(i as u32), (ixgbe.get_reg32(IXGBE_SRRCTL(i as u32)) & !IXGBE_SRRCTL_DESCTYPE_MASK) | IXGBE_SRRCTL_DESCTYPE_ADV_ONEBUF);
        // let nic drop packets if no rx descriptor is available instead of buffering them
        ixgbe.set_flags32(IXGBE_SRRCTL(i as u32), IXGBE_SRRCTL_DROP_EN);

        // section 7.1.9 - setup descriptor ring
        let ring_size_bytes = (NUM_RX_QUEUE_ENTRIES) as usize * mem::size_of::<ixgbe_adv_rx_desc>();

        let dma = DmaMemory::allocate(ring_size_bytes, true)?;

        // initialize to 0xff to prevent rogue memory accesses on premature dma activation
        unsafe { memset(dma.virt, ring_size_bytes, 0xff); }

        ixgbe.set_reg32(IXGBE_RDBAL(i as u32), (dma.phys as u64 & 0xffffffff) as u32);
        ixgbe.set_reg32(IXGBE_RDBAH(i as u32), (dma.phys as u64 >> 32) as u32);
        ixgbe.set_reg32(IXGBE_RDLEN(i as u32), ring_size_bytes as u32);

        debug!("rx ring {} phys addr: {:p}", i, dma.phys);
        debug!("rx ring {} virt addr: {:p}", i, dma.virt);

        // set ring to empty at start
        ixgbe.set_reg32(IXGBE_RDH(i as u32), 0);
        ixgbe.set_reg32(IXGBE_RDT(i as u32), 0);


        let mempool_size = if NUM_RX_QUEUE_ENTRIES + NUM_TX_QUEUE_ENTRIES < 4096 {
            4096
        } else {
            NUM_RX_QUEUE_ENTRIES + NUM_TX_QUEUE_ENTRIES
        };

        let mempool = Rc::new(
            RefCell::new(
                Mempool::allocate(mempool_size, 2048).unwrap()
            )
        );

        let rx_queue = IxgbeRxQueue {
            descriptors: dma.virt as *mut ixgbe_adv_rx_desc,
            mempool,
            num_entries: NUM_RX_QUEUE_ENTRIES,
            rx_index: 0,
            mempool_entries: Vec::with_capacity(MAX_RX_QUEUE_ENTRIES as usize),
        };

        ixgbe.rx_queues.push(rx_queue);
    }

    // last sentence of section 4.6.7 - set some magic bits
    ixgbe.set_flags32(IXGBE_CTRL_EXT, IXGBE_CTRL_EXT_NS_DIS);

    // probably a broken feature, this flag is initialized with 1 but has to be set to 0
    for i in 0..ixgbe.num_rx_queues {
        ixgbe.clear_flags32(IXGBE_DCA_RXCTRL(i as u32), 1 << 12);
    }

    // start rx
    ixgbe.set_flags32(IXGBE_RXCTRL, IXGBE_RXCTRL_RXEN);

    Ok(())
}

// section 4.6.8
fn init_tx(ixgbe: &mut IxgbeDevice) -> Result<(), Box<Error>> {
    // crc offload and small packet padding
    ixgbe.set_flags32(IXGBE_HLREG0, IXGBE_HLREG0_TXCRCEN | IXGBE_HLREG0_TXPADEN);

    // section 4.6.11.3.4 - set default buffer size allocations
    ixgbe.set_reg32(IXGBE_TXPBSIZE(0), IXGBE_TXPBSIZE_40KB);
    for i in 1..8 {
        ixgbe.set_reg32(IXGBE_TXPBSIZE(i), 0);
    }

    // required when not using DCB/VTd
    ixgbe.set_reg32(IXGBE_DTXMXSZRQ, 0xffff);
    ixgbe.clear_flags32(IXGBE_RTTDCS, IXGBE_RTTDCS_ARBDIS);

    // configure queues
    for i in 0..ixgbe.num_tx_queues {
        debug!("initializing tx queue {}", i);
        // section 7.1.9 - setup descriptor ring
        let ring_size_bytes = NUM_TX_QUEUE_ENTRIES as usize * mem::size_of::<ixgbe_adv_tx_desc>();

        let dma = DmaMemory::allocate(ring_size_bytes, true)?;
        unsafe { memset(dma.virt, ring_size_bytes, 0xff); }

        ixgbe.set_reg32(IXGBE_TDBAL(i as u32), (dma.phys as u64 & 0xffffffff) as u32);
        ixgbe.set_reg32(IXGBE_TDBAH(i as u32), (dma.phys as u64 >> 32) as u32);
        ixgbe.set_reg32(IXGBE_TDLEN(i as u32), ring_size_bytes as u32);

        debug!("tx ring {} phys addr: {:p}", i, dma.phys);
        debug!("tx ring {} virt addr: {:p}", i, dma.virt);

        // descriptor writeback magic values, important to get good performance and low PCIe overhead
        // see 7.2.3.4.1 and 7.2.3.5 for an explanation of these values and how to find good ones
        // we just use the defaults from DPDK here, but this is a potentially interesting point for optimizations
        let mut txdctl = ixgbe.get_reg32(IXGBE_TXDCTL(i as u32));
        // there are no defines for this in constants.rs for some reason
        // pthresh: 6:0, hthresh: 14:8, wthresh: 22:16
        txdctl &= !(0x3F | (0x3F << 8) | (0x3F << 16));
        txdctl |= 36 | (8 << 8) | (4 << 16);

        ixgbe.set_reg32(IXGBE_TXDCTL(i as u32), txdctl);

        let tx_queue = IxgbeTxQueue {
            descriptors: dma.virt as *mut ixgbe_adv_tx_desc,
            queue: VecDeque::new(),
            num_entries: NUM_RX_QUEUE_ENTRIES,
            clean_index: 0,
            tx_index: 0,
        };

        ixgbe.tx_queues.push(tx_queue);
    }

    // final step: enable DMA
    ixgbe.set_reg32(IXGBE_DMATXCTL, IXGBE_DMATXCTL_TE);

    Ok(())
}

fn start_rx_queue(ixgbe: &mut IxgbeDevice, queue_id: u16) -> Result<(), Box<Error>> {
    debug!("starting rx queue {}", queue_id);
    {
        let queue = &mut ixgbe.rx_queues[queue_id as usize];

        if queue.num_entries & (queue.num_entries - 1) != 0 {
            return Err(Box::new(std::io::Error::new(ErrorKind::Other, "number of queue entries must be a power of 2")))
        }

        for i in 0..queue.num_entries {
            let pool = &queue.mempool;

            let mut buf;

            if let Some(x) = pool.borrow_mut().pkt_buf_alloc() {
                buf = x;
            } else {
                break;
            }

            unsafe {
                // write to ixgbe_adv_rx_desc.read.pkt_addr
                ptr::write_volatile(&mut (*queue.descriptors.offset(i as isize)).read.pkt_addr as *mut u64, pool.borrow().get_phys_addr(buf as usize) as u64);

                // write to ixgbe_adv_rx_desc.read.hdr_addr
                ptr::write_volatile(&mut (*queue.descriptors.offset(i as isize)).read.hdr_addr as *mut u64, 0);
            }

            // we need to remember which descriptor entry belongs to which mempool entry
            queue.mempool_entries.push(buf);
        }
    }

    let queue = &ixgbe.rx_queues[queue_id as usize];

    // enable queue and wait if necessary
    ixgbe.set_flags32(IXGBE_RXDCTL(queue_id as u32), IXGBE_RXDCTL_ENABLE);
    ixgbe.wait_set_reg32(IXGBE_RXDCTL(queue_id as u32), IXGBE_RXDCTL_ENABLE);

    // rx queue starts out full
    ixgbe.set_reg32(IXGBE_RDH(queue_id as u32), 0);

    // was set to 0 before in the init function
    ixgbe.set_reg32(IXGBE_RDT(queue_id as u32), queue.num_entries - 1);

    Ok(())
}

fn start_tx_queue(ixgbe: &mut IxgbeDevice, queue_id: u16) -> Result<(), Box<Error>> {
    debug!("starting tx queue {}", queue_id);
    {
        let queue = &mut ixgbe.tx_queues[queue_id as usize];

        if queue.num_entries & (queue.num_entries - 1) != 0 {
            return Err(Box::new(std::io::Error::new(ErrorKind::Other, "number of queue entries must be a power of 2")))
        }
    }

    // tx queue starts out empty
    ixgbe.set_reg32(IXGBE_TDH(queue_id as u32), 0);
    ixgbe.set_reg32(IXGBE_TDT(queue_id as u32), 0);

    // enable queue and wait if necessary
    ixgbe.set_flags32(IXGBE_TXDCTL(queue_id as u32), IXGBE_TXDCTL_ENABLE);
    ixgbe.wait_set_reg32(IXGBE_TXDCTL(queue_id as u32), IXGBE_TXDCTL_ENABLE);

    Ok(())
}

fn ixgbe_rx_batch(ixgbe: &mut IxgbeDevice, queue_id: u32, buffer: &mut Vec<Packet>, num_packets: usize) -> usize {
    let mut rx_index;
    let mut last_rx_index;
    let mut received_packets = 0;

    {
        let queue = &mut ixgbe.rx_queues[queue_id as usize];

        rx_index = queue.rx_index;
        last_rx_index = queue.rx_index;

        for i in 0..num_packets {
            let desc = unsafe { queue.descriptors.offset(rx_index as isize) as *mut ixgbe_adv_rx_desc };
            let status = unsafe { ptr::read_volatile(&mut (*desc).wb.upper.status_error as *mut u32) };

            if (status & IXGBE_RXDADV_STAT_DD) != 0 {
                if (status & IXGBE_RXDADV_STAT_EOP) == 0 {
                    panic!("increase buffer size or decrease MTU")
                }

                let mut pool = queue.mempool.borrow_mut();

                let addr_virt = pool.get_virt_addr(queue.mempool_entries[rx_index as usize] as usize);
                let addr_phys = pool.get_phys_addr(queue.mempool_entries[rx_index as usize] as usize);
                // read ixgbe_adv_rx_desc.wb.upper.length
                let len = unsafe { ptr::read_volatile(&(*desc).wb.upper.length as *const u16) as usize };
                let mempool_entry = queue.mempool_entries[rx_index as usize];

                buffer.push(unsafe { Packet::new(addr_virt, addr_phys, len, &queue.mempool, mempool_entry) });

                if let Some(buf) = pool.pkt_buf_alloc() {
                    let addr_phys = pool.get_phys_addr(buf as usize);

                    unsafe {
                        // write to ixgbe_adv_rx_desc.read.pkt_addr
                        ptr::write_volatile(&mut (*desc).read.pkt_addr as *mut u64, addr_phys as u64);
                        // write to ixgbe_adv_rx_desc.read.hdr_addr
                        ptr::write_volatile(&mut (*desc).read.hdr_addr as *mut u64, 0);
                    }

                    queue.mempool_entries[rx_index as usize] = buf;
                } else {
                    // TODO handle this case properly
                    panic!("no buffer available");
                }

                last_rx_index = rx_index;
                rx_index = wrap_ring(rx_index, queue.num_entries);
                received_packets = i + 1;
            } else {
                break;
            }
        }
    }

    if rx_index != last_rx_index {
        ixgbe.set_reg32(IXGBE_RDT(queue_id), last_rx_index);
        ixgbe.rx_queues[queue_id as usize].rx_index = rx_index;
    }

    received_packets
}

fn ixgbe_tx_batch(ixgbe: &mut IxgbeDevice, queue_id: u32, packets: &mut Vec<Packet>) -> usize {
    let mut sent = 0;

    {
        let queue = &mut ixgbe.tx_queues[queue_id as usize];

        let mut clean_index = queue.clean_index;
        let mut cur_index = queue.tx_index;

        loop {
            let mut cleanable = cur_index as i32 - clean_index as i32;

            if cleanable < 0 {
                cleanable = queue.num_entries as i32 + cleanable;
            }

            if (cleanable as u32) < TX_CLEAN_BATCH {
                break;
            }

            let mut cleanup_to = clean_index + TX_CLEAN_BATCH - 1;

            if cleanup_to >= queue.num_entries {
                cleanup_to = cleanup_to - queue.num_entries;
            }

            // read from ixgbe_adv_tx_desc.wb.status
            let status = unsafe { ptr::read_volatile(&(*queue.descriptors.offset(cleanup_to as isize)).wb.status as *const u32) };

            if (status & IXGBE_ADVTXD_STAT_DD) != 0 {
                for _ in 0..cleanable {
                    queue.queue.pop_front();
                }
                clean_index = wrap_ring(cleanup_to, queue.num_entries);
            } else {
                break;
            }
        }

        queue.clean_index = clean_index;

        // TODO only take as many packets from the vector as are sent out
        for packet in packets.drain(..) {
            let next_index = wrap_ring(cur_index, queue.num_entries);

            if clean_index == next_index {
                return sent
            }

            queue.tx_index = wrap_ring(queue.tx_index, queue.num_entries);

            unsafe {
                // write to ixgbe_adv_tx_desc.read.buffer_addr
                ptr::write_volatile(&mut (*queue.descriptors.offset(cur_index as isize)).read.buffer_addr as *mut u64, packet.get_phys_addr() as u64);
                // write to ixgbe_adv_tx_desc.read.cmd_type_len
                ptr::write_volatile(&mut (*queue.descriptors.offset(cur_index as isize)).read.cmd_type_len as *mut u32, IXGBE_ADVTXD_DCMD_EOP | IXGBE_ADVTXD_DCMD_RS | IXGBE_ADVTXD_DCMD_IFCS | IXGBE_ADVTXD_DCMD_DEXT | IXGBE_ADVTXD_DTYP_DATA | packet.len() as u32);
                // write to ixgbe_adv_tx_desc.read.olinfo_status
                ptr::write_volatile(&mut (*queue.descriptors.offset(cur_index as isize)).read.olinfo_status as *mut u32, (packet.len() as u32) << IXGBE_ADVTXD_PAYLEN_SHIFT);
            }

            queue.queue.push_back(packet);

            cur_index = next_index;
            sent = sent + 1;
        }
    }

    ixgbe.set_reg32(IXGBE_TDT(queue_id), ixgbe.tx_queues[queue_id as usize].tx_index);

    sent
}

impl IxyDriver for IxgbeDevice {
    fn init(pci_addr: &str, num_rx_queues: u16, num_tx_queues: u16) -> Result<IxgbeDevice, Box<Error>> {
        if unsafe { libc::getuid() } != 0 {
            warn!("not running as root, this will probably fail");
        }

        if num_rx_queues > MAX_QUEUES {
            return Err(Box::new(std::io::Error::new(ErrorKind::Other, format!("cannot configure {} rx queues: limit is {}", num_rx_queues, MAX_QUEUES))))
        }

        if num_tx_queues > MAX_QUEUES {
            return Err(Box::new(std::io::Error::new(ErrorKind::Other, format!("cannot configure {} tx queues: limit is {}", num_tx_queues, MAX_QUEUES))))
        }

        println!("pci mapping device");

        let (addr, len) = pci_map_resource(pci_addr)?;
        let rx_queues = Vec::with_capacity(num_rx_queues as usize);
        let tx_queues = Vec::with_capacity(num_tx_queues as usize);
        let mut dev = IxgbeDevice { addr, len, num_rx_queues, num_tx_queues, rx_queues, tx_queues };

        reset_and_init(&mut dev, pci_addr)?;

        Ok(dev)
    }

    fn get_driver_name(&self) -> &str {
        DRIVER_NAME
    }

    fn rx_batch(&mut self, queue_id: u32, buffer: &mut Vec<Packet>, num_packets: usize) -> usize {
        ixgbe_rx_batch(self, queue_id, buffer, num_packets)
    }

    fn tx_batch(&mut self, queue_id: u32, packets: &mut Vec<Packet>) -> usize {
        ixgbe_tx_batch(self, queue_id, packets)
    }

    fn read_stats(&self, stats: &mut DeviceStats) {
        let rx_pkts = self.get_reg32(IXGBE_GPRC) as u64;
        let tx_pkts = self.get_reg32(IXGBE_GPTC) as u64;
        let rx_bytes = self.get_reg32(IXGBE_GORCL) as u64 + ((self.get_reg32(IXGBE_GORCH) as u64) << 32);
        let tx_bytes = self.get_reg32(IXGBE_GOTCL) as u64 + ((self.get_reg32(IXGBE_GOTCH) as u64) << 32);

        stats.rx_pkts += rx_pkts;
        stats.tx_pkts += tx_pkts;
        stats.rx_bytes += rx_bytes;
        stats.tx_bytes += tx_bytes;
    }

    fn reset_stats(&self) {
        self.get_reg32(IXGBE_GPRC);
        self.get_reg32(IXGBE_GPTC);
        self.get_reg32(IXGBE_GORCL);
        self.get_reg32(IXGBE_GORCH);
        self.get_reg32(IXGBE_GOTCL);
        self.get_reg32(IXGBE_GOTCH);
    }

    fn set_promisc(&self, enabled: bool) {
        if enabled {
            info!("enabling promisc mode");
            self.set_flags32(IXGBE_FCTRL, IXGBE_FCTRL_MPE | IXGBE_FCTRL_UPE);
        } else {
            info!("disabling promisc mode");
            self.clear_flags32(IXGBE_FCTRL, IXGBE_FCTRL_MPE | IXGBE_FCTRL_UPE);
        }
    }

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
    fn get_reg32(&self, reg: u32) -> u32 {
        if reg as usize <= self.len - 4 as usize {
            unsafe { ptr::read_volatile((self.addr as usize + reg as usize) as *mut u32) }
        } else {
            panic!("memory access out of bounds");
        }
    }

    fn set_reg32(&self, reg: u32, value: u32) {
        if reg as usize <= self.len - 4 as usize {
            unsafe { ptr::write_volatile((self.addr as usize + reg as usize) as *mut u32, value); }
        } else {
            panic!("memory access out of bounds");
        }
    }

    fn set_flags32(&self, reg: u32, flags: u32) {
        self.set_reg32(reg, self.get_reg32(reg) | flags);
    }

    fn clear_flags32(&self, reg: u32, flags: u32) {
        self.set_reg32(reg, self.get_reg32(reg) & !flags);
    }

    fn wait_clear_reg32(&self, reg: u32, value: u32) {
        loop {
            let current = self.get_reg32(reg);
            if (current & value) == 0 {
                break;
            }
            println!("Register: {:x}, current: {:x}, value: {:x}, expected: {:x}", reg, current, value, 0);
            thread::sleep(Duration::from_millis(100));
        }
    }

    fn wait_set_reg32(&self, reg: u32, value: u32) {
        loop {
            //let current = unsafe { ptr::read_volatile((self.addr + reg as usize) as *const u32) };
            let current = self.get_reg32(reg);
            if (current & value) == value {
                break;
            }
            println!("Register: {:x}, current: {:x}, value: {:x}, expected: ~{:x}", reg, current, value, value);
            thread::sleep(Duration::from_millis(100));
        }
    }
}

impl Drop for IxgbeDevice {
    fn drop(&mut self) {
        // TODO
    }
}

// see section 4.6.4
fn init_link(ixgbe: &IxgbeDevice) {
    // link auto-configuration register should already be set correctly, we're resetting it anyway
    ixgbe.set_reg32(IXGBE_AUTOC, (ixgbe.get_reg32(IXGBE_AUTOC) & !IXGBE_AUTOC_LMS_MASK) | IXGBE_AUTOC_LMS_10G_SERIAL);
    ixgbe.set_reg32(IXGBE_AUTOC, (ixgbe.get_reg32(IXGBE_AUTOC) & !IXGBE_AUTOC_10G_PMA_PMD_MASK) | IXGBE_AUTOC_10G_XAUI);
    // negotiate link
    ixgbe.set_flags32(IXGBE_AUTOC, IXGBE_AUTOC_AN_RESTART);
    // datasheet wants us to wait for the link here, but we can continue and wait afterwards
}

fn wait_for_link(ixgbe: &IxgbeDevice) {
    info!("waiting for link");
    let time = Instant::now();
    let mut speed = ixgbe.get_link_speed();
    while speed == 0 && time.elapsed().as_secs() > 10 {
        thread::sleep(Duration::from_millis(100));
        speed = ixgbe.get_link_speed();
    }
    info!("link speed is {} Mbit/s", ixgbe.get_link_speed());
}