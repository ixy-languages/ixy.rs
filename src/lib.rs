#![feature(const_fn)]
#![feature(untagged_unions)]

extern crate libc;
extern crate byteorder;

#[allow(dead_code)]
#[allow(non_snake_case)]
#[allow(non_camel_case_types)]
#[allow(non_upper_case_globals)]
mod constants;

mod ixgbe;
pub mod memory;
mod pci;

use self::ixgbe::*;
use self::memory::*;
use self::pci::*;

use std::error::Error;
use std::io::Seek;
use std::io::SeekFrom;

use byteorder::ReadBytesExt;
use byteorder::NativeEndian;


const MAX_QUEUES: u16 = 64;


pub struct IxyDevice {
    pci_addr: String,
    driver_name: String,
    num_rx_queues: u16,
    num_tx_queues: u16,
    driver: Box<IxyDriver>,
}

pub struct DeviceStats {
    pub rx_pkts: u64,
    pub tx_pkts: u64,
    pub rx_bytes: u64,
    pub tx_bytes: u64,
}


pub trait IxyDriver {
    fn init(pci_addr: &str, num_rx_queues: u16, num_tx_queues: u16) -> Result<Self, Box<Error>> where Self: Sized;
    fn get_driver_name(&self) -> &str;
    fn rx_batch(&mut self, queue_id: u32, buffer: &mut Vec<Packet>, num_packets: usize) -> usize;
    fn tx_batch(&mut self, queue_id: u32, packets: &mut Vec<Packet>) -> usize;
    fn read_stats(&self, stats: &mut DeviceStats);
    fn reset_stats(&self);
    fn set_promisc(&self, enabled: bool);
    fn get_link_speed(&self) -> u16;
}

impl DeviceStats {
    pub fn new() -> Self {
        DeviceStats {
            rx_pkts: 0,
            tx_pkts: 0,
            rx_bytes: 0,
            tx_bytes: 0,
        }
    }

    pub fn set_to_stats(&mut self, stats: &DeviceStats) {
        self.rx_pkts = stats.rx_pkts;
        self.tx_pkts = stats.tx_pkts;
        self.rx_bytes = stats.rx_bytes;
        self.tx_bytes = stats.tx_bytes;
    }

    pub fn print_stats_diff(&self, dev: &IxyDevice, stats_old: &DeviceStats, nanos: u64) {
        let pci_addr = dev.get_pci_addr();
        let mbits = diff_mbit(self.rx_bytes, stats_old.rx_bytes, self.rx_pkts, stats_old.rx_pkts, nanos);
        let mpps = diff_mpps(self.rx_pkts, stats_old.rx_pkts, nanos);
        println!("[{}] RX: {:.2} Mbit/s {:.2} Mpps", pci_addr, mbits, mpps);
        let mbits = diff_mbit(self.tx_bytes, stats_old.tx_bytes, self.tx_pkts, stats_old.tx_pkts, nanos);
        let mpps = diff_mpps(self.tx_pkts, stats_old.tx_pkts, nanos);
        println!("[{}] TX: {:.2} Mbit/s {:.2} Mpps", pci_addr, mbits, mpps);
    }
}

fn diff_mbit(bytes_new: u64, bytes_old: u64, pkts_new: u64, pkts_old: u64, nanos: u64) -> f64 {
    (((bytes_new - bytes_old) as f64 / 1000000.0 / (nanos as f64 / 1000000000.0)) * 8 as f64
        + diff_mpps(pkts_new, pkts_old, nanos) * 20 as f64 * 8 as f64)
}

fn diff_mpps(pkts_new: u64, pkts_old: u64, nanos: u64) -> f64 {
    (pkts_new - pkts_old) as f64 / 1000000.0 / (nanos as f64 / 1000000000.0)
}

impl IxyDevice {
    pub fn rx_batch(&mut self, queue_id: u32, buffer: &mut Vec<Packet>, num_packets: usize) -> usize {
        self.driver.rx_batch(queue_id, buffer, num_packets)
    }

    pub fn tx_batch(&mut self, queue_id: u32, buffer: &mut Vec<Packet>) -> usize {
        self.driver.tx_batch(queue_id, buffer)
    }

    pub fn read_stats(&self, stats: &mut DeviceStats) {
        self.driver.read_stats(stats)
    }

    pub fn reset_stats(&self) {
        self.driver.reset_stats();
    }

    pub fn set_promisc(&self, enabled: bool) {
        self.driver.set_promisc(enabled);
    }

    pub fn get_link_speed(&self) -> u16 {
        self.driver.get_link_speed()
    }

    pub fn get_pci_addr(&self) -> &str {
        &self.pci_addr
    }
}

impl std::cmp::PartialEq for IxyDevice {
    fn eq(&self, other: &'_ IxyDevice) -> bool {
        self.pci_addr == other.pci_addr
    }
}

pub fn ixy_init(pci_addr: &str, rx_queues: u16, tx_queues: u16) -> Result<IxyDevice, Box<Error>> {
    {
        let mut config_file = pci_open_resource(pci_addr, "config")?;

        config_file.seek(SeekFrom::Start(8))?;
        let class_id = config_file.read_u32::<NativeEndian>()? >> 24;

        if class_id != 2 {
            panic!("Device {} is not a network card!", pci_addr);
        }
    }

    let driver: IxgbeDevice  = IxyDriver::init(pci_addr, rx_queues, tx_queues).unwrap();

    let ixy = IxyDevice {
        pci_addr: pci_addr.to_string(),
        driver_name: driver.get_driver_name().to_string(),
        num_rx_queues: rx_queues,
        num_tx_queues: tx_queues,
        driver: Box::new(driver),
    };

    Ok(ixy)
}