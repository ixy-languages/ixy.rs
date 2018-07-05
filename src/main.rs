#![feature(asm)]
#![feature(const_fn)]
#![feature(untagged_unions)]
#![feature(extern_prelude)]

#[allow(dead_code)]
#[allow(unused_variables)]
mod driver;
use driver::*;

use std::env;
use std::process;

// cargo run 0000:03:00.1
fn main() {
    let mut args = env::args();
    args.next();

    let pci_addr = match args.next() {
        Some(arg) => arg,
        None => {
            eprintln!("Problem parsing arguments: {}", "too few arguments");
            process::exit(1);
        }
    };

    let pci_addr = format!("/sys/bus/pci/devices/{}/resource0", pci_addr);

    unbind_driver(&pci_addr).expect("driver could not be unbound");

    let mut dev = ixy_init(&pci_addr, 1, 1).unwrap();

    let batch_size = 32;

    loop {
        let packets = dev.rx_batch(0, batch_size);

        for packet in packets{
            println!("Packet address: {}", packet as usize);
        }
    }


    //let resource = format!("/sys/bus/pci/devices/{}/resource0", pci_addr);
    //let pci_addr = pci_map(&resource).unwrap();
    // TODO: ixgbe_init with amount of rx- and tx-queues
    //let ixgbe = driver::ixgbe_init( &pci_addr, 15, 15);
    //unsafe { println!("Link speed: {} Mbit/s", get_link_speed(&ixgbe)) };
    //get_link_speed(&ixgbe);
}

/*pub fn receive(dev: &IxyDevice, queue_id: u32) {
    //let received =
}*/