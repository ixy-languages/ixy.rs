const MOVING_AVERAGE_RANGE: usize = 5;
const INTERRUPT_THRESHOLD: f64 = 1.2;
pub const INTERRUPT_INITIAL_INTERVAL: u64 = 1000 * 1000 * 1000;

#[derive(Default, Copy, Clone)]
pub struct Interrupts {
    pub interrupts_enabled: bool, // Whether interrupts for this device are enabled or disabled.
    pub itr_rate: u32, // The Interrupt Throttling Rate
    pub interrupt_type: u8, // MSI or MSIX
    pub timeout_ms: i16, // interrupt timeout in milliseconds (-1 to disable the timeout)
    pub queues: Vec<InterruptsQueue>,  // Interrupt settings per queue
}

#[derive(Default, Copy, Clone)]
pub struct InterruptsQueue {
    pub vfio_event_fd: u32, // event fd
    pub vfio_epoll_fd: u32, // epoll fd
    pub interrupt_enabled: bool, // Whether interrupt for this queue is enabled or not
    pub last_time_checked: u64, // Last time the interrupt flag was checked
    pub rx_pkts: u64, // The number of received packets since the last check
    pub interval: u64, // The interval to check the interrupt flag
    pub moving_avg: InterruptMovingAvg, // The moving average of the hybrid interrupt
}

#[derive(Default, Copy, Clone)]
pub struct InterruptMovingAvg {
    index: usize, // The current index
    length: usize, // The moving average length
    sum: f64, // The moving average sum
    measured_rates: [f64; MOVING_AVERAGE_RANGE], // The moving average window
}

#[allow(non_camel_case_types)]
#[repr(C)]
struct vfio_irq_set {
    argsz: u32,
    flags: u32,
    index: u32,
    start: u32,
    count: u32,
    data: Vec<u8>,
}

impl InterruptsQueue {
    /**
     * Enable VFIO MSI interrupts.
     * @param device_fd The VFIO file descriptor.
     */
    fn vfio_enable_msi(&mut self, device_fd: RawFd) {
        let irq_set_buf = [char, IRQ_SET_BUF_LEN];
        let mut fd_ptr: &Vec<u8>;

        // setup event fd
        let mut event_fd: RawFd = eventfd(0, 0);

        let irq_set: &mut vfio_irq_set =  Default::default();
        irq_set.argsz = sizeof(irq_set_buf);
        irq_set.count = 1;
        irq_set.flags = VFIO_IRQ_SET_DATA_EVENTFD | VFIO_IRQ_SET_ACTION_TRIGGER;
        irq_set.index = VFIO_PCI_MSI_IRQ_INDEX;
        irq_set.start = 0;
        fd_ptr = &irq_set.data;
        fd_ptr = event_fd;

        if unsafe { ioctl(device_fd, VFIO_DEVICE_SET_IRQS, irq_set) } == -1 {
            return Err("enable MSI interrupts".into());
        }

        self.vfio_event_fd = event_fd;
    }

    /**
     * Disable VFIO MSI interrupts.
     * @param device_fd The VFIO file descriptor.
     * @return 0 on success.
     */
    fn vfio_disable_msi(&mut self, device_fd: RawFd) {
        let irq_set_buf = [char, IRQ_SET_BUF_LEN];

        let irq_set: &mut vfio_irq_set =  Default::default();
        irq_set.argsz = sizeof(irq_set_buf);
        irq_set.count = 0;
        irq_set.flags = VFIO_IRQ_SET_DATA_NONE | VFIO_IRQ_SET_ACTION_TRIGGER;
        irq_set.index = VFIO_PCI_MSI_IRQ_INDEX;
        irq_set.start = 0;

        if unsafe { ioctl(device_fd, VFIO_DEVICE_SET_IRQS, irq_set) } == -1 {
            return Err("disable MSI interrupts".into());
        }

        self.vfio_event_fd = 0;
    }

    /**
     * Calculate packets per second based on the received number of packets and the elapsed time in nanoseconds since the
     * last calculation.
     * @param elapsed_time_nanos Time elapsed in nanoseconds since the last calculation.
     * @return Packets per second.
     */
    fn diff_mpps(&self, nanos: u32) -> f64 {
        self.rx_pkts as f64 / 1_000_000.0 / (f64::from(nanos) / 1_000_000_000.0)
    }

    /**
     * Check if interrupts or polling should be used based on the current number of received packets per seconds.
     * @param diff The difference since the last call in nanoseconds.
     * @param buf_index The current buffer index.
     * @param buf_size The maximum buffer size.
     * @return Whether to disable NIC interrupts or not.
     */
    fn check_interrupt(&mut self, diff: u64, buf_index: u32, buf_size: u32) {
        self.moving_avg.measured_rates[self.moving_avg.index] = self.mpps(diff);
        self.moving_avg.sum += self.moving_avg.measured_rates[self.moving_avg.index];
        if self.moving_avg.length == MOVING_AVERAGE_RANGE {
            if self.moving_avg.index == 0 {
                self.moving_avg.sum -= self.moving_avg.measured_rates[MOVING_AVERAGE_RANGE - 1];
            } else {
                self.moving_avg.sum -= self.moving_avg.measured_rates[self.moving_avg.index - 1];
            }
            self.moving_avg.length -= 1;
        }
        self.moving_avg.index = (self.moving_avg.index + 1) % MOVING_AVERAGE_RANGE;
        self.moving_avg.length += 1;
        self.moving_avg.rx_pkts = 0;
        let mut average = self.moving_avg.sum / (self.moving_avg.length - 1) as f64;
        if average > INTERRUPT_THRESHOLD {
            self.interrupt_enabled = false;
        } else if buf_index == buf_size {
            self.interrupt_enabled = false;
        } else {
            self.interrupt_enabled = true;
        }
        self.last_time_checked = monotonic_time();
    }
}