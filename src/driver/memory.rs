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


const HUGE_PAGE_BITS: u32 = 21;
const HUGE_PAGE_SIZE: u32 = 1 << HUGE_PAGE_BITS;

pub struct DmaMemory {
    pub virt: *mut u8,
    pub phys: *mut u8,
}

impl DmaMemory {
    pub fn allocate(id: &mut u32, size: u32) -> Result<(DmaMemory), Box<Error>> {
        let path = format!("/mnt/huge/ixy-{}-{}", process::id(), id);

        *id = *id + 1;

        match fs::OpenOptions::new().read(true).write(true).create(true).open(path) {
            Ok(f) => {
                let ptr = unsafe {
                    libc::mmap(
                        ptr::null_mut(),
                        size as usize,
                        libc::PROT_READ | libc::PROT_WRITE,
                        libc::MAP_SHARED | libc::MAP_HUGETLB,
                        f.as_raw_fd(),
                        0,
                    ) as *mut u8
                };

                if ptr.is_null() || (ptr as isize) < 0 {
                    Err(Box::new(std::io::Error::new(ErrorKind::Other, "memory mapping failed")))
                } else {
                    if unsafe { libc::mlock(ptr as *mut libc::c_void, size as usize) } == 0 {
                        let memory = DmaMemory {
                            virt: ptr,
                            phys: virt_to_phys(ptr as usize).unwrap(),
                        };

                        Ok(memory)
                    } else {
                        Err(Box::new(std::io::Error::new(ErrorKind::Other, "memory locking failed")))
                    }
                }
            }
            Err(ref e) if e.kind() == std::io::ErrorKind::NotFound => Err(Box::new(std::io::Error::new(ErrorKind::NotFound, "did you forget to enable hugepages?"))),
            Err(e) => Err(Box::new(e)),
        }
    }
}

pub struct Packet {
    addr: *mut u8,
    len: usize,
    mempool: Rc<RefCell<Mempool>>,
    mempool_entry: u32,
}

impl Deref for Packet {
    type Target = [u8];

    fn deref(&self) -> &[u8] {
        unsafe { slice::from_raw_parts(self.addr, self.len) }
    }
}

impl DerefMut for Packet {
    fn deref_mut(&mut self) -> &mut [u8] {
        unsafe { slice::from_raw_parts_mut(self.addr, self.len) }
    }
}

impl Drop for Packet {
    fn drop(&mut self) {
        self.mempool.borrow_mut().pkt_buf_free(self.mempool_entry);
    }
}

impl Packet {
    pub fn new(addr: *mut u8, len: usize, mempool: &Rc<RefCell<Mempool>>, mempool_entry: u32) -> Packet {
        Packet { addr, len, mempool: mempool.clone(), mempool_entry }
    }

    pub fn get_addr(&self) -> *mut u8 {
        self.addr
    }
}

pub struct Mempool {
    pub base_addr: *mut u8,
    pub num_entries: u32,
    pub entry_size: u32,
    pub free_stack: Vec<u32>,
}

impl Mempool {
    pub fn allocate(id: &mut u32, entries: u32, size: u32) -> Result<Mempool, Box<Error>> {
        let entry_size = match size {
            0 => 2048,
            x => x,
        };

        let dma = DmaMemory::allocate(id, entries * size).unwrap();
        let mut mempool = Mempool {
            num_entries: entries,
            entry_size: size,
            base_addr: dma.virt,
            free_stack: Vec::new(),
        };

        unsafe { memset(mempool.base_addr, mempool.num_entries * mempool.entry_size, 0x00) }

        if HUGE_PAGE_SIZE % entry_size as u32 != 0 {
            panic!("entry size must be a divisor of the page size");
        }

        for i in 0..entries {
            let virt_addr = (mempool.base_addr as usize) + (i * entry_size) as usize;
            let phys_addr = virt_to_phys(virt_addr).unwrap();

            mempool.free_stack.push(i);
        }

        Ok(mempool)
    }

    pub fn offset(&self, offset: u32) -> *mut u8 {
        (self.base_addr as usize + (offset * self.entry_size) as usize) as *mut u8
    }

    pub fn dump(&self) {
        for i in 0..10 {
            let addr = unsafe {self.base_addr.offset((i*self.entry_size) as isize) } as *mut u64;
            println!("{:#0x}: {:x}", addr as usize, unsafe { ptr::read_volatile(addr) });
        }
    }

    pub fn pkt_buf_alloc_batch(&mut self, num_bufs: usize) -> Vec<u32> {
        let len = self.free_stack.len();

        if self.free_stack.len() < num_bufs {
            self.free_stack.clone()
        } else {
            self.free_stack.split_off(len - num_bufs)
        }
    }

    pub fn pkt_buf_alloc(&mut self) -> u32 {
        self.pkt_buf_alloc_batch(1)[0]
    }

    pub fn pkt_buf_free(&mut self, entry: u32) {
        self.free_stack.push(entry);
    }
}

pub unsafe fn memset(addr: *mut u8, len: u32, value: u8) {
    for i in 0..len {
        ptr::write_volatile(addr.offset(i as isize) as *mut u8, value);
    }
}

pub fn virt_to_phys(addr: usize) -> Result<*mut u8, Box<Error>> {
    let pagesize = unsafe { libc::sysconf(libc::_SC_PAGE_SIZE) } as usize;

    let mut file = fs::OpenOptions::new().read(true).open("/proc/self/pagemap")?;
    file.seek(SeekFrom::Start((addr / pagesize * mem::size_of::<usize>()) as u64))?;

    let mut buffer = [0; mem::size_of::<usize>()];
    file.read_exact(&mut buffer)?;

    let phys = unsafe { std::mem::transmute::<[u8; mem::size_of::<usize>()], usize>(buffer) }.to_le();

    Ok(((phys & 0x7fffffffffffff) * pagesize + (addr) % pagesize) as *mut u8)
}

pub fn allocate_mempool(id: &mut u32, entries: u32, size: u32) -> Result<Rc<RefCell<Mempool>>, Box<Error>> {
    let entry_size = match size {
        0 => 2048,
        x => x,
    };

    let dma = DmaMemory::allocate(id, entries * size).unwrap();
    let mempool = Rc::new(
        RefCell::new(
            Mempool {
                num_entries: entries,
                entry_size: size,
                base_addr: dma.virt,
                free_stack: Vec::new(),
            }));

    if HUGE_PAGE_SIZE % entry_size as u32 != 0 {
        panic!("entry size must be a divisor of the page size");
    }

    for i in 0..entries {
        let mut pool = mempool.borrow_mut();
        let virt_addr = (pool.base_addr as usize) + (i * entry_size) as usize;
        let phys_addr = virt_to_phys(virt_addr).unwrap();

        pool.free_stack.push(i);
    }

    Ok(mempool)
}
