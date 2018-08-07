use std;
use std::fs;
use std::fs::File;
use std::error::Error;
use std::io::ErrorKind;
use std::io::Write;
use std::io::Seek;
use std::io::SeekFrom;

use byteorder::ReadBytesExt;
use byteorder::WriteBytesExt;
use byteorder::NativeEndian;

use std::os::unix::prelude::AsRawFd;
use std::ptr;

use libc;

/// Unbinds the driver
///
/// # Examples
///
/// ```
/// use ixy::pci;
///
/// let result = pci::unbind_driver("abc");
///
/// assert!(result.is_ok());
/// ```
pub fn unbind_driver(pci_addr: &str) -> Result<(), Box<Error>> {
    let path = format!("/sys/bus/pci/devices/{}/driver/unbind", pci_addr);

    match fs::OpenOptions::new().write(true).open(path) {
        Ok(mut f) => Ok(write!(f, "{}", pci_addr)?),
        Err(ref e) if e.kind() == std::io::ErrorKind::NotFound => Ok(()),
        Err(e) => Err(Box::new(e)),
    }
}

pub fn enable_dma(pci_addr: &str) -> Result<(), Box<Error>> {
    let path = format!("/sys/bus/pci/devices/{}/config", pci_addr);
    let mut file = fs::OpenOptions::new()
        .read(true)
        .write(true)
        .open(&path)?;

    assert_eq!(file.seek(SeekFrom::Start(4))?, 4);
    let mut dma = file.read_u16::<NativeEndian>()?;

    dma |= 1 << 2;

    assert_eq!(file.seek(SeekFrom::Start(4))?, 4);
    file.write_u16::<NativeEndian>(dma)?;

    Ok(())
}

pub fn pci_map_resource(pci_addr: &str) -> Result<(*mut u8, usize), Box<Error>> {
    let path = format!("/sys/bus/pci/devices/{}/resource0", pci_addr);

    unbind_driver(pci_addr)?;
    println!("after unbind");
    enable_dma(pci_addr)?;

    let file = fs::OpenOptions::new()
        .read(true)
        .write(true)
        .open(&path)?;
    let len = fs::metadata(&path)?.len() as usize;

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

pub fn pci_open_resource(pci_addr: &str, resource: &str) -> Result<File, Box<Error>> {
    let path = format!("/sys/bus/pci/devices/{}/{}", pci_addr, resource);

    Ok(File::open(path)?)
}