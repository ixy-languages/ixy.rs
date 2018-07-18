extern crate libc;

#[allow(dead_code)]
#[allow(non_snake_case)]
#[allow(non_camel_case_types)]
#[allow(non_upper_case_globals)]
mod constants;

mod ixgbe;
mod memory;
mod pci;

use self::ixgbe::*;
use self::memory::*;

use std::error::Error;
use std::mem;


const MAX_QUEUES: u32 = 64;


pub struct IxyDevice {
    pci_addr: String,
    driver_name: String,
    num_rx_queues: u32,
    num_tx_queues: u32,
    driver: Box<IxyDriver>,
}

pub struct DeviceStats {
    device: IxyDevice,
    rx_pkts: u64,
    tx_pkts: u64,
    rx_bytes: u64,
    tx_bytes: u64,
}


pub trait IxyDriver {
    fn init(pci_addr: &str, num_rx_queues: u32, num_tx_queues: u32) -> Result<Self, Box<Error>> where Self: Sized;
    fn driver_name(&self) -> &str;
    fn rx_batch(&mut self, queue_id: u32, num_bufs: u32) -> Vec<Packet>;
    fn tx_batch(&mut self, queue_id: u32, packets: Vec<Packet>) -> u32;
    fn read_stats(&self, stats: &mut DeviceStats);
    fn reset_stats(&self);
    fn set_promisc(&self, enabled: bool);
    fn get_link_speed(&self) -> u16;
}

impl IxyDevice {
    pub fn rx_batch(&mut self, queue_id: u32, num_packets: u32) -> Vec<Packet> {
        self.driver.rx_batch(queue_id, num_packets)
    }

    pub fn tx_batch(&mut self, queue_id: u32, packets: Vec<Packet>) -> u32 {
        self.driver.tx_batch(queue_id, packets)
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
}

pub fn ixy_init(pci_addr: &str, rx_queues: u32, tx_queues: u32) -> Result<IxyDevice, Box<Error>> {
    let driver: IxgbeDevice  = IxyDriver::init(pci_addr, rx_queues, tx_queues).unwrap();

    let ixy = IxyDevice {
        pci_addr: pci_addr.to_string(),
        driver_name: driver.driver_name().to_string(),
        num_rx_queues: rx_queues,
        num_tx_queues: tx_queues,
        driver: Box::new(driver),
    };

    Ok(ixy)
}

/*
 * echo -n "0000:02:00.1" > /sys/bus/pci/drivers/igb_uio/unbind
 * echo -n "0000:03:00.1" > /sys/bus/pci/drivers/ixgbe/unbind
 */
pub fn unbind_driver(pci_addr: &str) -> Result<(), Box<Error>> {
    pci::unbind_driver(pci_addr)
}
