#![feature(duration_as_u128)]

extern crate ixy;
extern crate simple_logger;

use std::collections::VecDeque;
use std::env;
use std::process;
use std::time::Instant;

use ixy::memory::{alloc_pkt_batch, Packet, Packetpool};
use ixy::*;

// number of packets sent simultaneously by our driver
const BATCH_SIZE: usize = 32;
// number of packets in our packetpool
const NUM_PACKETS: usize = 2048;
// size of our packets
const PACKET_SIZE: usize = 60;

pub fn main() {
    simple_logger::init().unwrap();

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

    // pre-fill all packet buffer in the pool with data and return them to the packet pool
    {
        let mut buffer: VecDeque<Packet> = VecDeque::with_capacity(NUM_PACKETS);

        alloc_pkt_batch(&pool, &mut buffer, NUM_PACKETS, PACKET_SIZE);

        for p in buffer.iter_mut() {
            for (i, data) in pkt_data.iter().enumerate() {
                p[i] = *data;
            }
            let checksum = calc_ip_checksum(p, 14, 20);
            p[24] = (checksum >> 8) as u8;
            p[25] = (checksum & 0xff) as u8;
        }
    }

    let mut dev_stats = Default::default();
    let mut dev_stats_old = Default::default();

    dev.reset_stats();

    dev.read_stats(&mut dev_stats);
    dev.read_stats(&mut dev_stats_old);

    let mut buffer: VecDeque<Packet> = VecDeque::with_capacity(BATCH_SIZE);
    let mut time = Instant::now();
    let mut seq_num = 0;
    let mut counter = 0;

    loop {
        // re-fill our packet queue with new packets to send out
        alloc_pkt_batch(&pool, &mut buffer, BATCH_SIZE, PACKET_SIZE);

        // update sequence number and checksum of all packets
        for p in buffer.iter_mut() {
            p[PACKET_SIZE - 4] = seq_num;
            seq_num = (seq_num % std::u8::MAX) + 1;
        }

        dev.tx_batch(0, &mut buffer);

        // don't poll the time unnecessarily
        if counter & 0xfff == 0 {
            let nanos = time.elapsed().as_nanos() as u64;
            // every second
            if nanos > 1_000_000_000 {
                dev.read_stats(&mut dev_stats);
                dev_stats.print_stats_diff(&dev, &dev_stats_old, nanos);
                dev_stats_old = dev_stats;

                time = Instant::now();
            }
        }

        counter += 1;
    }
}

// calculate IP/TCP/UDP checksum
fn calc_ip_checksum(packet: &mut Packet, offset: usize, len: usize) -> u16 {
    let mut checksum = 0;
    for i in 0..len / 2 {
        checksum += ((u32::from(packet[i + offset])) << 8) + u32::from(packet[i + offset + 1]);
        if checksum > 0xffff {
            checksum = (checksum & 0xfff) + 1;
        }
    }
    return !(checksum as u16);
}
