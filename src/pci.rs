use std::error::Error;
use std::fs::{self, File, OpenOptions};
use std::io::{self, Read, Seek, SeekFrom, Write};
use std::os::unix::prelude::AsRawFd;
use std::ptr;

use byteorder::{NativeEndian, ReadBytesExt, WriteBytesExt};

// write to the command register (offset 4) in the PCIe config space
pub const COMMAND_REGISTER_OFFSET: u64 = 4;
// bit 2 is "bus master enable", see PCIe 3.0 specification section 7.5.1.1
pub const BUS_MASTER_ENABLE_BIT: u64 = 2;

/// Unbinds the driver from the device at `pci_addr`.
pub fn unbind_driver(pci_addr: &str) -> Result<(), Box<dyn Error>> {
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
pub fn enable_dma(pci_addr: &str) -> Result<(), Box<dyn Error>> {
    let path = format!("/sys/bus/pci/devices/{}/config", pci_addr);
    let mut file = fs::OpenOptions::new().read(true).write(true).open(&path)?;

    let mut dma = read_io16(&mut file, COMMAND_REGISTER_OFFSET)?;
    dma |= 1 << BUS_MASTER_ENABLE_BIT;
    write_io16(&mut file, dma, COMMAND_REGISTER_OFFSET)?;

    Ok(())
}

/// Mmaps a pci resource and returns a pointer to the mapped memory.
pub fn pci_map_resource(pci_addr: &str) -> Result<(*mut u8, usize), Box<dyn Error>> {
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
pub fn pci_open_resource(pci_addr: &str, resource: &str) -> Result<File, Box<dyn Error>> {
    let path = format!("/sys/bus/pci/devices/{}/{}", pci_addr, resource);
    Ok(OpenOptions::new().read(true).write(true).open(path)?)
}

/// Opens a pci resource file at the given address in read-only mode.
pub fn pci_open_resource_ro(pci_addr: &str, resource: &str) -> Result<File, Box<dyn Error>> {
    let path = format!("/sys/bus/pci/devices/{}/{}", pci_addr, resource);
    Ok(OpenOptions::new().read(true).write(false).open(path)?)
}

/// Reads and returns an u8 at `offset` in `file`.
pub fn read_io8(file: &mut File, offset: u64) -> Result<u8, io::Error> {
    file.seek(SeekFrom::Start(offset))?;
    file.read_u8()
}

/// Reads and returns an u16 at `offset` in `file`.
pub fn read_io16(file: &mut File, offset: u64) -> Result<u16, io::Error> {
    file.seek(SeekFrom::Start(offset))?;
    file.read_u16::<NativeEndian>()
}

/// Reads and returns an u32 at `offset` in `file`.
pub fn read_io32(file: &mut File, offset: u64) -> Result<u32, io::Error> {
    file.seek(SeekFrom::Start(offset))?;
    file.read_u32::<NativeEndian>()
}

/// Writes an u8 at `offset` in `file`.
pub fn write_io8(file: &mut File, value: u8, offset: u64) -> Result<(), io::Error> {
    file.seek(SeekFrom::Start(offset))?;
    file.write_u8(value)
}

/// Writes an u16 at `offset` in `file`.
pub fn write_io16(file: &mut File, value: u16, offset: u64) -> Result<(), io::Error> {
    file.seek(SeekFrom::Start(offset))?;
    file.write_u16::<NativeEndian>(value)
}

/// Writes an u32 at `offset` in `file`.
pub fn write_io32(file: &mut File, value: u32, offset: u64) -> Result<(), io::Error> {
    file.seek(SeekFrom::Start(offset))?;
    file.write_u32::<NativeEndian>(value)
}

/// Reads a hex string from `file` and returns it as `u64`.
pub fn read_hex(file: &mut File) -> Result<u64, Box<dyn Error>> {
    let mut buffer = String::new();
    file.read_to_string(&mut buffer)?;

    Ok(u64::from_str_radix(
        &buffer.trim().trim_start_matches("0x"),
        16,
    )?)
}
