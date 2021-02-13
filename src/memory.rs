use std::cell::RefCell;
use std::collections::{HashMap, VecDeque};
use std::error::Error;
use std::fmt::{self, Debug};
use std::io::{self, Read, Seek};
use std::ops::{Deref, DerefMut};
use std::os::unix::io::{AsRawFd, RawFd};
use std::rc::Rc;
use std::sync::atomic::{AtomicUsize, Ordering};
use std::sync::Mutex;
use std::{fs, mem, process, ptr, slice};

use crate::vfio::vfio_map_dma;

use lazy_static::lazy_static;

// from https://www.kernel.org/doc/Documentation/x86/x86_64/mm.txt
const X86_VA_WIDTH: u8 = 47;

const HUGE_PAGE_BITS: u32 = 21;
const HUGE_PAGE_SIZE: usize = 1 << HUGE_PAGE_BITS;

pub const IOVA_WIDTH: u8 = X86_VA_WIDTH;

// this differs from upstream ixy as our packet metadata is stored outside of the actual packet data
// which results in a different alignment requirement
pub const PACKET_HEADROOM: usize = 32;

static HUGEPAGE_ID: AtomicUsize = AtomicUsize::new(0);

// we want one VFIO Container for all NICs, so every NIC can read from every
// other NICs memory, especially the mempool. When not using the IOMMU / VFIO,
// this variable is unused.
pub(crate) static mut VFIO_CONTAINER_FILE_DESCRIPTOR: RawFd = -1;

lazy_static! {
    pub(crate) static ref VFIO_GROUP_FILE_DESCRIPTORS: Mutex<HashMap<i32, RawFd>> =
        Mutex::new(HashMap::new());
}

pub struct Dma<T> {
    pub virt: *mut T,
    pub phys: usize,
}

const MAP_HUGE_2MB: i32 = 0x5400_0000; // 21 << 26

impl<T> Dma<T> {
    /// Allocates dma memory on a huge page.
    pub fn allocate(size: usize, require_contiguous: bool) -> Result<Dma<T>, Box<dyn Error>> {
        let size = if size % HUGE_PAGE_SIZE != 0 {
            ((size >> HUGE_PAGE_BITS) + 1) << HUGE_PAGE_BITS
        } else {
            size
        };

        if get_vfio_container() != -1 {
            debug!("allocating dma memory via VFIO");

            let ptr = if IOVA_WIDTH < X86_VA_WIDTH {
                // To support IOMMUs capable of 39 bit wide IOVAs only, we use
                // 32 bit addresses. Since mmap() ignores libc::MAP_32BIT when
                // using libc::MAP_HUGETLB, we create a 32 bit address with the
                // right alignment (huge page size, e.g. 2 MB) on our own.

                // first allocate memory of size (needed size + 1 huge page) to
                // get a mapping containing the huge page size aligned address
                let addr = unsafe {
                    libc::mmap(
                        ptr::null_mut(),
                        size + HUGE_PAGE_SIZE,
                        libc::PROT_READ | libc::PROT_WRITE,
                        libc::MAP_PRIVATE | libc::MAP_ANONYMOUS | libc::MAP_32BIT,
                        -1,
                        0,
                    )
                };

                // calculate the huge page size aligned address by rounding up
                let aligned_addr = ((addr as isize + HUGE_PAGE_SIZE as isize - 1)
                    & -(HUGE_PAGE_SIZE as isize))
                    as *mut libc::c_void;

                let free_chunk_size = aligned_addr as usize - addr as usize;

                // free unneeded pages (i.e. all chunks of the additionally mapped huge page)
                unsafe {
                    libc::munmap(addr, free_chunk_size);
                    libc::munmap(aligned_addr.add(size), HUGE_PAGE_SIZE - free_chunk_size);
                }

                // finally map huge pages at the huge page size aligned 32 bit address
                unsafe {
                    libc::mmap(
                        aligned_addr as *mut libc::c_void,
                        size,
                        libc::PROT_READ | libc::PROT_WRITE,
                        libc::MAP_SHARED
                            | libc::MAP_ANONYMOUS
                            | libc::MAP_HUGETLB
                            | MAP_HUGE_2MB
                            | libc::MAP_FIXED,
                        -1,
                        0,
                    )
                }
            } else {
                unsafe {
                    libc::mmap(
                        ptr::null_mut(),
                        size,
                        libc::PROT_READ | libc::PROT_WRITE,
                        libc::MAP_SHARED | libc::MAP_ANONYMOUS | libc::MAP_HUGETLB | MAP_HUGE_2MB,
                        -1,
                        0,
                    )
                }
            };

            // This is the main IOMMU work: IOMMU DMA MAP the memory...
            if ptr == libc::MAP_FAILED {
                Err(format!(
                    "failed to memory map DMA-memory. Errno: {}",
                    std::io::Error::last_os_error()
                )
                .into())
            } else {
                let iova = vfio_map_dma(ptr as usize, size)?;

                let memory = Dma {
                    virt: ptr as *mut T,
                    phys: iova,
                };

                Ok(memory)
            }
        } else {
            debug!("allocating dma memory via huge page");

            if require_contiguous && size > HUGE_PAGE_SIZE {
                return Err("failed to map physically contiguous memory".into());
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
                        )
                    };

                    if ptr == libc::MAP_FAILED {
                        Err("failed to memory map huge page - huge pages enabled and free?".into())
                    } else if unsafe { libc::mlock(ptr as *mut libc::c_void, size) } == 0 {
                        let memory = Dma {
                            virt: ptr as *mut T,
                            phys: virt_to_phys(ptr as usize)?,
                        };

                        Ok(memory)
                    } else {
                        Err("failed to memory lock huge page".into())
                    }
                }
                Err(ref e) if e.kind() == io::ErrorKind::NotFound => Err(Box::new(io::Error::new(
                    io::ErrorKind::NotFound,
                    format!(
                        "huge page {} could not be created - huge pages enabled?",
                        path
                    ),
                ))),
                Err(e) => Err(Box::new(e)),
            }
        }
    }
}

pub struct Packet {
    pub(crate) addr_virt: *mut u8,
    pub(crate) addr_phys: usize,
    pub(crate) len: usize,
    pub(crate) pool: Rc<Mempool>,
    pub(crate) pool_entry: usize,
}

impl Clone for Packet {
    fn clone(&self) -> Self {
        let mut p = alloc_pkt(&self.pool, self.len).expect("no buffer available");
        p.clone_from_slice(&self);

        p
    }
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

impl Debug for Packet {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        (**self).fmt(f)
    }
}

impl Drop for Packet {
    fn drop(&mut self) {
        self.pool.free_buf(self.pool_entry);
    }
}

impl Packet {
    /// Returns a new `Packet`.
    pub(crate) unsafe fn new(
        addr_virt: *mut u8,
        addr_phys: usize,
        len: usize,
        pool: Rc<Mempool>,
        pool_entry: usize,
    ) -> Packet {
        Packet {
            addr_virt,
            addr_phys,
            len,
            pool,
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
    pub fn get_pool(&self) -> &Rc<Mempool> {
        &self.pool
    }

    /// Prefetch the (first cacheline of) packet content.
    ///
    /// The temporal consistency is chosen by the user, where strong consistency will lead to lower
    /// access times at the cost of cache space in stepwise lower cache tiers (smaller). This
    /// method is only available on `x86` or `x86_64` architectures with `sse` enabled.
    ///
    /// ```bash
    /// RUSTFLAGS="-C target-cpu=native -C target-feature=+sse" cargo build …
    /// ```
    #[cfg(all(
        any(target_arch = "x86", target_arch = "x86_64"),
        target_feature = "sse"
    ))]
    #[inline(always)]
    pub fn prefetch(&self, hint: Prefetch) {
        #[cfg(target_arch = "x86")]
        use core::arch::x86;
        #[cfg(target_arch = "x86_64")]
        use core::arch::x86_64 as x86;

        let addr = self.get_virt_addr() as *const _;
        unsafe {
            match hint {
                Prefetch::Time0 => x86::_mm_prefetch(addr, x86::_MM_HINT_T0),
                Prefetch::Time1 => x86::_mm_prefetch(addr, x86::_MM_HINT_T1),
                Prefetch::Time2 => x86::_mm_prefetch(addr, x86::_MM_HINT_T2),
                Prefetch::NonTemporal => x86::_mm_prefetch(addr, x86::_MM_HINT_NTA),
            }
        }
    }

    /// Shorten the packet.
    ///
    /// Can be used to shorten an already allocated packet, for example when packets were
    /// preallocated in bulk. If len is greater than the packet's current length, this has no
    /// effect.
    pub fn truncate(&mut self, len: usize) {
        // Validity invariant: the referred to memory range is a proper subset of the previous one.
        self.len = self.len.min(len)
    }

    /// Returns a mutable slice to the headroom of the packet.
    ///
    /// The `len` parameter controls how much of the headroom is returned.
    ///
    /// # Panics
    ///
    /// Panics if `len` is greater than [`PACKET_HEADROOM`].
    pub fn headroom_mut(&mut self, len: usize) -> &mut [u8] {
        assert!(len <= PACKET_HEADROOM);
        unsafe { slice::from_raw_parts_mut(self.addr_virt.sub(len), len) }
    }
}

/// Common representation for prefetch strategies.
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum Prefetch {
    /// Corresponds to _MM_HINT_T0 on x86 sse.
    Time0,

    /// Corresponds to _MM_HINT_T1 on x86 sse.
    Time1,

    /// Corresponds to _MM_HINT_T2 on x86 sse.
    Time2,

    /// Corresponds to _MM_HINT_NTA on x86 sse.
    NonTemporal,
}

pub struct Mempool {
    base_addr: *mut u8,
    num_entries: usize,
    entry_size: usize,
    phys_addresses: Vec<usize>,
    pub(crate) free_stack: RefCell<Vec<usize>>,
}

impl Mempool {
    /// Allocates a new `Mempool`.
    ///
    /// # Panics
    ///
    /// Panics if `size` is not a divisor of the page size.
    pub fn allocate(entries: usize, size: usize) -> Result<Rc<Mempool>, Box<dyn Error>> {
        let entry_size = match size {
            0 => 2048,
            x => x,
        };

        if (get_vfio_container() == -1) && HUGE_PAGE_SIZE % entry_size != 0 {
            panic!("entry size must be a divisor of the page size");
        }

        let dma: Dma<u8> = Dma::allocate(entries * entry_size, false)?;
        let mut phys_addresses = Vec::with_capacity(entries);

        for i in 0..entries {
            if get_vfio_container() != -1 {
                phys_addresses.push(dma.phys + (i * entry_size));
            } else {
                phys_addresses
                    .push(unsafe { virt_to_phys(dma.virt.add(i * entry_size) as usize)? });
            }
        }

        let pool = Mempool {
            base_addr: dma.virt,
            num_entries: entries,
            entry_size,
            phys_addresses,
            free_stack: RefCell::new(Vec::with_capacity(entries)),
        };

        unsafe { memset(pool.base_addr, pool.num_entries * pool.entry_size, 0x00) }

        let pool = Rc::new(pool);
        pool.free_stack.borrow_mut().extend(0..entries);

        Ok(pool)
    }

    /// Returns the position of a free buffer in the memory pool, or [`None`] if the pool is empty.
    pub(crate) fn alloc_buf(&self) -> Option<usize> {
        self.free_stack.borrow_mut().pop()
    }

    /// Marks a buffer in the memory pool as free.
    pub(crate) fn free_buf(&self, id: usize) {
        assert!(id < self.num_entries, "buffer outside of memory pool");

        self.free_stack.borrow_mut().push(id);
    }

    /// Returns the virtual address of a buffer from the memory pool.
    pub(crate) fn get_virt_addr(&self, id: usize) -> *mut u8 {
        assert!(id < self.num_entries, "buffer outside of memory pool");

        unsafe { self.base_addr.add(id * self.entry_size) }
    }

    /// Returns the physical address of a buffer from the memory pool.
    pub(crate) fn get_phys_addr(&self, id: usize) -> usize {
        self.phys_addresses[id]
    }

    /// Returns the size of the buffers in the memory pool.
    pub fn entry_size(&self) -> usize {
        self.entry_size
    }
}

/// Returns `num_packets` free packets from the `pool` with size `packet_size`.
pub fn alloc_pkt_batch(
    pool: &Rc<Mempool>,
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
pub fn alloc_pkt(pool: &Rc<Mempool>, size: usize) -> Option<Packet> {
    if size > pool.entry_size - PACKET_HEADROOM {
        return None;
    }

    pool.alloc_buf().map(|id| unsafe {
        Packet::new(
            pool.get_virt_addr(id).add(PACKET_HEADROOM),
            pool.get_phys_addr(id) + PACKET_HEADROOM,
            size,
            Rc::clone(pool),
            id,
        )
    })
}

/// Initializes `len` fields of type `T` at `addr` with `value`.
pub(crate) unsafe fn memset<T: Copy>(addr: *mut T, len: usize, value: T) {
    for i in 0..len {
        ptr::write_volatile(addr.add(i) as *mut T, value);
    }
}

/// Translates a virtual address to its physical counterpart.
pub(crate) fn virt_to_phys(addr: usize) -> Result<usize, Box<dyn Error>> {
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

pub(crate) fn get_vfio_container() -> RawFd {
    unsafe { VFIO_CONTAINER_FILE_DESCRIPTOR }
}

pub(crate) fn set_vfio_container(cfd: RawFd) {
    unsafe { VFIO_CONTAINER_FILE_DESCRIPTOR = cfd }
}
