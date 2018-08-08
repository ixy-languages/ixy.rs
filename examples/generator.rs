#![feature(duration_as_u128)]

extern crate ixy;
use std::env;

use std::process;
use std::time::Instant;
use std::rc::Rc;
use std::cell::RefCell;

use ixy::*;
use ixy::memory::{Packetpool, Packet, alloc_pkt_batch};

// number of packets sent simultaneously by our driver
const BATCH_SIZE: usize = 32;
// number of packets in our memorypool
const NUM_PACKETS: usize = 2048;

const PACKET_SIZE: usize = 60;

// cargo run --example generator 0000:05:00.0
pub fn main() {
    let mut args = env::args();
    args.next();

    let pci_addr = match args.next() {
        Some(arg) => arg,
        None => {
            eprintln!("Usage: cargo run --example generator <pci bus id>");
            process::exit(1);
        }
    };

    let mut dev = ixy_init(&pci_addr, 1, 1).unwrap();

    let pkt_data = [
        0x01, 0x02, 0x03, 0x04, 0x05, 0x06,         // dst MAC
        0x11, 0x12, 0x13, 0x14, 0x15, 0x16,         // src MAC
        0x08, 0x00,                                 // ether type: IPv4
        0x45, 0x00,                                 // Version, IHL, TOS
        ((PACKET_SIZE - 14) >> 8) as u8,            // ip len excluding ethernet, high byte
        ((PACKET_SIZE - 14) & 0xFF) as u8,          // ip len excluding ethernet, low byte
        0x00, 0x00, 0x00, 0x00,                     // id, flags, fragmentation
        0x40, 0x11, 0x00, 0x00,                     // TTL (64), protocol (UDP), checksum
        0x0A, 0x00, 0x00, 0x01,                     // src ip (10.0.0.1)
        0x0A, 0x00, 0x00, 0x02,                     // dst ip (10.0.0.2)
        0x00, 0x2A, 0x05, 0x39,                     // src and dst ports (42 -> 1337)
        ((PACKET_SIZE - 20 - 14) >> 8) as u8,       // udp len excluding ip & ethernet, high byte
        ((PACKET_SIZE - 20 - 14) & 0xFF) as u8,     // udp len excluding ip & ethernet, low byte
        0x00, 0x00,                                 // udp checksum, optional
        b'i', b'x', b'y'                            // payload
        // rest of the payload is zero-filled because mempools guarantee empty bufs
    ];

    let pool = Packetpool::allocate(NUM_PACKETS, 0).unwrap();

    // pre-fill all packet buffer in the memory pool with data and return them to
    // the memory pool
    {
        let mut buffer: Vec<Packet> = Vec::with_capacity(NUM_PACKETS);

        alloc_pkt_batch(&pool, &mut buffer, NUM_PACKETS, PACKET_SIZE);

        for p in buffer.iter_mut() {
            for (i, data) in pkt_data.iter().enumerate() {
                p[i] = *data;
            }
        }
    }

    let mut dev_stats = DeviceStats::new();
    let mut dev_stats_old = DeviceStats::new();

    dev.reset_stats();

    dev.read_stats(&mut dev_stats);
    dev.read_stats(&mut dev_stats_old);

    let mut buffer: Vec<Packet> = Vec::with_capacity(BATCH_SIZE);
    let mut time = Instant::now();
    let mut seq_num = 0;
    let mut counter = 0;

    loop {
        // re-fill our packet queue with new packets to send out
        alloc_pkt_batch(&pool, &mut buffer, BATCH_SIZE, PACKET_SIZE);

        // update sequence number and checksum of all packets
        for p in buffer.iter_mut() {
            p[PACKET_SIZE-4] = seq_num;
            let checksum = calc_ip_checksum(p);
            p[24] = (checksum >> 8) as u8;
            p[25] = (checksum & 0xff) as u8;
            seq_num += 1;
        }

        dev.tx_batch(0, &mut buffer);

        // don't poll the time unnecessarily
        if counter & 0xfff == 0 {
            let nanos = time.elapsed().as_nanos() as u64;
            // every second
            if nanos > 1_000_000_000 {
                dev.read_stats(&mut dev_stats);
                dev_stats.print_stats_diff(&dev, &dev_stats_old, nanos);
                dev_stats_old.set_to_stats(&dev_stats);

                time = Instant::now();
            }
        }

        counter += 1;
    }
}
// calculate IP/TCP/UDP checksum
fn calc_ip_checksum(packet: &mut Packet) -> u16 {
    let mut checksum = 0;
    for i in 0..packet.len()/2 {
        checksum += ((packet[i] as u32) << 8) + packet[i+1] as u32;
        if checksum > 0xffff {
            checksum = (checksum & 0xfff) + 1;
        }
    }
    return !(checksum as u16)
}