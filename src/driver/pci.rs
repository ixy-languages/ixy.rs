use std::fs;
use std::error::Error;
use std::io::ErrorKind;
use std::io::Write;

use std::os::unix::prelude::AsRawFd;
use std::ptr;

pub fn pci_map(path: &str) -> Result<(*mut u8, usize), Box<Error>> {
    let file = fs::OpenOptions::new()
        .read(true)
        .write(true)
        .open(path)?;
    let len = fs::metadata(path)?.len() as usize;

    let ptr = unsafe {
        libc::mmap(
            ptr::null_mut(),
            len,
            libc::PROT_READ | libc::PROT_WRITE,
            libc::MAP_SHARED,
            file.as_raw_fd(),
            0,
        ) as *mut u8
    };

    if ptr.is_null() || (ptr as isize) < 0 || len == 0 {
        Err(Box::new(std::io::Error::new(ErrorKind::Other, "pci mapping failed")))
    } else {
        Ok((ptr, len))
    }
}

pub fn unbind_driver(pci_addr: &str) -> Result<(), Box<Error>> {
    let path = format!("/sys/bus/pci/devices/{}/driver/unbind", pci_addr);

    match fs::OpenOptions::new().write(true).open(path) {
        Ok(mut f) => Ok(write!(f, "{}", pci_addr)?),
        Err(ref e) if e.kind() == std::io::ErrorKind::NotFound => Ok(()),
        Err(e) => Err(Box::new(e)),
    }
}