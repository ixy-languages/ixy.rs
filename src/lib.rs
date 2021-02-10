//! # ixy.rs
//!
//! ixy.rs is a Rust rewrite of the ixy userspace network driver.
//! It is designed to be readable, idiomatic Rust code.
//! It supports Intel 82599 10GbE NICs (ixgbe family).

#![warn(rust_2018_idioms)]

#[macro_use]
extern crate log;

#[rustfmt::skip]
mod constants;
mod interrupts;
mod ixgbe;
mod ixgbevf;
pub mod memory;
mod pci;
mod vfio;
mod virtio;
#[rustfmt::skip]
mod virtio_constants;

use self::interrupts::*;
use self::ixgbe::*;
use self::ixgbevf::*;
use self::memory::*;
use self::pci::*;
use self::virtio::VirtioDevice;

use std::collections::VecDeque;
use std::error::Error;
use std::os::unix::io::RawFd;

/// Used for implementing an ixy device driver like ixgbe or virtio.
pub trait IxyDevice {
    /// Returns the driver's name.
    fn get_driver_name(&self) -> &str;

    /// Returns the card's iommu capability.
    fn is_card_iommu_capable(&self) -> bool;

    /// Returns VFIO container file descriptor or [`None`] if IOMMU is not available.
    fn get_vfio_container(&self) -> Option<RawFd>;

    /// Returns the pci address of this device.
    fn get_pci_addr(&self) -> &str;

    /// Returns the layer 2 address of this device.
    fn get_mac_addr(&self) -> [u8; 6];

    /// Sets the layer 2 address of this device.
    fn set_mac_addr(&self, mac: [u8; 6]);

    /// Pushes up to `num_packets` `Packet`s onto `buffer` depending on the amount of
    /// received packets by the network card. Returns the number of received packets.
    ///
    /// # Examples
    ///
    /// ```rust,no_run
    /// use ixy::*;
    /// use ixy::memory::Packet;
    /// use std::collections::VecDeque;
    ///
    /// let mut dev = ixy_init("0000:01:00.0", 1, 1, 0).unwrap();
    /// let mut buf: VecDeque<Packet> = VecDeque::new();
    ///
    /// dev.rx_batch(0, &mut buf, 32);
    /// ```
    fn rx_batch(
        &mut self,
        queue_id: u16,
        buffer: &mut VecDeque<Packet>,
        num_packets: usize,
    ) -> usize;

    /// Takes `Packet`s out of `buffer` until `buffer` is empty or the network card's tx
    /// queue is full. Returns the number of sent packets.
    ///
    /// # Examples
    ///
    /// ```rust,no_run
    /// use ixy::*;
    /// use ixy::memory::Packet;
    /// use std::collections::VecDeque;
    ///
    /// let mut dev = ixy_init("0000:01:00.0", 1, 1, 0).unwrap();
    /// let mut buf: VecDeque<Packet> = VecDeque::new();
    ///
    /// assert_eq!(dev.tx_batch(0, &mut buf), 0);
    /// ```
    fn tx_batch(&mut self, queue_id: u16, buffer: &mut VecDeque<Packet>) -> usize;

    /// Reads the network card's stats registers into `stats`.
    ///
    /// # Examples
    ///
    /// ```rust,no_run
    /// use ixy::*;
    ///
    /// let mut dev = ixy_init("0000:01:00.0", 1, 1, 0).unwrap();
    /// let mut stats: DeviceStats = Default::default();
    ///
    /// dev.read_stats(&mut stats);
    /// ```
    fn read_stats(&self, stats: &mut DeviceStats);

    /// Resets the network card's stats registers.
    ///
    /// # Examples
    ///
    /// ```rust,no_run
    /// use ixy::*;
    ///
    /// let mut dev = ixy_init("0000:01:00.0", 1, 1, 0).unwrap();
    /// dev.reset_stats();
    /// ```
    fn reset_stats(&mut self);

    /// Returns the network card's link speed.
    ///
    /// # Examples
    ///
    /// ```rust,no_run
    /// use ixy::*;
    ///
    /// let mut dev = ixy_init("0000:01:00.0", 1, 1, 0).unwrap();
    /// println!("Link speed is {} Mbit/s", dev.get_link_speed());
    /// ```
    fn get_link_speed(&self) -> u16;

    /// Takes `Packet`s out of `buffer` to send out. This will busy wait until all packets from
    /// `buffer` are queued.
    fn tx_batch_busy_wait(&mut self, queue_id: u16, buffer: &mut VecDeque<Packet>) {
        while !buffer.is_empty() {
            self.tx_batch(queue_id, buffer);
        }
    }
}

/// Holds network card stats about sent and received packets.
#[derive(Default, Copy, Clone)]
pub struct DeviceStats {
    pub rx_pkts: u64,
    pub tx_pkts: u64,
    pub rx_bytes: u64,
    pub tx_bytes: u64,
}

impl DeviceStats {
    ///  Prints the stats differences between `stats_old` and `self`.
    pub fn print_stats_diff(&self, dev: &dyn IxyDevice, stats_old: &DeviceStats, nanos: u64) {
        let pci_addr = dev.get_pci_addr();
        let mbits = self.diff_mbit(
            self.rx_bytes,
            stats_old.rx_bytes,
            self.rx_pkts,
            stats_old.rx_pkts,
            nanos,
        );
        let mpps = self.diff_mpps(self.rx_pkts, stats_old.rx_pkts, nanos);
        println!("[{}] RX: {:.2} Mbit/s {:.2} Mpps", pci_addr, mbits, mpps);

        let mbits = self.diff_mbit(
            self.tx_bytes,
            stats_old.tx_bytes,
            self.tx_pkts,
            stats_old.tx_pkts,
            nanos,
        );
        let mpps = self.diff_mpps(self.tx_pkts, stats_old.tx_pkts, nanos);
        println!("[{}] TX: {:.2} Mbit/s {:.2} Mpps", pci_addr, mbits, mpps);
    }

    /// Returns Mbit/s between two points in time.
    fn diff_mbit(
        &self,
        bytes_new: u64,
        bytes_old: u64,
        pkts_new: u64,
        pkts_old: u64,
        nanos: u64,
    ) -> f64 {
        ((bytes_new - bytes_old) as f64 / 1_000_000.0 / (nanos as f64 / 1_000_000_000.0))
            * f64::from(8)
            + self.diff_mpps(pkts_new, pkts_old, nanos) * f64::from(20) * f64::from(8)
    }

    /// Returns Mpps between two points in time.
    fn diff_mpps(&self, pkts_new: u64, pkts_old: u64, nanos: u64) -> f64 {
        (pkts_new - pkts_old) as f64 / 1_000_000.0 / (nanos as f64 / 1_000_000_000.0)
    }
}

/// Initializes the network card at `pci_addr`.
///
/// `rx_queues` and `tx_queues` specify the number of queues that will be initialized and used
/// while `interrupt_timeout` enables interrupts if greater or less than zero.
pub fn ixy_init(
    pci_addr: &str,
    rx_queues: u16,
    tx_queues: u16,
    interrupt_timeout: i16,
) -> Result<Box<dyn IxyDevice>, Box<dyn Error>> {
    let mut vendor_file = pci_open_resource_ro(pci_addr, "vendor").expect("wrong pci address");
    let mut device_file = pci_open_resource_ro(pci_addr, "device").expect("wrong pci address");
    let mut config_file = pci_open_resource_ro(pci_addr, "config").expect("wrong pci address");

    let vendor_id = read_hex(&mut vendor_file)?;
    let device_id = read_hex(&mut device_file)?;
    let class_id = read_io32(&mut config_file, 8)? >> 24;

    if class_id != 2 {
        return Err(format!("device {} is not a network card", pci_addr).into());
    }

    if vendor_id == 0x1af4 && device_id == 0x1000 {
        // `device_id == 0x1041` would be for non-transitional devices which we don't support atm
        if rx_queues > 1 || tx_queues > 1 {
            warn!("cannot configure multiple rx/tx queues: we don't support multiqueue (VIRTIO_NET_F_MQ)");
        }
        if interrupt_timeout != 0 {
            warn!("interrupts requested but virtio does not support interrupts yet");
        }
        let device = VirtioDevice::init(pci_addr)?;
        Ok(Box::new(device))
    } else if vendor_id == 0x8086
        && (device_id == 0x10ed || device_id == 0x1515 || device_id == 0x1565)
    {
        // looks like a virtual function
        if interrupt_timeout != 0 {
            warn!("interrupts requested but ixgbevf does not support interrupts yet");
        }
        let device = IxgbeVFDevice::init(pci_addr, rx_queues, tx_queues)?;
        Ok(Box::new(device))
    } else {
        // let's give it a try with ixgbe
        let device = IxgbeDevice::init(pci_addr, rx_queues, tx_queues, interrupt_timeout)?;
        Ok(Box::new(device))
    }
}

impl IxyDevice for Box<dyn IxyDevice> {
    fn get_driver_name(&self) -> &str {
        (**self).get_driver_name()
    }

    fn is_card_iommu_capable(&self) -> bool {
        (**self).is_card_iommu_capable()
    }

    fn get_vfio_container(&self) -> Option<RawFd> {
        (**self).get_vfio_container()
    }

    fn get_pci_addr(&self) -> &str {
        (**self).get_pci_addr()
    }

    fn get_mac_addr(&self) -> [u8; 6] {
        (**self).get_mac_addr()
    }

    fn set_mac_addr(&self, addr: [u8; 6]) {
        (**self).set_mac_addr(addr)
    }

    fn rx_batch(
        &mut self,
        queue_id: u16,
        buffer: &mut VecDeque<Packet>,
        num_packets: usize,
    ) -> usize {
        (**self).rx_batch(queue_id, buffer, num_packets)
    }

    fn tx_batch(&mut self, queue_id: u16, buffer: &mut VecDeque<Packet>) -> usize {
        (**self).tx_batch(queue_id, buffer)
    }

    fn read_stats(&self, stats: &mut DeviceStats) {
        (**self).read_stats(stats)
    }

    fn reset_stats(&mut self) {
        (**self).reset_stats()
    }

    fn get_link_speed(&self) -> u16 {
        (**self).get_link_speed()
    }
}
