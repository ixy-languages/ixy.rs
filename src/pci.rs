use std::error::Error;
use std::fs::{self, File};
use std::io::{self, Seek, SeekFrom, Write};
use std::os::unix::prelude::AsRawFd;
use std::ptr;

use byteorder::{NativeEndian, ReadBytesExt, WriteBytesExt};

use libc;

/* write to the command register (offset 4) in the PCIe config space */
pub const COMMAND_REGISTER_OFFSET: u64 = 4;
/* bit 2 is "bus master enable", see PCIe 3.0 specification section 7.5.1.1 */
pub const BUS_MASTER_ENABLE_BIT: u64 = 2;

/// Unbinds the driver from the device at `pci_addr`.
pub fn unbind_driver(pci_addr: &str) -> Result<(), Box<Error>> {
    let path = format!("/sys/bus/pci/devices/{}/driver/unbind", pci_addr);

    match fs::OpenOptions::new().write(true).open(path) {
        Ok(mut f) => {
            write!(f, "{}", pci_addr)?;
            Ok(())
        }
        Err(ref e) if e.kind() == io::ErrorKind::NotFound => Ok(()),
        Err(e) => Err(Box::new(e)),
    }
}

/// Enables direct memory access for the device at `pci_addr`.
pub fn enable_dma(pci_addr: &str) -> Result<(), Box<Error>> {
    let path = format!("/sys/bus/pci/devices/{}/config", pci_addr);
    let mut file = fs::OpenOptions::new().read(true).write(true).open(&path)?;

    assert_eq!(
        file.seek(SeekFrom::Start(COMMAND_REGISTER_OFFSET))?,
        COMMAND_REGISTER_OFFSET
    );
    let mut dma = file.read_u16::<NativeEndian>()?;

    dma |= 1 << BUS_MASTER_ENABLE_BIT;

    assert_eq!(
        file.seek(SeekFrom::Start(COMMAND_REGISTER_OFFSET))?,
        COMMAND_REGISTER_OFFSET
    );
    file.write_u16::<NativeEndian>(dma)?;

    Ok(())
}

/// Mmaps a pci resource and returns a pointer to the mapped memory.
pub fn pci_map_resource(pci_addr: &str) -> Result<(*mut u8, usize), Box<Error>> {
    let path = format!("/sys/bus/pci/devices/{}/resource0", pci_addr);

    unbind_driver(pci_addr)?;
    enable_dma(pci_addr)?;

    let file = fs::OpenOptions::new().read(true).write(true).open(&path)?;
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

    if ptr.is_null() || len == 0 {
        Err("pci mapping failed".into())
    } else {
        Ok((ptr, len))
    }
}

/// Opens a pci resource file at the given address.
pub fn pci_open_resource(pci_addr: &str, resource: &str) -> Result<File, Box<Error>> {
    let path = format!("/sys/bus/pci/devices/{}/{}", pci_addr, resource);
    Ok(File::open(path)?)
}

/// Reads and returns an u16 at `offset` in `file`.
pub fn read_io16(file: &mut File, offset: usize) -> Result<u16, Box<Error>> {
    file.seek(SeekFrom::Start(offset as u64))?;
    Ok(file.read_u16::<NativeEndian>()?)
}

/// Reads and returns an u32 at `offset` in `file`.
pub fn read_io32(file: &mut File, offset: usize) -> Result<u32, Box<Error>> {
    file.seek(SeekFrom::Start(offset as u64))?;
    Ok(file.read_u32::<NativeEndian>()?)
}
