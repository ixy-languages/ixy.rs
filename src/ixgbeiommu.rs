/* actually, the dma assignment is used, but in unsafe code, so it is not
 * recognized */
#![allow(unused_assignments)]

use std::collections::VecDeque;
use std::error::Error;
use std::fs;
use std::fs::{File, OpenOptions};
use std::mem;
use std::os::unix::io::{AsRawFd, RawFd};
use std::ptr;

use crate::ixgbe::IxgbeDevice;
use crate::memory::*;
use crate::pci::*;

use crate::DeviceStats;
use crate::IxyDevice;
use crate::MAX_QUEUES;
use libc;

const DRIVER_NAME: &str = "ixy-ixgbe-iommu";
const DRIVER_IOMMU: bool = true;

/* constants needed for IOMMU. Grabbed from linux/vfio.h */
const VFIO_GET_API_VERSION: u64 = 15204;
const VFIO_CHECK_EXTENSION: u64 = 15205;
const VFIO_SET_IOMMU: u64 = 15206;
const VFIO_GROUP_GET_STATUS: u64 = 15207;
const VFIO_GROUP_SET_CONTAINER: u64 = 15208;
const VFIO_GROUP_GET_DEVICE_FD: u64 = 15210;
const VFIO_DEVICE_GET_REGION_INFO: u64 = 15212;

const VFIO_API_VERSION: i32 = 0;
const VFIO_TYPE1_IOMMU: u64 = 1;
const VFIO_GROUP_FLAGS_VIABLE: u32 = 1;
const VFIO_PCI_CONFIG_REGION_INDEX: u32 = 7;
const VFIO_PCI_BAR0_REGION_INDEX: u32 = 0;

static mut CONTAINER_FILE: Option<File> = None;
static mut CFD: RawFd = 0;

/* struct vfio_group_status, grabbed from linux/vfio.h */
#[allow(non_camel_case_types)]
#[repr(C)]
struct vfio_group_status {
    argsz: u32,
    flags: u32,
}

/* struct vfio_region_info, grabbed from linux/vfio.h */
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

pub struct IxgbeIommuDevice {
    dev: IxgbeDevice,
}

impl IxyDevice for IxgbeIommuDevice {
    /// Returns an initialized `IxgbeDevice` on success.
    ///
    /// # Panics
    /// Panics if `num_rx_queues` or `num_tx_queues` exceeds `MAX_QUEUES`.
    fn init(
        pci_addr: &str,
        num_rx_queues: u16,
        num_tx_queues: u16,
    ) -> Result<IxgbeIommuDevice, Box<Error>> {
        if unsafe { libc::getuid() } != 0 {
            warn!("not running as root, this will probably fail");
        }

        assert!(
            num_rx_queues <= MAX_QUEUES,
            "cannot configure {} rx queues: limit is {}",
            num_rx_queues,
            MAX_QUEUES
        );
        assert!(
            num_tx_queues <= MAX_QUEUES,
            "cannot configure {} tx queues: limit is {}",
            num_tx_queues,
            MAX_QUEUES
        );

        let dfd: RawFd;
        let group_file: Option<File>;
        let gfd: RawFd;
        let addr: *mut u8;
        let len: usize;
        let mut cfd: RawFd = unsafe { CFD };
        /* we also have to build this vfio struct... */
        let group_status: vfio_group_status = vfio_group_status {
            argsz: mem::size_of::<vfio_group_status> as u32,
            flags: 0,
        };

        let mut first_time_setup = false;

        /* if the VFIO container is not initialized yet... */
        if unsafe { CONTAINER_FILE.is_none() } {
            /* ...initialize it */
            first_time_setup = true;
            /* Open new VFIO Container */
            unsafe {
                CONTAINER_FILE = Some(
                    OpenOptions::new()
                        .read(true)
                        .write(true)
                        .open("/dev/vfio/vfio")?,
                );
            }
            unsafe { CFD = get_raw_fd(&CONTAINER_FILE) };
            cfd = unsafe { CFD };
            /* check IOMMU API version */
            if unsafe { libc::ioctl(cfd, VFIO_GET_API_VERSION) } != VFIO_API_VERSION {
                info!("Unknown VFIO API Version. Application will probably die soon(ish).");
            }

            /* check if device supports Type1 IOMMU */
            if unsafe { libc::ioctl(cfd, VFIO_CHECK_EXTENSION, VFIO_TYPE1_IOMMU) } != 1 {
                info!("Device doesn't support Type1 IOMMU. Application will probably crash soon(ish).");
            }
        }

        /* find vfio group for device */
        let link = fs::read_link(format!("/sys/bus/pci/devices/{}/iommu_group", pci_addr))?;
        let group = link
            .file_name()
            .unwrap()
            .to_str()
            .unwrap()
            .parse::<i32>()
            .unwrap();

        /* open the devices' group */
        group_file = Some(
            OpenOptions::new()
                .read(true)
                .write(true)
                .open(format!("/dev/vfio/{}", group))?,
        );
        gfd = get_raw_fd(&group_file);

        /* Test the group is viable and available */
        if unsafe { libc::ioctl(gfd, VFIO_GROUP_GET_STATUS, &group_status) } == -1 {
            error!(
                "[ERROR]Could not VFIO_GROUP_GET_STATUS. Errno: {}",
                unsafe { *libc::__errno_location() }
            );
        }
        if (group_status.flags & VFIO_GROUP_FLAGS_VIABLE) != 1 {
            info!("Group is not viable (ie, not all devices bound for vfio). Application will probably crash soon(ish).");
        }

        /* Add the group to the container */
        if unsafe { libc::ioctl(gfd, VFIO_GROUP_SET_CONTAINER, &cfd) } == -1 {
            error!(
                "[ERROR]Could not VFIO_GROUP_SET_CONTAINER. Errno: {}",
                unsafe { *libc::__errno_location() }
            );
        }

        if first_time_setup {
            /* Enable the IOMMU model we want */
            if unsafe { libc::ioctl(cfd, VFIO_SET_IOMMU, VFIO_TYPE1_IOMMU) } == -1 {
                error!(
                    "[ERROR]Could not VFIO_SET_IOMMU to VFIO_TYPE1_IOMMU. Errno: {}",
                    unsafe { *libc::__errno_location() }
                );
            }
        }

        /* Get a file descriptor for the device */
        dfd = unsafe { libc::ioctl(gfd, VFIO_GROUP_GET_DEVICE_FD, pci_addr) };
        if dfd == -1 {
            error!(
                "[ERROR]Could not VFIO_GROUP_GET_DEVICE_FD. Errno: {}",
                unsafe { *libc::__errno_location() }
            );
        }

        enable_dma(dfd);

        /* map BAR0 space */
        let bar0_reg: vfio_region_info = vfio_region_info {
            argsz: mem::size_of::<vfio_region_info> as u32,
            flags: 0,
            index: VFIO_PCI_BAR0_REGION_INDEX,
            cap_offset: 0,
            size: 0,
            offset: 0,
        };
        if unsafe { libc::ioctl(dfd, VFIO_DEVICE_GET_REGION_INFO, &bar0_reg) } == -1 {
            error!(
                    "[ERROR]Could not VFIO_DEVICE_GET_REGION_INFO for index VFIO_PCI_BAR0_REGION_INDEX. Errno: {}",
                    unsafe { *libc::__errno_location() }
                );
        }

        len = bar0_reg.size as usize;

        let ptr = unsafe {
            libc::mmap(
                ptr::null_mut(),
                len,
                libc::PROT_READ | libc::PROT_WRITE,
                libc::MAP_SHARED,
                dfd,
                bar0_reg.offset as i64,
            )
        };
        if ptr == libc::MAP_FAILED {
            error!("[ERROR]Could not mmap bar0. Errno: {}", unsafe {
                *libc::__errno_location()
            });
        }
        addr = ptr as *mut u8;

        let rx_queues = Vec::with_capacity(num_rx_queues as usize);
        let tx_queues = Vec::with_capacity(num_tx_queues as usize);

        let mut ixgbedev = IxgbeDevice {
            pci_addr: pci_addr.to_string(),
            addr,
            len,
            num_rx_queues,
            num_tx_queues,
            rx_queues,
            tx_queues,
            iommu: true,
            vfio_container: cfd,
        };

        ixgbedev.reset_and_init(pci_addr)?;

        let dev = IxgbeIommuDevice { dev: ixgbedev };

        Ok(dev)
    }

    /// Returns the driver's name of this device.
    fn get_driver_name(&self) -> &str {
        DRIVER_NAME
    }

    /// Returns the driver's iommu capability.
    fn is_card_iommu_capable(&self) -> bool {
        DRIVER_IOMMU
    }

    /// Returns the VFIO container file descriptor.
    /// When implementing non-VFIO / IOMMU devices, just return 0.
    fn get_vfio_container(&self) -> RawFd {
        self.dev.vfio_container
    }

    /// Returns the pci address of this device.
    fn get_pci_addr(&self) -> &str {
        self.dev.get_pci_addr()
    }

    /// Pushes up to `num_packets` received `Packet`s onto `buffer`.
    fn rx_batch(
        &mut self,
        queue_id: u32,
        buffer: &mut VecDeque<Packet>,
        num_packets: usize,
    ) -> usize {
        self.dev.rx_batch(queue_id, buffer, num_packets)
    }

    /// Pops as many packets as possible from `packets` to put them into the device`s tx queue.
    fn tx_batch(&mut self, queue_id: u32, packets: &mut VecDeque<Packet>) -> usize {
        self.dev.tx_batch(queue_id, packets)
    }

    /// Reads the stats of this device into `stats`.
    fn read_stats(&self, stats: &mut DeviceStats) {
        self.dev.read_stats(stats)
    }

    /// Resets the stats of this device.
    fn reset_stats(&self) {
        self.dev.reset_stats()
    }

    /// Returns the link speed of this device.
    fn get_link_speed(&self) -> u16 {
        self.dev.get_link_speed()
    }
}

/// Enables DMA Bit for VFIO devices
fn enable_dma(device_file_descriptor: RawFd) {
    /* Get region info for config region */
    let conf_reg: vfio_region_info = vfio_region_info {
        argsz: mem::size_of::<vfio_region_info> as u32,
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
            &conf_reg,
        )
    } == -1
    {
        error!(
                "[ERROR]Could not VFIO_DEVICE_GET_REGION_INFO for index VFIO_PCI_CONFIG_REGION_INDEX. Errno: {}",
                unsafe { *libc::__errno_location() }
            );
    }

    let mut dma: u16 = 0;
    let dma_ptr: *mut u16 = &mut dma;
    if unsafe {
        libc::pread(
            device_file_descriptor,
            dma_ptr as *mut libc::c_void,
            2,
            (conf_reg.offset + COMMAND_REGISTER_OFFSET) as i64,
        )
    } == -1
    {
        error!("[ERROR]Could not pread. Errno: {}", unsafe {
            *libc::__errno_location()
        });
    }

    dma |= 1 << BUS_MASTER_ENABLE_BIT;

    if unsafe {
        libc::pwrite(
            device_file_descriptor,
            dma_ptr as *mut libc::c_void,
            2,
            (conf_reg.offset + COMMAND_REGISTER_OFFSET) as i64,
        )
    } == -1
    {
        error!("[ERROR]Could not pwrite. Errno: {}", unsafe {
            *libc::__errno_location()
        });
    }
}

fn get_raw_fd(f: &Option<File>) -> RawFd {
    match *f {
        Some(ref x) => x.as_raw_fd(),
        None => -1,
    }
}
