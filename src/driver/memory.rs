use std::fs;
use std::error::Error;

use std::io::prelude::*;
use std::io::ErrorKind;
use std::io::SeekFrom;

use std::process;
use std::os::unix::prelude::AsRawFd;
use std::mem;
use std::ptr;


const HUGE_PAGE_BITS: u32 = 21;
const HUGE_PAGE_SIZE: u32 = 1 << HUGE_PAGE_BITS;
//const SIZE_PKT_BUF_HEADROOM: usize = 40;


// TODO impl instead of public?!
pub struct Mempool {
    pub base_addr: *const usize,
    pub num_entries: u32,
    pub entry_size: u32,
    pub free_stack_top: u32,
    pub free_stack: Vec<u32>,
    pub pkt_buffer: Vec<PktBuf>,
}

pub struct DmaMemory {
    pub virt: *const usize,
    pub phys: *const usize,
}

pub struct PktBuf {
    pub id: u32,
    pub size: u32,
    pub addr_phys: *const usize,
    pub addr_virt: *const usize,
    //mempool: Mempool,
    //mempool_idx: u32,
    //head_room: [u8; SIZE_PKT_BUF_HEADROOM],
    //data: Vec<u8>,
}


pub fn virt_to_phys(addr: usize) -> Result<*const usize, Box<Error>> {
    let pagesize =  unsafe { libc::sysconf(libc::_SC_PAGE_SIZE) } as usize;

    let mut file = fs::OpenOptions::new().read(true).open("/proc/self/pagemap")?;
    file.seek(SeekFrom::Start((addr/pagesize*mem::size_of::<usize>()) as u64))?;

    let mut buffer = [0; mem::size_of::<usize>()];
    file.read_exact(&mut buffer)?;

    let phys = unsafe { std::mem::transmute::<[u8; mem::size_of::<usize>()], usize>(buffer) }.to_le();

    Ok(((phys & 0x7fffffffffffff) * pagesize + (addr) % pagesize) as *const usize)
}

pub fn allocate_dma_memory(id: &mut u32, size: u32) -> Result<(DmaMemory), Box<Error>> {
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
                ) as *const usize
            };

            if ptr.is_null() || (ptr as isize) < 0 {
                Err(Box::new(std::io::Error::new(ErrorKind::Other, "memory mapping failed")))
            } else {
                if unsafe { libc::mlock(ptr as *mut libc::c_void, size as usize) } == 0 {
                    // TODO check physical address
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


pub fn allocate_mempool(id: &mut u32, entries: u32, size: u32) -> Result<Mempool, Box<Error>> {
    let entry_size = match size {
        0 => 2048,
        x => x,
    };

    let dma = allocate_dma_memory(id, entries * size).unwrap();
    let mut mempool = Mempool {
        num_entries: entries,
        entry_size: size,
        base_addr: dma.virt,
        free_stack_top: entries,
        free_stack: Vec::new(),
        pkt_buffer: Vec::new()
    };

    if HUGE_PAGE_SIZE % entry_size as u32 != 0 {
        panic!("entry size must be a divisor of the page size");
    }

    for i in 0..entries {
        let virt_addr = (mempool.base_addr as usize) + (i*entry_size) as usize;
        let phys_addr = virt_to_phys(virt_addr).unwrap();

        mempool.free_stack.push(i);
        mempool.pkt_buffer.push(PktBuf {
            id: i,
            size: 0,
            addr_phys: phys_addr,
            addr_virt: virt_addr as *const usize
        });

        /*let buf_addr = (mempool.base_addr as usize + (i*entries) as usize) as *mut Packet;
        let mut buf = unsafe { &mut *buf_addr };
        buf.buf_addr_phys = virt_to_phys(buf_addr as usize).unwrap();
        buf.mempool_idx = i;
        buf.mempool = &mempool;
        buf.size = 0;*/
    }

    Ok(mempool)
}

pub fn pkt_buf_alloc_batch(mempool: &mut Mempool, num_bufs: u32) -> Vec<&PktBuf> {
    let mut pkt_buffer = Vec::new();

    for i in 0..num_bufs {
        pkt_buffer.push(&mempool.pkt_buffer[(mempool.free_stack_top -1) as usize]);
        mempool.free_stack_top = mempool.free_stack_top - 1;
    }

    pkt_buffer
}

pub fn pkt_buf_alloc(mempool: &mut Mempool) -> &PktBuf {
    pkt_buf_alloc_batch(mempool, 1)[0]
}

// TODO: mempool should be referenced by PktBuf
pub fn pkt_buf_free(mempool: &mut Mempool, pkt_buf: &PktBuf) {
    mempool.free_stack[mempool.free_stack_top as usize] = pkt_buf.id;
    mempool.free_stack_top = mempool.free_stack_top + 1;
}