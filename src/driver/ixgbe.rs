use driver::*;
use std::thread;
use std::time::Duration;
use std::ptr;
use std::error::Error;

use self::constants::*;
use self::memory::*;
use self::pci::*;

const DRIVER_NAME: &str = "ixy-ixgbe";

const MAX_RX_QUEUE_ENTRIES: u32 = 4096;
const MAX_TX_QUEUE_ENTRIES: u32 = 4096;

const NUM_RX_QUEUE_ENTRIES: u32 = 512;
const NUM_TX_QUEUE_ENTRIES: u32 = 512;

const TX_CLEAN_BATCH: u32 = 32;

const fn wrap_ring(index: u32, ring_size: u32) -> u32 {
    (index + 1) & (ring_size - 1)
}

pub struct IxgbeDevice {
    //ixy: IxyDevice,
    // TODO: usize, *const u8 or other type for pointers? which where?
    addr: usize,
    num_rx_queues: u32,
    num_tx_queues: u32,
    rx_queues: Vec<IxgbeRxQueue>,
    tx_queues: Vec<IxgbeTxQueue>,
}

struct IxgbeRxQueue {
    descriptors: *mut ixgbe_adv_rx_desc,
    // TODO Option?!
    mempool: Option<Mempool>,
    num_entries: u32,
    rx_index: u32,
    virtual_addresses: Vec<*const usize>,
}

struct IxgbeTxQueue {
    descriptors: *mut ixgbe_adv_tx_desc,
    num_entries: u32,
    clean_index: u32,
    tx_index: u32,
    virtual_addresses: Vec<*const usize>,
}

unsafe fn reset_and_init(ixgbe: &mut IxgbeDevice) {
    let mut huge_page_id: u32 = 0;

    // section 4.6.3.1 - disable all interrupts
    set_reg32(ixgbe.addr, IXGBE_EIMC, 0x7FFFFFFF);

    // section 4.6.3.2
    set_reg32(ixgbe.addr, IXGBE_CTRL, IXGBE_CTRL_RST_MASK);
    wait_clear_reg32(ixgbe.addr, IXGBE_CTRL, IXGBE_CTRL_RST_MASK);
    thread::sleep(Duration::from_millis(10));

    // section 4.6.3.1 - disable interrupts again after reset
    set_reg32(ixgbe.addr, IXGBE_EIMC, 0x7FFFFFFF);

    println!("initializing device");

    // section 4.6.3 - wait for EEPROM auto read completion
    wait_set_reg32(ixgbe.addr, IXGBE_EEC, IXGBE_EEC_ARD);

    // section 4.6.3 - wait for dma initialization done
    wait_set_reg32(ixgbe.addr, IXGBE_RDRXCTL, IXGBE_RDRXCTL_DMAIDONE);

    println!("initializing link");

    // section 4.6.4 - initialize link (auto negotiation)
    init_link(ixgbe);

    println!("resetting stats");

    // section 4.6.5 - reset registers
    ixgbe.reset_stats();

    println!("initializing rx");

    // section 4.6.7 - init rx
    init_rx(ixgbe, &mut huge_page_id);

    println!("initializing tx");

    // section 4.6.8 - init tx
    init_tx(ixgbe, &mut huge_page_id);

    println!("starting rx queues");

    for i in 0..ixgbe.num_rx_queues {
        start_rx_queue(ixgbe, i, &mut huge_page_id);
    }

    println!("starting tx queues");

    for i in 0..ixgbe.num_tx_queues {
        start_tx_queue(ixgbe, i);
    }

    println!("starting promisc mode");

    ixgbe.set_promisc(true);

    println!("waiting for link");

    wait_for_link(ixgbe);
}

// sections 4.6.7
unsafe fn init_rx(ixgbe: &mut IxgbeDevice, huge_page_id: &mut u32) {
    // disable rx while re-configuring
    clear_flags32(ixgbe.addr, IXGBE_RXCTRL, IXGBE_RXCTRL_RXEN);

    // section 4.6.11.3.4 - allocate all queues and traffic to PB0
    set_reg32(ixgbe.addr, IXGBE_RXPBSIZE(0), IXGBE_RXPBSIZE_128KB);
    for i in 1..8 {
        set_reg32(ixgbe.addr, IXGBE_RXPBSIZE(i), 0);
    }

    // enable CRC offloading
    set_flags32(ixgbe.addr, IXGBE_HLREG0, IXGBE_HLREG0_RXCRCSTRP);
    set_flags32(ixgbe.addr, IXGBE_RDRXCTL, IXGBE_RDRXCTL_CRCSTRIP);

    // accept broadcast packets
    set_flags32(ixgbe.addr, IXGBE_FCTRL, IXGBE_FCTRL_BAM);

    // configure queues
    for i in 0..ixgbe.num_rx_queues {
        // TODO: IXGBE_SRRCTL(i) should be a const function but rust doesn't support if/else in const functions yet
        set_reg32(ixgbe.addr, IXGBE_SRRCTL(i), (get_reg32(ixgbe.addr, IXGBE_SRRCTL(i)) & !IXGBE_SRRCTL_DESCTYPE_MASK) | IXGBE_SRRCTL_DESCTYPE_ADV_ONEBUF);

        set_flags32(ixgbe.addr, IXGBE_SRRCTL(i), IXGBE_SRRCTL_DROP_EN);

        // section 7.1.9 - setup descriptor ring
        let ring_size_bytes = (NUM_RX_QUEUE_ENTRIES) * mem::size_of::<ixgbe_adv_rx_desc>() as u32;

        // TODO check result of allocate_dma_memory
        let dma = allocate_dma_memory(huge_page_id, ring_size_bytes).unwrap();

        memset(dma.virt, 0xff, ring_size_bytes);

        set_reg32(ixgbe.addr, IXGBE_RDBAL(i), (dma.phys as u64 & 0xffffffff) as u32);
        set_reg32(ixgbe.addr, IXGBE_RDBAH(i), (dma.phys as u64 >> 32) as u32);
        set_reg32(ixgbe.addr, IXGBE_RDLEN(i), ring_size_bytes as u32);

        set_reg32(ixgbe.addr, IXGBE_RDH(i), 0);
        set_reg32(ixgbe.addr, IXGBE_RDT(i), 0);

        let rx_queue = IxgbeRxQueue {
            descriptors: dma.virt as *mut ixgbe_adv_rx_desc,
            mempool: None,
            num_entries: NUM_RX_QUEUE_ENTRIES,
            rx_index: 0,
            virtual_addresses: Vec::new(),
        };

        ixgbe.rx_queues.push(rx_queue);
    }

    // last sentence of section 4.6.7
    set_flags32(ixgbe.addr, IXGBE_CTRL_EXT, IXGBE_CTRL_EXT_NS_DIS);

    // combine both for loops?
    for i in 0..ixgbe.num_rx_queues {
        clear_flags32(ixgbe.addr, IXGBE_DCA_RXCTRL(i), 1 << 12);
    }

    // start rx
    set_flags32(ixgbe.addr, IXGBE_RXCTRL, IXGBE_RXCTRL_RXEN);
}

// section 4.6.8
unsafe fn init_tx(ixgbe: &mut IxgbeDevice, huge_page_id: &mut u32) {
    // crc offload
    set_flags32(ixgbe.addr, IXGBE_HLREG0, IXGBE_HLREG0_TXCRCEN | IXGBE_HLREG0_TXPADEN);

    // section 4.6.11.3.4
    set_reg32(ixgbe.addr, IXGBE_TXPBSIZE(0), IXGBE_TXPBSIZE_40KB);
    for i in 1..8 {
        set_reg32(ixgbe.addr, IXGBE_TXPBSIZE(i), 0);
    }

    // required when not using DCB/VTd
    set_reg32(ixgbe.addr, IXGBE_DTXMXSZRQ, 0xffff);
    clear_flags32(ixgbe.addr, IXGBE_RTTDCS, IXGBE_RTTDCS_ARBDIS);

    // configure queues
    for i in 0..ixgbe.num_tx_queues {
        // setup descriptor ring, see section 7.1.9
        let ring_size_bytes = NUM_TX_QUEUE_ENTRIES * mem::size_of::<ixgbe_adv_tx_desc>() as u32;

        // TODO check result of allocate_dma_memory
        let dma = allocate_dma_memory(huge_page_id, ring_size_bytes).unwrap();
        memset(dma.virt, 0xff, ring_size_bytes);
        set_reg32(ixgbe.addr, IXGBE_TDBAL(i), (dma.phys as u64 & 0xffffffff) as u32);
        set_reg32(ixgbe.addr, IXGBE_TDBAH(i), (dma.phys as u64 >> 32) as u32);
        set_reg32(ixgbe.addr, IXGBE_TDLEN(i), ring_size_bytes as u32);

        let mut txdctl = get_reg32(ixgbe.addr, IXGBE_TXDCTL(i));

        txdctl &= !(0x3F | (0x3F << 8) | (0x3F << 16));
        txdctl |= 36 | (8 << 8) | (4 << 16);

        set_reg32(ixgbe.addr, IXGBE_TXDCTL(i), txdctl);

        let tx_queue = IxgbeTxQueue {
            descriptors: dma.virt as *mut ixgbe_adv_tx_desc,
            num_entries: NUM_RX_QUEUE_ENTRIES,
            clean_index: 0,
            tx_index: 0,
            virtual_addresses: Vec::new(),
        };

        ixgbe.tx_queues.push(tx_queue);
    }

    // final step: enable DMA
    set_reg32(ixgbe.addr, IXGBE_DMATXCTL, IXGBE_DMATXCTL_TE);
}

unsafe fn start_rx_queue(ixgbe: &mut IxgbeDevice, queue_id: u32, huge_page_id: &mut u32) {
    let queue = &mut ixgbe.rx_queues[queue_id as usize];

    let mempool_size = NUM_RX_QUEUE_ENTRIES + NUM_TX_QUEUE_ENTRIES;

    if mempool_size < 4096 {
        let mempool_size = 4096;
    }

    let mut mempool = allocate_mempool(huge_page_id, mempool_size, 2048).unwrap();

    if queue.num_entries & (queue.num_entries - 1) != 0 {
        panic!("number of queue entries must be a power of 2");
    }

    for i in 0..queue.num_entries {
        let mut rdx = &mut *((queue.descriptors as usize + (i as usize) * mem::size_of::<ixgbe_adv_rx_desc>()) as *mut ixgbe_adv_rx_desc);
        let buf = pkt_buf_alloc(&mut mempool);

        rdx.read.pkt_addr = buf.addr_phys as u64;
        rdx.read.hdr_addr = 0;

        queue.virtual_addresses.push(buf.addr_virt);

        //rdx.read.pkt_addr = virt_to_phys(mempool.base_addr as usize + (i * mempool.entry_size) as usize).unwrap() as u64;
        //rdx.read.hdr_addr = 0;
        //queue.virtual_addresses.push((mempool.base_addr as usize + (i * mempool.entry_size) as usize) as *const usize);
    }

    queue.mempool = Some(mempool);

    set_flags32(ixgbe.addr, IXGBE_RXDCTL(queue_id), IXGBE_RXDCTL_ENABLE);
    wait_set_reg32(ixgbe.addr, IXGBE_RXDCTL(queue_id), IXGBE_RXDCTL_ENABLE);

    // rx queue starts out full
    set_reg32(ixgbe.addr, IXGBE_RDH(queue_id), 0);

    // was set to 0 before in the init function
    set_reg32(ixgbe.addr, IXGBE_RDT(queue_id), queue.num_entries - 1);
}

unsafe fn start_tx_queue(ixgbe: &mut IxgbeDevice, queue_id: u32) {
    let queue = &mut ixgbe.tx_queues[queue_id as usize];

    let mempool_size = NUM_RX_QUEUE_ENTRIES * NUM_TX_QUEUE_ENTRIES;

    if queue.num_entries & (queue.num_entries - 1) != 0 {
        println!("number of queue entries must be a power of 2");
    }

    // tx queue starts out empty
    set_reg32(ixgbe.addr, IXGBE_TDH(queue_id), 0);
    set_reg32(ixgbe.addr, IXGBE_TDT(queue_id), 0);

    // enable queue and wait if necessary
    set_flags32(ixgbe.addr, IXGBE_TXDCTL(queue_id), IXGBE_TXDCTL_ENABLE);
    wait_set_reg32(ixgbe.addr, IXGBE_TXDCTL(queue_id), IXGBE_TXDCTL_ENABLE);
}

unsafe fn ixgbe_rx_batch(ixgbe: &mut IxgbeDevice, queue_id: u32, num_bufs: u32) -> Vec<*const usize> {
    let queue = &mut ixgbe.rx_queues[queue_id as usize];

    let mut packets = Vec::new();

    let mut rx_index = queue.rx_index;
    let mut last_rx_index = rx_index;

    for i in 0..num_bufs {
        let mut desc_ptr = &mut *((queue.descriptors as usize + (rx_index as usize) * mem::size_of::<ixgbe_adv_rx_desc>()) as *mut ixgbe_adv_rx_desc);
        let status = desc_ptr.wb.upper.status_error;
        if (status & IXGBE_RXDADV_STAT_DD) != 0 {
            println!("inside loop");
            if !(status & IXGBE_RXDADV_STAT_EOP) != 0 {
                panic!("increase buffer size or decrease MTU")
            }

            if let Some(ref mut mempool) = queue.mempool {
                packets.push(queue.virtual_addresses[i as usize]);
                desc_ptr.read.pkt_addr = virt_to_phys(mempool.base_addr as usize + (mempool.free_stack_top * mempool.entry_size) as usize).unwrap() as u64;
                desc_ptr.read.hdr_addr = 0;
                queue.virtual_addresses[i as usize] = (mempool.base_addr as usize + (mempool.free_stack_top * mempool.entry_size) as usize) as *const usize;
                mempool.free_stack_top = mempool.free_stack_top - 1;
            }

            last_rx_index = rx_index;
            rx_index = wrap_ring(rx_index, queue.num_entries);
        }
    }

    if rx_index != last_rx_index {
        set_reg32(ixgbe.addr, IXGBE_RDT(queue_id), last_rx_index);
        queue.rx_index = rx_index;
    }

    packets
}


impl IxyDriver for IxgbeDevice {
    // TODO: return proper result
    fn init(pci_addr: &str, num_rx_queues: u32, num_tx_queues: u32) -> Result<IxgbeDevice, Box<Error>> {
        // TODO: check if root
        if num_rx_queues > MAX_QUEUES || num_tx_queues > MAX_QUEUES {
            panic!("too many queues")
        }

        println!("pci mapping device");

        let addr = pci_map(pci_addr)?;

        let rx_queues = Vec::new();
        let tx_queues = Vec::new();
        let mut dev = IxgbeDevice { addr, num_rx_queues, num_tx_queues, rx_queues, tx_queues };

        /*let ixy = IxyDevice {
            pci_addr: pci_addr.to_string(),
            driver_name: DRIVER_NAME.to_string(),
            num_rx_queues: no_rx_queues,
            num_tx_queues: no_tx_queues,
            driver: Box::new(dev),
            //rx_batch: ixgbe_rx_batch,
            //tx_batch: ixgbe_tx_batch,
            //read_stats: ixgbe_read_stats,
            //set_promisc: ixgbe_set_promisc,
            //get_link_speed: ixgbe_get_link_speed,
        };*/

        // TODO: safe <-> unsafe
        unsafe { reset_and_init(&mut dev) };

        Ok(dev)
    }

    fn driver_name(&self) -> &str {
        DRIVER_NAME
    }

    fn rx_batch(&mut self, queue_id: u32, num_bufs: u32) -> Vec<*const usize> {
        unsafe { ixgbe_rx_batch(self, queue_id, num_bufs) }
    }

    fn tx_batch(&mut self, queue_id: u32, num_bufs: u32) -> Vec<*const usize> {
        unsafe { ixgbe_rx_batch(self, queue_id, num_bufs) }
    }

    fn read_stats(&self, stats: &mut DeviceStats) {
        unsafe {
            let rx_pkts = get_reg32(self.addr, IXGBE_GPRC) as u64;
            let tx_pkts = get_reg32(self.addr, IXGBE_GPTC) as u64;
            let rx_bytes = get_reg32(self.addr, IXGBE_GORCL) as u64 + ((get_reg32(self.addr, IXGBE_GORCH) as u64) << 32);
            let tx_bytes = get_reg32(self.addr, IXGBE_GOTCL) as u64 + ((get_reg32(self.addr, IXGBE_GOTCH) as u64) << 32);

            stats.rx_pkts += rx_pkts;
            stats.tx_pkts += tx_pkts;
            stats.rx_bytes += rx_bytes;
            stats.tx_bytes += tx_bytes;
        }
    }

    fn reset_stats(&self) {
        unsafe {
            let rx_pkts = get_reg32(self.addr, IXGBE_GPRC) as u64;
            let tx_pkts = get_reg32(self.addr, IXGBE_GPTC) as u64;
            let rx_bytes = get_reg32(self.addr, IXGBE_GORCL) as u64 + ((get_reg32(self.addr, IXGBE_GORCH) as u64) << 32);
            let tx_bytes = get_reg32(self.addr, IXGBE_GOTCL) as u64 + ((get_reg32(self.addr, IXGBE_GOTCH) as u64) << 32);
        }
    }

    fn set_promisc(&self, enabled: bool) {
        unsafe {
            if enabled {
                println!("enabling promisc mode");
                set_flags32(self.addr, IXGBE_FCTRL, IXGBE_FCTRL_MPE | IXGBE_FCTRL_UPE);
            } else {
                println!("disabling promisc mode");
                clear_flags32(self.addr, IXGBE_FCTRL, IXGBE_FCTRL_MPE | IXGBE_FCTRL_UPE);
            }
        }
    }

    fn get_link_speed(&self) -> u16 {
        unsafe {
            let speed = get_reg32(self.addr, IXGBE_LINKS);
            if (speed & IXGBE_LINKS_UP) == 0 {
                return 0
            }
            match speed & IXGBE_LINKS_SPEED_82599 {
                IXGBE_LINKS_SPEED_100_82599 => 100,
                IXGBE_LINKS_SPEED_1G_82599 => 1000,
                IXGBE_LINKS_SPEED_10G_82599 => 10000,
                _ => 0,
            }
        }
    }
}

// see section 4.6.4
unsafe fn init_link(ixgbe: &IxgbeDevice) {
    set_reg32(ixgbe.addr, IXGBE_AUTOC, (get_reg32(ixgbe.addr, IXGBE_AUTOC) & !IXGBE_AUTOC_LMS_MASK) | IXGBE_AUTOC_LMS_10G_SERIAL);
    set_reg32(ixgbe.addr, IXGBE_AUTOC, (get_reg32(ixgbe.addr, IXGBE_AUTOC) & !IXGBE_AUTOC_10G_PMA_PMD_MASK) | IXGBE_AUTOC_10G_XAUI);
    // negotiate link
    set_flags32(ixgbe.addr, IXGBE_AUTOC, IXGBE_AUTOC_AN_RESTART);
}

unsafe fn wait_for_link(ixgbe: &IxgbeDevice) {
    let mut max_wait = 10000; // 10 seconds
    let poll_interval = 10;
    let speed = ixgbe.get_link_speed();
    while speed == 0 && max_wait > 0 {
        thread::sleep(Duration::from_millis(poll_interval));
        max_wait -= poll_interval;
    }
    println!("Link speed is {} Mbit/s", ixgbe.get_link_speed());
}

unsafe fn get_reg32(addr: usize, reg: u32) -> u32 {
    ptr::read_volatile((addr + reg as usize) as *const u32)
}

unsafe fn set_reg32(addr: usize, reg: u32, value: u32) {
    ptr::write_volatile((addr + reg as usize) as *mut u32, value);
}

unsafe fn set_flags32(addr: usize, reg: u32, flags: u32) {
    set_reg32(addr, reg, get_reg32(addr, reg) | flags);
}

unsafe fn clear_flags32(addr: usize, reg: u32, flags: u32) {
    set_reg32(addr, reg, get_reg32(addr, reg) & !flags);
}

unsafe fn wait_clear_reg32(data: usize, register: u32, value: u32) {
    //asm!("" :::: "volatile" : "memory");
    loop {
        let current = ptr::read_volatile((data + register as usize) as *const u32);
        if (current & value) == 0 {
            break;
        }
        println!("Register: {:x}, current: {:x}, value: {:x}, expected: {:x}", register, current, value, 0);
        thread::sleep(Duration::from_millis(100));
        //asm!("" :::: "volatile" : "memory");
    }
}

unsafe fn wait_set_reg32(data: usize, register: u32, value: u32) {
    //asm!("" :::: "volatile" : "memory");
    loop {
        let current = ptr::read_volatile((data + register as usize) as *const u32);
        if (current & value) == value {
            break;
        }
        println!("Register: {:x}, current: {:x}, value: {:x}, expected: ~{:x}", register, current, value, value);
        thread::sleep(Duration::from_millis(100));
        //asm!("" :::: "volatile" : "memory");
    }
}

unsafe fn memset(addr: *const usize, value: u8, length: u32) {
    for i in 0..length {
        ptr::write_volatile((addr as usize + i as usize) as *mut u8, value);
    }
}