use std::cell::RefCell;
use std::collections::VecDeque;
use std::error::Error;
use std::fs;
use std::io::{self, Read, Seek};
use std::mem;
use std::ops::{Deref, DerefMut};
use std::os::unix::prelude::AsRawFd;
use std::process;
use std::rc::Rc;
use std::sync::atomic::{AtomicUsize, Ordering, ATOMIC_USIZE_INIT};
use std::{ptr, slice};

use libc;

const HUGE_PAGE_BITS: u32 = 21;
const HUGE_PAGE_SIZE: usize = 1 << HUGE_PAGE_BITS;

static HUGEPAGE_ID: AtomicUsize = ATOMIC_USIZE_INIT;

pub struct DmaMemory<T> {
    pub virt: *mut T,
    pub phys: *const usize,
}

impl<T> DmaMemory<T> {
    /// Allocates dma memory on a huge page.
    pub fn allocate(size: usize, require_contigous: bool) -> Result<DmaMemory<T>, Box<Error>> {
        let size = if size % HUGE_PAGE_SIZE != 0 {
            ((size >> HUGE_PAGE_BITS) + 1) << HUGE_PAGE_BITS
        } else {
            size
        };

        if require_contigous && size > HUGE_PAGE_SIZE {
            return Err("could not map physically contigous memory".into());
        }

        let id = HUGEPAGE_ID.fetch_add(1, Ordering::SeqCst);
        let path = format!("/mnt/huge/ixy-{}-{}", process::id(), id);

        match fs::OpenOptions::new()
            .read(true)
            .write(true)
            .create(true)
            .open(path.clone())
        {
            Ok(f) => {
                let ptr = unsafe {
                    libc::mmap(
                        ptr::null_mut(),
                        size,
                        libc::PROT_READ | libc::PROT_WRITE,
                        libc::MAP_SHARED | libc::MAP_HUGETLB,
                        f.as_raw_fd(),
                        0,
                    ) as *mut T
                };

                if ptr.is_null() || (ptr as isize) < 0 {
                    Err("memory mapping failed".into())
                } else if unsafe { libc::mlock(ptr as *mut libc::c_void, size) } == 0 {
                    let memory = DmaMemory {
                        virt: ptr,
                        phys: virt_to_phys(ptr as usize)?,
                    };

                    Ok(memory)
                } else {
                    Err("memory locking failed".into())
                }
            }
            Err(ref e) if e.kind() == io::ErrorKind::NotFound => Err(Box::new(io::Error::new(
                io::ErrorKind::NotFound,
                format!("{} could not be created - hugepages enabled?", path),
            ))),
            Err(e) => Err(Box::new(e)),
        }
    }
}

pub struct Packet {
    addr_virt: *mut u8,
    addr_phys: *const usize,
    len: usize,
    pool: Rc<RefCell<Packetpool>>,
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
        let p = unsafe { Packet::new(self.addr_virt, self.addr_phys, self.len, &self.pool) };
        self.pool.borrow_mut().free_pkt(p);
    }
}

impl Packet {
    /// Returns a new `Packet`.
    pub(crate) unsafe fn new(
        addr_virt: *mut u8,
        addr_phys: *const usize,
        len: usize,
        pool: &Rc<RefCell<Packetpool>>,
    ) -> Packet {
        Packet {
            addr_virt,
            addr_phys,
            len,
            pool: pool.clone(),
        }
    }

    /// Sets the size of the packet.
    pub unsafe fn set_size(&mut self, len: usize) {
        self.len = len
    }

    /// Returns the virtual address of the packet.
    pub fn get_virt_addr(&self) -> *mut u8 {
        self.addr_virt
    }

    /// Returns the physical address of the packet.
    pub fn get_phys_addr(&self) -> *const usize {
        self.addr_phys
    }

    /// Returns a reference to the packet`s pool.
    pub fn get_pool(&self) -> &Rc<RefCell<Packetpool>> {
        &self.pool
    }
}

pub struct Packetpool {
    base_addr: *mut u8,
    num_entries: usize,
    entry_size: usize,
    free_stack: Vec<Packet>,
}

impl Packetpool {
    /// Allocates a new `Packetpool`.
    ///
    /// # Panics
    ///
    /// Panics if `size` is not a divisor of the page size.
    pub fn allocate(entries: usize, size: usize) -> Result<Rc<RefCell<Packetpool>>, Box<Error>> {
        let entry_size = match size {
            0 => 2048,
            x => x,
        };

        let dma: DmaMemory<u8> = DmaMemory::allocate(entries * entry_size, false)?;
        let mut phys_addresses = Vec::with_capacity(entries);

        for i in 0..entries {
            phys_addresses.push(unsafe {
                virt_to_phys(dma.virt.offset((i * entry_size) as isize) as usize)?
            });
        }

        let pool = Packetpool {
            base_addr: dma.virt,
            num_entries: entries,
            entry_size,
            free_stack: Vec::with_capacity(entries),
        };

        unsafe { memset(pool.base_addr, pool.num_entries * pool.entry_size, 0x00) }

        if HUGE_PAGE_SIZE % entry_size != 0 {
            panic!("entry size must be a divisor of the page size");
        }

        let pool = Rc::new(RefCell::new(pool));

        for i in 0..entries {
            let addr_virt = unsafe { dma.virt.offset((i * entry_size) as isize) };
            let addr_phys = virt_to_phys(addr_virt as usize)?;
            let len = 0;
            let p = unsafe { Packet::new(addr_virt, addr_phys, len, &pool) };
            pool.borrow_mut().free_stack.push(p);
        }

        Ok(pool)
    }

    /// Removes a packet from the packet pool and returns it, or [`None`] if the pool is empty.
    pub fn alloc_pkt(&mut self) -> Option<Packet> {
        self.free_stack.pop()
    }

    /// Returns a packet to the packet pool.
    pub fn free_pkt(&mut self, p: Packet) {
        self.free_stack.push(p);
    }
}

/// Returns `num_packets` free packets from the `pool` with size `packet_size`.
pub fn alloc_pkt_batch(
    pool: &Rc<RefCell<Packetpool>>,
    buffer: &mut VecDeque<Packet>,
    num_packets: usize,
    packet_size: usize,
) -> usize {
    let mut allocated = 0;

    while let Some(p) = alloc_pkt(pool, packet_size) {
        buffer.push_back(p);

        allocated += 1;
        if allocated >= num_packets {
            break;
        }
    }

    allocated
}

/// Returns a free packet from the `pool`.
pub fn alloc_pkt(pool: &Rc<RefCell<Packetpool>>, size: usize) -> Option<Packet> {
    if size > pool.borrow().entry_size {
        return None;
    } /*else if let Some(mut p) = pool.borrow_mut().alloc_pkt() {
        unsafe { p.set_size(size) };
        return Some(p);
    }*/

    match pool.borrow_mut().alloc_pkt() {
        Some(mut p) => {
            unsafe { p.set_size(size) };
            Some(p)
        },
        _ => None,
    }
}

/// Initializes `len` bytes at `addr` with `value`.
pub unsafe fn memset<T: Copy>(addr: *mut T, len: usize, value: T) {
    for i in 0..len {
        ptr::write_volatile(addr.offset(i as isize) as *mut T, value);
    }
}

/// Translates a virtual address to its physical counterpart.
pub fn virt_to_phys(addr: usize) -> Result<*const usize, Box<Error>> {
    let pagesize = unsafe { libc::sysconf(libc::_SC_PAGE_SIZE) } as usize;

    let mut file = fs::OpenOptions::new()
        .read(true)
        .open("/proc/self/pagemap")?;

    file.seek(io::SeekFrom::Start(
        (addr / pagesize * mem::size_of::<usize>()) as u64,
    ))?;

    let mut buffer = [0; mem::size_of::<usize>()];
    file.read_exact(&mut buffer)?;

    let phys = unsafe { mem::transmute::<[u8; mem::size_of::<usize>()], usize>(buffer) };
    Ok(((phys & 0x007f_ffff_ffff_ffff) * pagesize + addr % pagesize) as *const usize)
}
