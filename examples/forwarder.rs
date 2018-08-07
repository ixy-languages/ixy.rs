#![feature(duration_as_u128)]

extern crate ixy;

use std::env;
use std::process;
use std::time::Instant;

use ixy::*;
use ixy::memory::Packet;

const BATCH_SIZE: usize = 32;

// cargo run --example forwarder 0000:05:00.0 0000:05:00.1
pub fn main() {
    let mut args = env::args();
    args.next();

    let pci_addr_1 = match args.next() {
        Some(arg) => arg,
        None => {
            eprintln!("Usage: cargo run --example forwarder <pci bus id1> <pci bus id2>");
            process::exit(1);
        }
    };

    let pci_addr_2 = match args.next() {
        Some(arg) => arg,
        None => {
            eprintln!("Usage: cargo run --example forwarder <pci bus id1> <pci bus id2>");
            process::exit(1);
        }
    };

    let mut dev1 = ixy_init(&pci_addr_1, 1, 1).unwrap();
    let mut dev2 = ixy_init(&pci_addr_2, 1, 1).unwrap();

    let mut dev1_stats = DeviceStats::new();
    let mut dev1_stats_old = DeviceStats::new();
    let mut dev2_stats = DeviceStats::new();
    let mut dev2_stats_old = DeviceStats::new();

    dev1.reset_stats();
    dev2.reset_stats();

    dev1.read_stats(&mut dev1_stats);
    dev1.read_stats(&mut dev1_stats_old);
    dev2.read_stats(&mut dev2_stats);
    dev2.read_stats(&mut dev2_stats_old);

    let mut buffer: Vec<Packet> = Vec::with_capacity(BATCH_SIZE);
    let mut time = Instant::now();
    let mut counter = 0;

    loop {
        forward(&mut buffer, &mut dev1, 0, &mut dev2, 0);
        forward(&mut buffer, &mut dev2, 0, &mut dev1, 0);

        if counter & 0xfff == 0 {
            let nanos = time.elapsed().as_nanos() as u64;
            if nanos > 1_000_000_000 {
                dev1.read_stats(&mut dev1_stats);
                dev1_stats.print_stats_diff(&dev1, &dev1_stats_old, nanos);
                dev1_stats_old.set_to_stats(&dev1_stats);

                if dev1 != dev2 {
                    dev2.read_stats(&mut dev2_stats);
                    dev2_stats.print_stats_diff(&dev2, &dev2_stats_old, nanos);
                    dev2_stats_old.set_to_stats(&dev2_stats);
                }

                time = Instant::now();
            }
        }

        counter += 1;
    }
}

fn forward(buffer: &mut Vec<Packet>, rx_dev: &mut IxyDevice, rx_queue: u32, tx_dev: &mut IxyDevice, tx_queue: u32) {
    let num_rx = rx_dev.rx_batch(rx_queue, buffer, BATCH_SIZE);

    if num_rx > 0 {
        for p in buffer.iter_mut() {
            p[48] += 1;
        }

        tx_dev.tx_batch(tx_queue, buffer);
    }
}