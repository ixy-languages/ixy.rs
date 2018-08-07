use std;
use std::fs;
use std::error::Error;

use std::io::prelude::*;
use std::io::ErrorKind;
use std::io::SeekFrom;

use std::process;
use std::os::unix::prelude::AsRawFd;
use std::mem;
use std::{ptr, slice};

use std::ops::{Deref, DerefMut};

use std::rc::Rc;
use std::cell::RefCell;
use libc;

use std::sync::atomic::{AtomicUsize, Ordering, ATOMIC_USIZE_INIT};

const HUGE_PAGE_BITS: u32 = 21;
const HUGE_PAGE_SIZE: u32 = 1 << HUGE_PAGE_BITS;

static HUGEPAGE_ID: AtomicUsize = ATOMIC_USIZE_INIT;

pub struct DmaMemory {
    pub virt: *mut u8,
    pub phys: *mut u8,
}

impl DmaMemory {
    pub fn allocate(size: usize) -> Result<(DmaMemory), Box<Error>> {
        let id = HUGEPAGE_ID.fetch_add(1, Ordering::SeqCst);
        let path = format!("/mnt/huge/ixy-{}-{}", process::id(), id);

        match fs::OpenOptions::new().read(true).write(true).create(true).open(path) {
            Ok(f) => {
                let ptr = unsafe {
                    libc::mmap(
                        ptr::null_mut(),
                        size,
                        libc::PROT_READ | libc::PROT_WRITE,
                        libc::MAP_SHARED | libc::MAP_HUGETLB,
                        f.as_raw_fd(),
                        0,
                    ) as *mut u8
                };

                if ptr.is_null() || (ptr as isize) < 0 {
                    Err(Box::new(std::io::Error::new(ErrorKind::Other, "memory mapping failed")))
                } else {
                    if unsafe { libc::mlock(ptr as *mut libc::c_void, size) } == 0 {
                        // TODO check physical address
                        let memory = DmaMemory {
                            virt: ptr,
                            phys: virt_to_phys(ptr).unwrap(),
                        };

                        Ok(memory)
                    } else {
                        Err(Box::new(std::io::Error::new(ErrorKind::Other, "memory locking failed")))
                    }
                }
            }
            Err(ref e) if e.kind() == std::io::ErrorKind::NotFound =>
                Err(Box::new(std::io::Error::new(ErrorKind::NotFound, "did you forget to enable hugepages?"))),
            Err(e) => Err(Box::new(e)),
        }
    }
}

pub struct Packet {
    addr_virt: *mut u8,
    addr_phys: *mut u8,
    len: usize,
    mempool: Rc<RefCell<Mempool>>,
    mempool_entry: u32,
}

impl Deref for Packet {
    type Target = [u8];

    fn deref(&self) -> &[u8] {
        unsafe { slice::from_raw_parts(self.addr_virt, self.len) }
    }
}

impl DerefMut for Packet {
    fn deref_mut(&mut self) -> &mut [u8] {
        unsafe { slice::from_raw_parts_mut(self.addr_virt, self.len) }
    }
}

impl Drop for Packet {
    fn drop(&mut self) {
        self.mempool.borrow_mut().pkt_buf_free(self.mempool_entry);
    }
}

impl Packet {
    pub(crate) unsafe fn new(addr_virt: *mut u8, addr_phys: *mut u8, len: usize,
                             mempool: &Rc<RefCell<Mempool>>, mempool_entry: u32) -> Packet {
        Packet { addr_virt, addr_phys, len, mempool: mempool.clone(), mempool_entry }
    }

    pub fn get_virt_addr(&self) -> *mut u8 {
        self.addr_virt
    }

    pub fn get_phys_addr(&self) -> *mut u8 {
        self.addr_phys
    }
}

pub struct Mempool {
    base_addr: *mut u8,
    num_entries: u32,
    entry_size: usize,
    free_stack: Vec<u32>,
    phys_addresses: Vec<*mut u8>,
}

impl Mempool {
    pub fn allocate(entries: u32, size: usize) -> Result<Mempool, Box<Error>> {
        let entry_size = match size {
            0 => 2048,
            x => x,
        };

        let dma = DmaMemory::allocate(entries as usize * entry_size).unwrap();

        let mut phys_addresses = Vec::with_capacity(entries as usize);

        for i in 0..entries {
            phys_addresses.push( unsafe {
                virt_to_phys(dma.virt.offset((i as usize * entry_size) as isize))
            }? );
        }

        let mut mempool = Mempool {
            num_entries: entries,
            entry_size,
            base_addr: dma.virt,
            free_stack: Vec::with_capacity(entries as usize),
            phys_addresses,
        };

        unsafe { memset(mempool.base_addr, mempool.num_entries as usize * mempool.entry_size, 0x00) }

        if HUGE_PAGE_SIZE % entry_size as u32 != 0 {
            panic!("entry size must be a divisor of the page size");
        }

        for i in 0..entries {
            mempool.free_stack.push(i);
        }

        Ok(mempool)
    }

    pub fn get_virt_addr(&self, offset: usize) -> *mut u8 {
        (self.base_addr as usize + (offset * self.entry_size) as usize) as *mut u8
    }

    pub fn get_phys_addr(&self, offset: usize) -> *mut u8 {
        self.phys_addresses[offset]
    }

    pub fn dump(&self) {
        for i in 0..10 {
            let addr = unsafe {
                self.base_addr.offset((i * self.entry_size) as isize)
            } as *mut u64;
            println!("{:#0x}: {:x}", addr as usize, unsafe { ptr::read_volatile(addr) });
        }
    }

    pub fn pkt_buf_alloc(&mut self) -> Option<u32> {
        self.free_stack.pop()
    }

    pub fn pkt_buf_free(&mut self, entry: u32) {
        self.free_stack.push(entry);
    }
}

pub fn alloc_packet_batch(mempool: &Rc<RefCell<Mempool>>, buffer: &mut Vec<Packet>, num_packets: usize, packet_size: usize) -> usize {
    let mut allocated = 0;

    while let Some(p) = alloc_packet(mempool, packet_size) {
        buffer.push(p);

        allocated += 1;
        if allocated == num_packets {
            break;
        } else {}
    }

    allocated
}

pub fn alloc_packet(mempool: &Rc<RefCell<Mempool>>, size: usize) -> Option<Packet> {
    if size > mempool.borrow().entry_size {
        return None
    }

    let buf = match mempool.borrow_mut().pkt_buf_alloc() {
        Some(buf) => buf,
        None => return None,
    };

    let addr_virt = mempool.borrow().get_virt_addr(buf as usize);
    let addr_phys = mempool.borrow().get_phys_addr(buf as usize);
    let len = size;

    Some(unsafe { Packet::new(addr_virt, addr_phys, len, mempool, buf) })
}

pub unsafe fn memset(addr: *mut u8, len: usize, value: u8) {
    for i in 0..len {
        ptr::write_volatile(addr.offset(i as isize) as *mut u8, value);
    }
}

pub fn virt_to_phys(addr: *mut u8) -> Result<*mut u8, Box<Error>> {
    let addr = addr as usize;
    let pagesize = unsafe { libc::sysconf(libc::_SC_PAGE_SIZE) } as usize;

    //println!("pagesize: {}", pagesize);
    //println!("addr: {:x}", addr);

    let mut file = fs::OpenOptions::new().read(true).open("/proc/self/pagemap")?;
    file.seek(SeekFrom::Start((addr / pagesize * mem::size_of::<usize>()) as u64))?;

    //println!("sizeof(usize): {}", mem::size_of::<usize>());
    //println!("seekposition: {:x}", (addr / pagesize * mem::size_of::<usize>()) as u64);

    let mut buffer = [0; mem::size_of::<usize>()];
    file.read_exact(&mut buffer)?;

    //println!("buffer: {:?}", buffer);

    let phys = unsafe { std::mem::transmute::<[u8; mem::size_of::<usize>()], usize>(buffer) };

    //println!("phy: {:x}", phys);
    //println!("result: {:p}", ((phys & 0x7fffffffffffff) * pagesize + addr % pagesize) as *mut u8);

    Ok(((phys & 0x7fffffffffffff) * pagesize + addr % pagesize) as *mut u8)
}