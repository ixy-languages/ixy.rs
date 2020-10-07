use std::collections::VecDeque;
use std::fs::File;
use std::io::{self, Write};
use std::time::{SystemTime, UNIX_EPOCH};
use std::{env, process};

use byteorder::{WriteBytesExt, LE};
use ixy::memory::Packet;
use ixy::*;
use simple_logger::SimpleLogger;

const BATCH_SIZE: usize = 32;

pub fn main() -> Result<(), io::Error> {
    SimpleLogger::new().init().unwrap();

    let mut args = env::args().skip(1);

    let (pci_addr, output_file) = match (args.next(), args.next()) {
        (Some(pci_addr), Some(output_file)) => (pci_addr, output_file),
        _ => {
            eprintln!("Usage: cargo run --example pcap <pci bus id> <output file> [n packets]");
            process::exit(1);
        }
    };

    let mut n_packets: Option<usize> = args
        .next()
        .map(|n| n.parse().expect("failed to parse n packets"));
    if let Some(n) = n_packets {
        println!("Capturing {} packets...", n);
    } else {
        println!("Capturing packets...");
    }

    let mut pcap = File::create(output_file)?;

    // pcap header
    pcap.write_u32::<LE>(0xa1b2_c3d4)?; // magic_number
    pcap.write_u16::<LE>(2)?; // version_major
    pcap.write_u16::<LE>(4)?; // version_minor
    pcap.write_i32::<LE>(0)?; // thiszone
    pcap.write_u32::<LE>(0)?; // sigfigs
    pcap.write_u32::<LE>(65535)?; // snaplen
    pcap.write_u32::<LE>(1)?; // network: Ethernet

    let mut dev = ixy_init(&pci_addr, 1, 1, 0).unwrap();

    let mut buffer: VecDeque<Packet> = VecDeque::with_capacity(BATCH_SIZE);
    while n_packets != Some(0) {
        dev.rx_batch(0, &mut buffer, BATCH_SIZE);
        let time = SystemTime::now();
        let time = time.duration_since(UNIX_EPOCH).unwrap();

        for packet in buffer.drain(..) {
            // pcap record header
            pcap.write_u32::<LE>(time.as_secs() as u32)?; // ts_sec
            pcap.write_u32::<LE>(time.subsec_millis())?; // ts_usec
            pcap.write_u32::<LE>(packet.len() as u32)?; // incl_len
            pcap.write_u32::<LE>(packet.len() as u32)?; // orig_len

            pcap.write_all(&packet)?;

            n_packets = n_packets.map(|n| n - 1);
            if n_packets == Some(0) {
                break;
            }
        }
    }

    Ok(())
}
