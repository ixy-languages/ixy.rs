use std::mem;
use std::os::unix::io::RawFd;
use epoll::Event;
use std::time::Instant;
use libc;
use std::error::Error;

const MOVING_AVERAGE_RANGE: usize = 5;
const INTERRUPT_THRESHOLD: f64 = 1.2;
pub const INTERRUPT_INITIAL_INTERVAL: u64 = 1_000_000_000;
const MAX_INTERRUPT_VECTORS: u32 = 32;

#[derive(Default)]
pub struct Interrupts {
    pub interrupts_enabled: bool, // Whether interrupts for this device are enabled or disabled.
    pub itr_rate: u32, // The Interrupt Throttling Rate
    pub interrupt_type: u64, // MSI or MSIX
    pub timeout_ms: i16, // interrupt timeout in milliseconds (-1 to disable the timeout)
    pub queues: Vec<InterruptsQueue>,  // Interrupt settings per queue
}

#[derive(Copy, Clone)]
pub struct InterruptsQueue {
    pub vfio_event_fd: RawFd, // event fd
    pub vfio_epoll_fd: RawFd, // epoll fd
    pub interrupt_enabled: bool, // Whether interrupt for this queue is enabled or not
    pub last_time_checked: Instant, // Last time the interrupt flag was checked
    pub rx_pkts: u64, // The number of received packets since the last check
    pub interval: u64, // The interval to check the interrupt flag
    pub moving_avg: InterruptMovingAvg, // The moving average of the hybrid interrupt
}

#[derive(Default)]
#[derive(Copy, Clone)]
pub struct InterruptMovingAvg {
    pub index: usize, // The current index
    pub length: usize, // The moving average length
    pub sum: f64, // The moving average sum
    pub measured_rates: [f64; MOVING_AVERAGE_RANGE], // The moving average window
}

/// constants and structs needed for IOMMU. Grabbed from linux/vfio.h
/// as this is a dynamically sized struct (has an array at the end) we need to use
/// Dynamically Sized Types (DSTs) which can be found at
/// https://doc.rust-lang.org/nomicon/exotic-sizes.html#dynamically-sized-types-dsts
#[allow(non_camel_case_types)]
#[repr(C)]
struct vfio_irq_set<T: ?Sized> {
    argsz: u32,
    flags: u32,
    index: u32,
    start: u32,
    count: u32,
    data: T,
}

#[allow(non_camel_case_types)]
#[repr(C)]
struct vfio_irq_info {
    argsz: u32,
    flags: u32,
    index: u32,		/* IRQ index */
    count: u32,		/* Number of IRQs within this index */
}

const VFIO_IRQ_SET_DATA_NONE: u32 = (1 << 0); /* Data not present */
const VFIO_IRQ_SET_DATA_EVENTFD: u32 = (1 << 2); /* Data is eventfd (s32) */
const VFIO_IRQ_SET_ACTION_TRIGGER: u32 = (1 << 5); /* Trigger interrupt */
const VFIO_DEVICE_GET_IRQ_INFO: u64 = 15213;
const VFIO_DEVICE_SET_IRQS: u64 = 15214;
pub const VFIO_PCI_MSI_IRQ_INDEX: u64 = 1;
pub const VFIO_PCI_MSIX_IRQ_INDEX: u64 = 2;
const VFIO_IRQ_INFO_EVENTFD: u32 = (1 << 0);

impl Interrupts {
    /// Setup VFIO interrupts by checking the `device_fd` for which interrupts this device supports.
    /// Returns the supported interrupt type.
    pub fn vfio_setup_interrupt(&mut self, device_fd: RawFd) -> Result<(), Box<dyn Error>> {
        for index in (0..(VFIO_PCI_MSIX_IRQ_INDEX + 1)).rev() {
            let irq_info: vfio_irq_info = vfio_irq_info {
                argsz: mem::size_of::<vfio_irq_info>() as u32,
                index: index as u32,
                flags: 0,
                count: 0,
            };

            if unsafe { libc::ioctl(device_fd, VFIO_DEVICE_GET_IRQ_INFO, &irq_info) } == -1 {
                return Err(format!(
                    "failed to VFIO_DEVICE_GET_IRQ_INFO for index {}. Errno: {}", index,
                    unsafe { *libc::__errno_location() }
                ).into());
            }

            if (irq_info.flags & VFIO_IRQ_INFO_EVENTFD) == 0 {
                continue;
            }

            self.interrupt_type = index;
            return Ok(());
        }

        self.interrupt_type = 0;
        Ok(())
    }
}

impl InterruptsQueue {

    /// Add the `event_fd` file descriptor to epoll.
    pub fn vfio_epoll_ctl(&mut self, event_fd: RawFd) -> Result<(), Box<dyn Error>> {
        let event: Event = Event {
            events: libc::EPOLLIN as u32,
            data: event_fd as u64
        };

        let epoll_fd = epoll::create(false)?;

        epoll::ctl(epoll_fd, epoll::ControlOptions::EPOLL_CTL_ADD, event_fd, event)?;

        self.vfio_epoll_fd = epoll_fd;
        Ok(())
    }

    /// Waits for events on the epoll instance referred to by the file descriptor `epoll_fd`.
    /// The memory area pointed to by events will contain the events that will be available for the caller.
    /// Up to `maxevents` are returned by epoll_wait. The `maxevents` argument must be greater than zero.
    /// The `timeout` argument specifies the minimum number of milliseconds that epoll_wait will block.
    /// Specifying a `timeout` of -1 causes epoll_wait to block indefinitely,
    /// while specifying a `timeout` equal to zero cause epoll_wait to return immediately, even if no events are available.
    /// Returns the number of ready file descriptors.
    pub fn vfio_epoll_wait(&self, maxevents: usize, timeout: i32)  -> Result<usize, Box<dyn Error>> {
        let mut events = [Event; 10];
        let rc: usize;

        loop {
            // info("Waiting for packets...");
            rc = epoll::wait(self.vfio_epoll_fd, timeout, &mut events)?;
            if rc > 0 {
                /* epoll_wait has at least one fd ready to read */
                for i in 0..rc {
                    let mut val: u16 = 0;
                    let val_ptr: *mut u16 = &mut val;
                    // read event file descriptor to clear interrupt.
                    if unsafe {
                        libc::read(events[i].data as i32, val_ptr as *mut libc::c_void, mem::size_of::<u64>())
                    } == -1
                    {
                        return Err(format!("failed to read event. Errno: {}", unsafe {
                            *libc::__errno_location()
                        }).into());
                    }
                }
                break;
            } else {
                /* rc == 0, epoll_wait timed out */
                break;
            }
        }

        Ok(rc)
    }

    /// Enable VFIO MSI interrupts for the given `device_fd`.
    pub fn vfio_enable_msi(&mut self, device_fd: RawFd) -> Result<(), Box<dyn Error>> {
        // setup event fd
        let event_fd: RawFd = unsafe { libc::eventfd(0, 0) };

        if event_fd == -1 {
            return Err(format!(
                "failed to create eventfd. Errno: {}",
                unsafe { *libc::__errno_location() }
            ).into());
        }

        let irq_set: vfio_irq_set<[u8; 1]> = vfio_irq_set {
            argsz: (mem::size_of::<vfio_irq_set<[u8; 1]>>() + mem::size_of::<RawFd>()) as u32,
            count: 1,
            flags: VFIO_IRQ_SET_DATA_EVENTFD | VFIO_IRQ_SET_ACTION_TRIGGER,
            index: VFIO_PCI_MSI_IRQ_INDEX as u32,
            start: 0,
            data: [event_fd as u8; 1],
        };

        if unsafe { libc::ioctl(device_fd, VFIO_DEVICE_SET_IRQS, &irq_set) } == -1 {
            return Err(format!(
                "failed to VFIO_DEVICE_SET_IRQS. Errno: {}",
                unsafe { *libc::__errno_location() }
            ).into());
        }

        self.vfio_event_fd = event_fd;
        Ok(())
    }

    /// Disable VFIO MSI interrupts for the given `device_fd`.
    pub fn vfio_disable_msi(&mut self, device_fd: RawFd) -> Result<(), Box<dyn Error>> {
        let irq_set: vfio_irq_set<[u8; 0]> = vfio_irq_set {
            argsz: (mem::size_of::<vfio_irq_set<[u8; 0]>>() + mem::size_of::<RawFd>()) as u32,
            count: 0,
            flags: VFIO_IRQ_SET_DATA_NONE | VFIO_IRQ_SET_ACTION_TRIGGER,
            index: VFIO_PCI_MSI_IRQ_INDEX as u32,
            start: 0,
            data: [0; 0],
        };

        if unsafe { libc::ioctl(device_fd, VFIO_DEVICE_SET_IRQS, irq_set) } == -1 {
            return Err(format!(
                "failed to VFIO_DEVICE_SET_IRQS. Errno: {}",
                unsafe { *libc::__errno_location() }
            ).into());
        }

        self.vfio_event_fd = 0;
        Ok(())
    }

    /// Enable VFIO MSI-X interrupts for the given `device_fd`.
    /// The `interrupt_vector` specifies the number of queues to watch.
    pub fn vfio_enable_msix(&mut self, device_fd: RawFd, mut interrupt_vector: u32) -> Result<(), Box<dyn Error>> {
        // setup event fd
        let event_fd: RawFd = unsafe { libc::eventfd(0, 0) };

        if event_fd == -1 {
            return Err(format!(
                "failed to create eventfd. Errno: {}",
                unsafe { *libc::__errno_location() }
            ).into());
        }

        if interrupt_vector == 0 {
            interrupt_vector = 1;
        } else if interrupt_vector > MAX_INTERRUPT_VECTORS {
            interrupt_vector = MAX_INTERRUPT_VECTORS + 1;
        }

        let irq_set: vfio_irq_set<[u8; 1]> = vfio_irq_set {
            argsz: (mem::size_of::<vfio_irq_set<[u8; 1]>>() + mem::size_of::<RawFd>() * (MAX_INTERRUPT_VECTORS + 1) as usize) as u32,
            count: interrupt_vector,
            flags: VFIO_IRQ_SET_DATA_EVENTFD | VFIO_IRQ_SET_ACTION_TRIGGER,
            index: VFIO_PCI_MSIX_IRQ_INDEX as u32,
            start: 0,
            data: [event_fd as u8; 1],
        };

        if unsafe { libc::ioctl(device_fd, VFIO_DEVICE_SET_IRQS, &irq_set) } == -1 {
            return Err(format!(
                "failed to VFIO_DEVICE_SET_IRQS. Errno: {}",
                unsafe { *libc::__errno_location() }
            ).into());
        }

        self.vfio_event_fd = event_fd;
        Ok(())
    }

    /// Disable VFIO MSI-X interrupts for the given `device_fd`.
    pub fn vfio_disable_msix(&mut self, device_fd: RawFd) -> Result<(), Box<dyn Error>> {
        let irq_set: vfio_irq_set<[u8; 0]> = vfio_irq_set {
            argsz: (mem::size_of::<vfio_irq_set<[u8; 0]>>() + mem::size_of::<RawFd>()) as u32,
            count: 0,
            flags: VFIO_IRQ_SET_DATA_NONE | VFIO_IRQ_SET_ACTION_TRIGGER,
            index: VFIO_PCI_MSI_IRQ_INDEX as u32,
            start: 0,
            data: [0; 0],
        };

        if unsafe { libc::ioctl(device_fd, VFIO_DEVICE_SET_IRQS, irq_set) } == -1 {
            return Err(format!(
                "failed to VFIO_DEVICE_SET_IRQS. Errno: {}",
                unsafe { *libc::__errno_location() }
            ).into());
        }

        self.vfio_event_fd = 0;
        return Ok(());
    }

    /// Calculate packets per second based on the received number of packets and the
    /// elapsed time in `nanos` since the last calculation.
    /// Returns the number of packets per second.
    pub fn mpps(&self, nanos: u64) -> f64 {
        self.rx_pkts as f64 / 1_000_000.0 / (nanos as f64 / 1_000_000_000.0)
    }

    /// Check if interrupts or polling should be used based on the current number of received packets per seconds.
    /// The `diff` specifies time elapsed since the last call in nanoseconds.
    /// The `buf_index` and `buf_size` the current buffer index and the size of the receive buffer.
    pub fn check_interrupt(&mut self, diff: u64, buf_index: usize, buf_size: usize) {
        self.moving_avg.sum -= self.moving_avg.measured_rates[self.moving_avg.index];
        self.moving_avg.measured_rates[self.moving_avg.index] = self.mpps(diff);
        self.moving_avg.sum += self.moving_avg.measured_rates[self.moving_avg.index];
        if self.moving_avg.length < MOVING_AVERAGE_RANGE {
            self.moving_avg.length -= 1;
        }
        self.moving_avg.index = (self.moving_avg.index + 1) % MOVING_AVERAGE_RANGE;
        self.moving_avg.length += 1;
        self.rx_pkts = 0;
        let average = self.moving_avg.sum / self.moving_avg.length as f64;
        if average > INTERRUPT_THRESHOLD {
            self.interrupt_enabled = false;
        } else if buf_index == buf_size {
            self.interrupt_enabled = false;
        } else {
            self.interrupt_enabled = true;
        }
        self.last_time_checked = Instant::now();
    }
}