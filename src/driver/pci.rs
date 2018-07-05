use std::fs;
use std::error::Error;
use std::io::ErrorKind;
use std::io::Write;

use std::os::unix::prelude::AsRawFd;
use std::ptr;

pub fn pci_map(path: &str) -> Result<usize, Box<Error>> {
    let file = fs::OpenOptions::new()
        .read(true)
        .write(true)
        .open(path)?;
    let size = fs::metadata(path)?.len();

    let ptr = unsafe {
        libc::mmap(
            ptr::null_mut(),
            size as usize,
            libc::PROT_READ | libc::PROT_WRITE,
            libc::MAP_SHARED,
            file.as_raw_fd(),
            0,
        ) as *const u32
    };

    if ptr.is_null() || (ptr as isize) < 0 {
        Err(Box::new(std::io::Error::new(ErrorKind::Other, "pci mapping failed")))
    } else {
        Ok(ptr as usize)
    }
}

// echo -n "0000:03:00.1" > /sys/bus/pci/drivers/ixgbe/unbind
pub fn unbind_driver(pci_addr: &str) -> Result<(), Box<Error>> {
    // TODO: path length should not be greater then maximum allowed path length (everywhere!)
    let path = format!("/sys/bus/pci/devices/{}/driver/unbind", pci_addr);

    match fs::OpenOptions::new().write(true).open(path) {
        Ok(mut f) => Ok(write!(f, "{}", pci_addr)?),
        Err(ref e) if e.kind() == std::io::ErrorKind::NotFound => Ok(()),
        Err(e) => Err(Box::new(e)),
    }
}