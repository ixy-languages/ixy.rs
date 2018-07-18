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

    println!("waiting for packets");

    loop {
        let packets = dev.rx_batch(0, batch_size);

        if packets.len() > 0 {
            println!("Packets received: {}", packets.len());
            let sent = dev.tx_batch(0, packets);
            println!("Packets sent: {}", sent);
        }
    }
}
