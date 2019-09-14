use std::error::Error;
use std::fs;
use std::fs::{File, OpenOptions};
use std::mem;
use std::os::unix::io::{AsRawFd, IntoRawFd, RawFd};
use std::ptr;

use crate::memory::{get_vfio_container, set_vfio_container};
use crate::pci::{BUS_MASTER_ENABLE_BIT, COMMAND_REGISTER_OFFSET};

// constants needed for IOMMU. Grabbed from linux/vfio.h
pub const VFIO_GET_API_VERSION: u64 = 15204;
pub const VFIO_CHECK_EXTENSION: u64 = 15205;
pub const VFIO_SET_IOMMU: u64 = 15206;
pub const VFIO_GROUP_GET_STATUS: u64 = 15207;
pub const VFIO_GROUP_SET_CONTAINER: u64 = 15208;
pub const VFIO_GROUP_GET_DEVICE_FD: u64 = 15210;
pub const VFIO_DEVICE_GET_REGION_INFO: u64 = 15212;

pub const VFIO_API_VERSION: i32 = 0;
pub const VFIO_TYPE1_IOMMU: u64 = 1;
pub const VFIO_GROUP_FLAGS_VIABLE: u32 = 1;
pub const VFIO_PCI_CONFIG_REGION_INDEX: u32 = 7;
pub const VFIO_PCI_BAR0_REGION_INDEX: u32 = 0;

const VFIO_DMA_MAP_FLAG_READ: u32 = 1;
const VFIO_DMA_MAP_FLAG_WRITE: u32 = 2;
const VFIO_IOMMU_MAP_DMA: u64 = 15217;

/// struct vfio_iommu_type1_dma_map, grabbed from linux/vfio.h
#[allow(non_camel_case_types)]
#[repr(C)]
struct vfio_iommu_type1_dma_map {
    argsz: u32,
    flags: u32,
    vaddr: *mut u8,
    iova: *mut u8,
    size: usize,
}

/// struct vfio_group_status, grabbed from linux/vfio.h
#[allow(non_camel_case_types)]
#[repr(C)]
struct vfio_group_status {
    argsz: u32,
    flags: u32,
}

/// struct vfio_region_info, grabbed from linux/vfio.h
#[allow(non_camel_case_types)]
#[repr(C)]
struct vfio_region_info {
    argsz: u32,
    flags: u32,
    index: u32,
    cap_offset: u32,
    size: u64,
    offset: u64,
}

/// Initializes the IOMMU for a given PCI device. The device must be bound to the VFIO driver.
pub fn vfio_init(pci_addr: &str) -> Result<RawFd, Box<dyn Error>> {
    let dfd: RawFd;
    let group_file: File;
    let gfd: RawFd;

    // we also have to build this vfio struct...
    let mut group_status: vfio_group_status = vfio_group_status {
        argsz: mem::size_of::<vfio_group_status>() as u32,
        flags: 0,
    };

    // need to set up the container exactly once
    let mut first_time_setup = false;
    let mut cfd = get_vfio_container();

    if cfd == -1 {
        first_time_setup = true;
        // open vfio file to create new vfio container
        let container_file = OpenOptions::new()
            .read(true)
            .write(true)
            .open("/dev/vfio/vfio")
            .unwrap();
        cfd = container_file.into_raw_fd();
        set_vfio_container(cfd);

        // check if the container's API version is the same as the VFIO API's
        if unsafe { libc::ioctl(cfd, VFIO_GET_API_VERSION) } != VFIO_API_VERSION {
            return Err("unknown VFIO API Version".into());
        }

        // check if type1 is supported
        if unsafe { libc::ioctl(cfd, VFIO_CHECK_EXTENSION, VFIO_TYPE1_IOMMU) } != 1 {
            return Err("container doesn't support Type1 IOMMU".into());
        }
    }

    // find vfio group for device
    let link = fs::read_link(format!("/sys/bus/pci/devices/{}/iommu_group", pci_addr)).unwrap();
    let group = link
        .file_name()
        .unwrap()
        .to_str()
        .unwrap()
        .parse::<i32>()
        .unwrap();

    // open the devices' group
    group_file = OpenOptions::new()
        .read(true)
        .write(true)
        .open(format!("/dev/vfio/{}", group))
        .unwrap();
    gfd = group_file.as_raw_fd();

    // Test the group is viable and available
    if unsafe { libc::ioctl(gfd, VFIO_GROUP_GET_STATUS, &mut group_status) } == -1 {
        return Err(
            format!("failed to VFIO_GROUP_GET_STATUS. Errno: {}", unsafe {
                *libc::__errno_location()
            })
            .into(),
        );
    }
    if (group_status.flags & VFIO_GROUP_FLAGS_VIABLE) != 1 {
        return Err(
            "group is not viable (ie, not all devices in this group are bound to vfio)".into(),
        );
    }

    // Add the group to the container
    if unsafe { libc::ioctl(gfd, VFIO_GROUP_SET_CONTAINER, &cfd) } == -1 {
        return Err(
            format!("failed to VFIO_GROUP_SET_CONTAINER. Errno: {}", unsafe {
                *libc::__errno_location()
            })
            .into(),
        );
    }

    if first_time_setup {
        // Enable the IOMMU model we want
        if unsafe { libc::ioctl(cfd, VFIO_SET_IOMMU, VFIO_TYPE1_IOMMU) } == -1 {
            return Err(format!(
                "failed to VFIO_SET_IOMMU to VFIO_TYPE1_IOMMU. Errno: {}",
                unsafe { *libc::__errno_location() }
            )
            .into());
        }
    }

    // Get a file descriptor for the device
    dfd = unsafe { libc::ioctl(gfd, VFIO_GROUP_GET_DEVICE_FD, pci_addr) };
    if dfd == -1 {
        return Err(
            format!("failed to VFIO_GROUP_GET_DEVICE_FD. Errno: {}", unsafe {
                *libc::__errno_location()
            })
            .into(),
        );
    }

    vfio_enable_dma(dfd)?;

    Ok(dfd)
}

/// Enables DMA Bit for VFIO devices
pub fn vfio_enable_dma(device_file_descriptor: RawFd) -> Result<(), Box<dyn Error>> {
    // Get region info for config region
    let mut conf_reg: vfio_region_info = vfio_region_info {
        argsz: mem::size_of::<vfio_region_info>() as u32,
        flags: 0,
        index: VFIO_PCI_CONFIG_REGION_INDEX,
        cap_offset: 0,
        size: 0,
        offset: 0,
    };
    if unsafe {
        libc::ioctl(
            device_file_descriptor,
            VFIO_DEVICE_GET_REGION_INFO,
            &mut conf_reg,
        )
    } == -1
    {
        return Err(format!(
            "failed to VFIO_DEVICE_GET_REGION_INFO for index VFIO_PCI_CONFIG_REGION_INDEX. Errno: {}",
            unsafe { *libc::__errno_location() }
        ).into());
    }

    let mut dma: u16 = 0;
    if unsafe {
        libc::pread(
            device_file_descriptor,
            &mut dma as *mut _ as *mut libc::c_void,
            2,
            (conf_reg.offset + COMMAND_REGISTER_OFFSET) as i64,
        )
    } == -1
    {
        return Err(format!("failed to pread DMA bit. Errno: {}", unsafe {
            *libc::__errno_location()
        })
        .into());
    }

    dma |= 1 << BUS_MASTER_ENABLE_BIT;

    if unsafe {
        libc::pwrite(
            device_file_descriptor,
            &mut dma as *mut _ as *mut libc::c_void,
            2,
            (conf_reg.offset + COMMAND_REGISTER_OFFSET) as i64,
        )
    } == -1
    {
        return Err(format!("failed to pwrite DMA bit. Errno: {}", unsafe {
            *libc::__errno_location()
        })
        .into());
    }
    Ok(())
}

/// Mmaps a VFIO resource and returns a pointer to the mapped memory.
pub fn vfio_map_region(fd: RawFd, index: u32) -> Result<(*mut u8, usize), Box<dyn Error>> {
    let mut region_info: vfio_region_info = vfio_region_info {
        argsz: mem::size_of::<vfio_region_info>() as u32,
        flags: 0,
        index,
        cap_offset: 0,
        size: 0,
        offset: 0,
    };
    if unsafe { libc::ioctl(fd, VFIO_DEVICE_GET_REGION_INFO, &mut region_info) } == -1 {
        return Err(
            format!("failed to VFIO_DEVICE_GET_REGION_INFO. Errno: {}", unsafe {
                *libc::__errno_location()
            })
            .into(),
        );
    }

    let len = region_info.size as usize;

    let ptr = unsafe {
        libc::mmap(
            ptr::null_mut(),
            len,
            libc::PROT_READ | libc::PROT_WRITE,
            libc::MAP_SHARED,
            fd,
            region_info.offset as i64,
        )
    };
    if ptr == libc::MAP_FAILED {
        return Err(format!("failed to mmap region. Errno: {}", unsafe {
            *libc::__errno_location()
        })
        .into());
    }
    let addr = ptr as *mut u8;

    Ok((addr, len))
}

pub fn vfio_map_dma(ptr: usize, size: usize) -> Result<usize, Box<dyn Error>> {
    let mut iommu_dma_map: vfio_iommu_type1_dma_map = vfio_iommu_type1_dma_map {
        argsz: mem::size_of::<vfio_iommu_type1_dma_map>() as u32,
        vaddr: ptr as *mut u8,
        size,
        iova: ptr as *mut u8,
        flags: VFIO_DMA_MAP_FLAG_READ | VFIO_DMA_MAP_FLAG_WRITE,
    };

    let ioctl_result =
        unsafe { libc::ioctl(get_vfio_container(), VFIO_IOMMU_MAP_DMA, &mut iommu_dma_map) };
    if ioctl_result != -1 {
        Ok(iommu_dma_map.iova as usize)
    } else {
        Err("failed to map the DMA memory - ulimit set for this user?".into())
    }
}
