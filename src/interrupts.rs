use crate::vfio::{
    vfio_irq_info, vfio_irq_set, Event, VFIO_DEVICE_GET_IRQ_INFO, VFIO_DEVICE_SET_IRQS,
    VFIO_IRQ_INFO_EVENTFD, VFIO_IRQ_SET_ACTION_TRIGGER, VFIO_IRQ_SET_DATA_EVENTFD,
    VFIO_IRQ_SET_DATA_NONE, VFIO_PCI_MSIX_IRQ_INDEX, VFIO_PCI_MSI_IRQ_INDEX,
};
use std::collections::VecDeque;
use std::error::Error;
use std::mem;
use std::os::unix::io::RawFd;
use std::time::Instant;

const MOVING_AVERAGE_RANGE: usize = 5;
const INTERRUPT_THRESHOLD: u64 = 1_200;
pub const INTERRUPT_INITIAL_INTERVAL: u64 = 1_000_000_000;
const MAX_INTERRUPT_VECTORS: u32 = 32;

#[derive(Default)]
pub struct Interrupts {
    pub interrupts_enabled: bool,     // Interrupts for this device enabled?
    pub itr_rate: u32,                // Interrupt Throttling Rate
    pub interrupt_type: u64,          // MSI or MSIX
    pub timeout_ms: i16,              // Interrupt timeout in ms (-1 to disable timeout)
    pub queues: Vec<InterruptsQueue>, // Interrupt settings per queue
}

pub struct InterruptsQueue {
    pub vfio_event_fd: RawFd,           // event fd
    pub vfio_epoll_fd: RawFd,           // epoll fd
    pub interrupt_enabled: bool,        // Interrupt for this queue enabled?
    pub instr_counter: u64,             // Counter to avoid unnecessary calls to elapsed time
    pub last_time_checked: Instant,     // Last time the interrupt flag was checked
    pub rx_pkts: u64,                   // The number of received packets since the last check
    pub interval: u64,                  // The interval to check the interrupt flag
    pub moving_avg: InterruptMovingAvg, // The moving average of the hybrid interrupt
}

#[derive(Default)]
pub struct InterruptMovingAvg {
    pub measured_rates: VecDeque<u64>, // Moving average window
    pub sum: u64,                      // Moving average sum
}

impl Interrupts {
    /// Setup VFIO interrupts by checking the `device_fd` for which interrupts this device supports.
    pub fn vfio_setup_interrupt(&mut self, device_fd: RawFd) -> Result<(), Box<dyn Error>> {
        info!("setting up VFIO interrupts");
        for index in (0..=VFIO_PCI_MSIX_IRQ_INDEX).rev() {
            let mut irq_info: vfio_irq_info = vfio_irq_info {
                argsz: mem::size_of::<vfio_irq_info>() as u32,
                index: index as u32,
                flags: 0,
                count: 0,
            };

            if unsafe { libc::ioctl(device_fd, VFIO_DEVICE_GET_IRQ_INFO, &mut irq_info) } == -1 {
                return Err(format!(
                    "failed to VFIO_DEVICE_GET_IRQ_INFO for index {}. Errno: {}",
                    index,
                    std::io::Error::last_os_error()
                )
                .into());
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
        let mut event: Event = Event {
            events: libc::EPOLLIN as u32,
            data: event_fd as u64,
        };

        let epoll_fd: RawFd = unsafe { libc::epoll_create1(0) };
        if epoll_fd == -1 {
            return Err(format!(
                "failed to epoll_create1. Errno: {}",
                std::io::Error::last_os_error()
            )
            .into());
        }

        if unsafe {
            libc::epoll_ctl(
                epoll_fd,
                libc::EPOLL_CTL_ADD,
                event_fd,
                &mut event as *mut _ as *mut libc::epoll_event,
            )
        } == -1
        {
            return Err(format!(
                "failed to epoll_ctl. Errno: {}",
                std::io::Error::last_os_error()
            )
            .into());
        }

        self.vfio_epoll_fd = epoll_fd;
        Ok(())
    }

    /// Waits for events on the epoll instance referred to by the file descriptor `epoll_fd`.
    ///
    /// The memory area pointed to by events will contain the events that will be available for the caller.
    /// The `timeout` argument specifies the minimum number of milliseconds that epoll_wait will block.
    /// Specifying a `timeout` of -1 causes epoll_wait to block indefinitely, while specifying a
    /// `timeout` equal to zero cause epoll_wait to return immediately, even if no events are available.
    /// Returns the number of ready file descriptors.
    pub fn vfio_epoll_wait(&self, timeout: i32) -> Result<usize, Box<dyn Error>> {
        let mut events = [Event::default(); 10];
        let rc: usize;

        let status = unsafe {
            libc::epoll_wait(
                self.vfio_epoll_fd,
                events.as_mut_ptr() as *mut libc::epoll_event,
                events.len() as i32,
                timeout,
            )
        };
        if status == -1 {
            return Err(format!(
                "failed to epoll_wait. Errno: {}",
                std::io::Error::last_os_error()
            )
            .into());
        }
        rc = status as usize;
        if rc > 0 {
            /* epoll_wait has at least one fd ready to read */
            for event in events.iter().take(rc) {
                let mut val: u64 = 0;
                // read event file descriptor to clear interrupt.
                if unsafe {
                    libc::read(
                        event.data as i32,
                        &mut val as *mut _ as *mut libc::c_void,
                        mem::size_of::<u64>(),
                    )
                } == -1
                {
                    return Err(format!(
                        "failed to read event. Errno: {}",
                        std::io::Error::last_os_error()
                    )
                    .into());
                }
            }
        }

        Ok(rc)
    }

    /// Enable VFIO MSI interrupts for the given `device_fd`.
    pub fn vfio_enable_msi(&mut self, device_fd: RawFd) -> Result<(), Box<dyn Error>> {
        info!("enabling MSI interrupts");
        // setup event fd
        let event_fd: RawFd = unsafe { libc::eventfd(0, 0) };

        if event_fd == -1 {
            return Err(format!(
                "failed to create eventfd. Errno: {}",
                std::io::Error::last_os_error()
            )
            .into());
        }

        let irq_set: vfio_irq_set<[u8; 1]> = vfio_irq_set {
            argsz: mem::size_of::<vfio_irq_set<[u8; 1]>>() as u32,
            count: 1,
            flags: VFIO_IRQ_SET_DATA_EVENTFD | VFIO_IRQ_SET_ACTION_TRIGGER,
            index: VFIO_PCI_MSI_IRQ_INDEX as u32,
            start: 0,
            data: [event_fd as u8; 1],
        };

        if unsafe { libc::ioctl(device_fd, VFIO_DEVICE_SET_IRQS, &irq_set) } == -1 {
            return Err(format!(
                "failed to VFIO_DEVICE_SET_IRQS. Errno: {}",
                std::io::Error::last_os_error()
            )
            .into());
        }

        self.vfio_event_fd = event_fd;
        Ok(())
    }

    /// Disable VFIO MSI interrupts for the given `device_fd`.
    #[allow(dead_code)]
    pub fn vfio_disable_msi(&mut self, device_fd: RawFd) -> Result<(), Box<dyn Error>> {
        info!("disabling MSI interrupts");
        let irq_set: vfio_irq_set<[u8; 0]> = vfio_irq_set {
            argsz: mem::size_of::<vfio_irq_set<[u8; 0]>>() as u32,
            count: 0,
            flags: VFIO_IRQ_SET_DATA_NONE | VFIO_IRQ_SET_ACTION_TRIGGER,
            index: VFIO_PCI_MSI_IRQ_INDEX as u32,
            start: 0,
            data: [0; 0],
        };

        if unsafe { libc::ioctl(device_fd, VFIO_DEVICE_SET_IRQS, &irq_set) } == -1 {
            return Err(format!(
                "failed to VFIO_DEVICE_SET_IRQS. Errno: {}",
                std::io::Error::last_os_error()
            )
            .into());
        }

        self.vfio_event_fd = 0;
        Ok(())
    }

    /// Enable VFIO MSI-X interrupts for the given `device_fd`.
    ///
    /// The `interrupt_vector` specifies the number of queues to watch.
    pub fn vfio_enable_msix(
        &mut self,
        device_fd: RawFd,
        mut interrupt_vector: u32,
    ) -> Result<(), Box<dyn Error>> {
        info!("enabling MSIX interrupts");
        if device_fd < 0 {
            return Err("device file descriptor invalid!".to_string().into());
        }
        // setup event fd
        let event_fd: RawFd = unsafe { libc::eventfd(0, 0) };
        if event_fd == -1 {
            return Err(format!(
                "failed to create eventfd. Errno: {}",
                std::io::Error::last_os_error()
            )
            .into());
        }

        if interrupt_vector == 0 {
            interrupt_vector = 1;
        } else if interrupt_vector > MAX_INTERRUPT_VECTORS {
            interrupt_vector = MAX_INTERRUPT_VECTORS + 1;
        }

        let irq_set: vfio_irq_set<[u8; 1]> = vfio_irq_set {
            argsz: mem::size_of::<vfio_irq_set<[u8; 1]>>() as u32,
            count: interrupt_vector,
            flags: VFIO_IRQ_SET_DATA_EVENTFD | VFIO_IRQ_SET_ACTION_TRIGGER,
            index: VFIO_PCI_MSIX_IRQ_INDEX as u32,
            start: 0,
            data: [event_fd as u8; 1],
        };

        if unsafe { libc::ioctl(device_fd, VFIO_DEVICE_SET_IRQS, &irq_set) } == -1 {
            return Err(format!(
                "failed to VFIO_DEVICE_SET_IRQS. Errno: {}",
                std::io::Error::last_os_error()
            )
            .into());
        }

        self.vfio_event_fd = event_fd;
        Ok(())
    }

    /// Disable VFIO MSI-X interrupts for the given `device_fd`.
    #[allow(dead_code)]
    pub fn vfio_disable_msix(&mut self, device_fd: RawFd) -> Result<(), Box<dyn Error>> {
        info!("disabling MSIX interrupts");
        let irq_set: vfio_irq_set<[u8; 0]> = vfio_irq_set {
            argsz: mem::size_of::<vfio_irq_set<[u8; 0]>>() as u32,
            count: 0,
            flags: VFIO_IRQ_SET_DATA_NONE | VFIO_IRQ_SET_ACTION_TRIGGER,
            index: VFIO_PCI_MSIX_IRQ_INDEX as u32,
            start: 0,
            data: [0; 0],
        };

        if unsafe { libc::ioctl(device_fd, VFIO_DEVICE_SET_IRQS, &irq_set) } == -1 {
            return Err(format!(
                "failed to VFIO_DEVICE_SET_IRQS. Errno: {}",
                std::io::Error::last_os_error()
            )
            .into());
        }

        self.vfio_event_fd = 0;
        Ok(())
    }

    /// Calculate packets per millisecond based on the received number of packets and the
    /// elapsed time in `nanos` since the last calculation.
    /// Returns the number of packets per millisecond.
    pub fn ppms(&self, nanos: u64) -> u64 {
        self.rx_pkts / (nanos / 1_000_000)
    }

    /// Check if interrupts or polling should be used based on the current number of received packets per seconds.
    /// The `diff` specifies time elapsed since the last call in nanoseconds.
    /// The `buf_index` and `buf_size` the current buffer index and the size of the receive buffer.
    pub fn check_interrupt(&mut self, diff: u64, buf_index: usize, buf_size: usize) {
        let ppms = self.ppms(diff);
        self.moving_avg.sum += ppms;
        self.moving_avg.measured_rates.push_back(ppms);
        if self.moving_avg.measured_rates.len() >= MOVING_AVERAGE_RANGE {
            self.moving_avg.sum -= self.moving_avg.measured_rates.pop_front().unwrap();
        }
        self.rx_pkts = 0;
        let average = self.moving_avg.sum / self.moving_avg.measured_rates.len() as u64;
        if average > INTERRUPT_THRESHOLD || buf_index == buf_size {
            self.interrupt_enabled = false;
        } else {
            self.interrupt_enabled = true;
        }
        self.last_time_checked = Instant::now();
    }
}
