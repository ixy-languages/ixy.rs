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
use IxgbeDevice;

const HUGE_PAGE_BITS: u32 = 21;
const HUGE_PAGE_SIZE: usize = 1 << HUGE_PAGE_BITS;

static HUGEPAGE_ID: AtomicUsize = ATOMIC_USIZE_INIT;

pub struct Dma<T> {
    pub virt: *mut T,
    pub phys: usize,
}

const VFIO_DMA_MAP_FLAG_READ: u32 = 1;
const VFIO_DMA_MAP_FLAG_WRITE: u32 = 2;
const VFIO_IOMMU_MAP_DMA: u64 = 15217;

/* struct vfio_iommu_type1_dma_map, grabbed from linux/vfio.h */
#[allow(non_camel_case_types)]
#[repr(C)]
struct vfio_iommu_type1_dma_map {
    argsz: u32,
    flags: u32,
    vaddr: *mut u8,
    iova: *mut u8,
    size: usize,
}

impl<T> Dma<T> {
    /// Allocates dma memory on a huge page.
    pub fn allocate(
        size: usize,
        require_contigous: bool,
        dev: *const IxgbeDevice,
    ) -> Result<Dma<T>, Box<Error>> {
        let size = if size % HUGE_PAGE_SIZE != 0 {
            ((size >> HUGE_PAGE_BITS) + 1) << HUGE_PAGE_BITS
        } else {
            size
        };

        if !unsafe { (*dev).iommu } {
            // get an anonymous mapped memory space from kernel
            let ptr = unsafe {
                libc::mmap(
                    ptr::null_mut(),
                    size,
                    libc::PROT_READ | libc::PROT_WRITE,
                    libc::MAP_PRIVATE | libc::MAP_ANONYMOUS,
                    0,
                    0,
                ) as *mut T
            };

            // This is the main IOMMU work: IOMMU DMA MAP the memory...
            if ptr.is_null() {
                Err("failed to memory map ".into())
            } else {
                let iommu_dma_map: vfio_iommu_type1_dma_map = vfio_iommu_type1_dma_map {
                    argsz: mem::size_of::<vfio_iommu_type1_dma_map> as u32,
                    vaddr: ptr as *mut u8,
                    size: size,
                    iova: ptr as *mut u8,
                    flags: VFIO_DMA_MAP_FLAG_READ | VFIO_DMA_MAP_FLAG_WRITE,
                };

                let ioctl_result =
                    unsafe { libc::ioctl((*dev).cfd, VFIO_IOMMU_MAP_DMA, &iommu_dma_map) };
                if ioctl_result != -1 {
                    let memory = Dma {
                        virt: iommu_dma_map.vaddr as *mut T,
                        phys: iommu_dma_map.iova as usize,
                    };

                    Ok(memory)
                } else {
                    Err("There was some problem mapping the DMA memory. Is ulimit set for this user?".into())
                }
            }
        } else {
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

                    if ptr.is_null() {
                        Err("failed to memory map hugepage - hugepages enabled and free?".into())
                    } else if unsafe { libc::mlock(ptr as *mut libc::c_void, size) } == 0 {
                        let memory = Dma {
                            virt: ptr,
                            phys: virt_to_phys(ptr as usize)?,
                        };

                        Ok(memory)
                    } else {
                        Err("failed to memory lock hugepage".into())
                    }
                }
                Err(ref e) if e.kind() == io::ErrorKind::NotFound => Err(Box::new(io::Error::new(
                    io::ErrorKind::NotFound,
                    format!(
                        "hugepage {} could not be created - hugepages enabled?",
                        path
                    ),
                ))),
                Err(e) => Err(Box::new(e)),
            }
        }
    }
}

#[derive(Clone)]
pub struct Packet {
    pub(crate) addr_virt: *mut u8,
    pub(crate) addr_phys: usize,
    pub(crate) len: usize,
    pub(crate) pool: Rc<RefCell<Mempool>>,
    pub(crate) pool_entry: usize,
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
        //println!("drop");
        self.pool.borrow_mut().free_buf(self.pool_entry);
    }
}

impl Packet {
    /// Returns a new `Packet`.
    pub(crate) unsafe fn new(
        addr_virt: *mut u8,
        addr_phys: usize,
        len: usize,
        pool: &Rc<RefCell<Mempool>>,
        pool_entry: usize,
    ) -> Packet {
        Packet {
            addr_virt,
            addr_phys,
            len,
            pool: pool.clone(),
            pool_entry,
        }
    }

    /// Returns the virtual address of the packet.
    pub fn get_virt_addr(&self) -> *mut u8 {
        self.addr_virt
    }

    /// Returns the physical address of the packet.
    pub fn get_phys_addr(&self) -> usize {
        self.addr_phys
    }

    /// Returns a reference to the packet`s pool.
    pub fn get_pool(&self) -> &Rc<RefCell<Mempool>> {
        &self.pool
    }
}

pub struct Mempool {
    base_addr: *mut u8,
    num_entries: usize,
    entry_size: usize,
    phys_addresses: Vec<usize>,
    pub(crate) free_stack: Vec<usize>,
}

impl Mempool {
    /// Allocates a new `Mempool`.
    ///
    /// # Panics
    ///
    /// Panics if `size` is not a divisor of the page size.
    pub fn allocate(
        entries: usize,
        size: usize,
        dev: *const IxgbeDevice,
    ) -> Result<Rc<RefCell<Mempool>>, Box<Error>> {
        let entry_size = match size {
            0 => 2048,
            x => x,
        };

        let dma: Dma<u8> = Dma::allocate(entries * entry_size, false, dev)?;
        let mut phys_addresses = Vec::with_capacity(entries);

        for i in 0..entries {
            phys_addresses.push(unsafe { virt_to_phys(dma.virt.add(i * entry_size) as usize)? });
        }

        let pool = Mempool {
            base_addr: dma.virt,
            num_entries: entries,
            entry_size,
            phys_addresses,
            free_stack: Vec::with_capacity(entries),
        };

        unsafe { memset(pool.base_addr, pool.num_entries * pool.entry_size, 0x00) }

        if HUGE_PAGE_SIZE % entry_size != 0 {
            panic!("entry size must be a divisor of the page size");
        }

        let pool = Rc::new(RefCell::new(pool));

        for i in 0..entries {
            pool.borrow_mut().free_stack.push(i);
        }

        Ok(pool)
    }

    /// Removes a packet from the packet pool and returns it, or [`None`] if the pool is empty.
    pub(crate) fn alloc_buf(&mut self) -> Option<usize> {
        self.free_stack.pop()
    }

    /// Returns a packet to the packet pool.
    pub(crate) fn free_buf(&mut self, id: usize) {
        self.free_stack.push(id);
    }

    /// Returns a packet to the packet pool.
    pub(crate) unsafe fn get_virt_addr(&self, id: usize) -> *mut u8 {
        self.base_addr.add(id * self.entry_size)
    }

    /// Returns a packet to the packet pool.
    pub(crate) unsafe fn get_phys_addr(&self, id: usize) -> usize {
        self.phys_addresses[id]
    }
}

/// Returns `num_packets` free packets from the `pool` with size `packet_size`.
pub fn alloc_pkt_batch(
    pool: &Rc<RefCell<Mempool>>,
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

/// Returns a free packet from the `pool`, or [`None`] if the requested packet size exceeds the
/// maximum size for that pool or if the pool is empty.
pub fn alloc_pkt(pool: &Rc<RefCell<Mempool>>, size: usize) -> Option<Packet> {
    let mut p = pool.borrow_mut();

    if size > p.entry_size {
        return None;
    }

    match p.alloc_buf() {
        Some(packet) => unsafe {
            Some(Packet::new(
                p.get_virt_addr(packet),
                p.get_phys_addr(packet),
                size,
                &pool.clone(),
                packet,
            ))
        },
        _ => None,
    }
}

/// Initializes `len` fields of type `T` at `addr` with `value`.
pub(crate) unsafe fn memset<T: Copy>(addr: *mut T, len: usize, value: T) {
    for i in 0..len {
        ptr::write_volatile(addr.add(i) as *mut T, value);
    }
}

/// Translates a virtual address to its physical counterpart.
pub(crate) fn virt_to_phys(addr: usize) -> Result<usize, Box<Error>> {
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
    Ok((phys & 0x007f_ffff_ffff_ffff) * pagesize + addr % pagesize)
}
