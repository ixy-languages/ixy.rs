use std::collections::VecDeque;
use std::env;
use std::process;
use std::time::Instant;

use ixy::memory::Packet;
use ixy::*;
use simple_logger::SimpleLogger;

const BATCH_SIZE: usize = 32;

pub fn main() {
    SimpleLogger::new().init().unwrap();

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

    let mut dev1 = ixy_init(&pci_addr_1, 1, 1, -1).unwrap();
    let mut dev2 = ixy_init(&pci_addr_2, 1, 1, 0).unwrap();

    let mut dev1_stats = Default::default();
    let mut dev1_stats_old = Default::default();
    let mut dev2_stats = Default::default();
    let mut dev2_stats_old = Default::default();

    dev1.reset_stats();
    dev2.reset_stats();

    dev1.read_stats(&mut dev1_stats);
    dev1.read_stats(&mut dev1_stats_old);
    dev2.read_stats(&mut dev2_stats);
    dev2.read_stats(&mut dev2_stats_old);

    let mut buffer: VecDeque<Packet> = VecDeque::with_capacity(BATCH_SIZE);
    let mut time = Instant::now();
    let mut counter = 0;

    loop {
        forward(&mut buffer, &mut *dev1, 0, &mut *dev2, 0);
        forward(&mut buffer, &mut *dev2, 0, &mut *dev1, 0);

        // don't poll the time unnecessarily
        if counter & 0xfff == 0 {
            let elapsed = time.elapsed();
            let nanos = elapsed.as_secs() * 1_000_000_000 + u64::from(elapsed.subsec_nanos());
            // every second
            if nanos > 1_000_000_000 {
                dev1.read_stats(&mut dev1_stats);
                dev1_stats.print_stats_diff(&dev1, &dev1_stats_old, nanos);
                dev1_stats_old = dev1_stats;

                dev2.read_stats(&mut dev2_stats);
                dev2_stats.print_stats_diff(&dev2, &dev2_stats_old, nanos);
                dev2_stats_old = dev2_stats;

                time = Instant::now();
            }
        }

        counter += 1;
    }
}

fn forward(
    buffer: &mut VecDeque<Packet>,
    rx_dev: &mut dyn IxyDevice,
    rx_queue: u16,
    tx_dev: &mut dyn IxyDevice,
    tx_queue: u16,
) {
    let num_rx = rx_dev.rx_batch(rx_queue, buffer, BATCH_SIZE);

    if num_rx > 0 {
        // touch all packets for a realistic workload
        for p in buffer.iter_mut() {
            // we change a byte of the destination MAC address to ensure
            // that all packets are put back on the link (vital for VFs)
            p[3] += 1;
        }

        tx_dev.tx_batch(tx_queue, buffer);

        // drop packets if they haven't been sent out
        buffer.drain(..);
    }
}
