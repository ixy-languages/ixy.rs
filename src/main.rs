#![feature(asm)]
#![feature(const_fn)]
#![feature(untagged_unions)]

extern crate libc;

#[allow(dead_code)]
#[allow(non_snake_case)]
#[allow(non_camel_case_types)]
#[allow(non_upper_case_globals)]
mod constants;

use std::fs;
use std::ptr;
use std::os::unix::prelude::AsRawFd;
use constants::*;

use std::thread;
use std::time::Duration;

struct ixgbe_device {
    addr: usize,
}


fn main() {
    // TODO
    unbind_driver();

    //let path = format!("{}", "README.md");
    let path = format!("/sys/bus/pci/devices/{}/resource0", "0000:03:00.1");

    let addr = pci_map(&path);
    let ixgbe = ixgbe_device { addr };

    /*for i in 0..255 {
        unsafe {
            println!("{:x}", read_reg32(ixgbe.addr, i*4));
        }
    }*/

    print!("Link speed: ");
    get_link_speed(&ixgbe);

    reset_and_init(&ixgbe);
    //get_link_speed(&ixgbe);
}

/* should return OK or ERR ?! */
fn pci_map(path: &str) -> usize {
    let f = fs::OpenOptions::new()
        .read(true)
        .write(true)
        .open(path)
        .expect("Unable to open file");
    let size = fs::metadata(path).expect("A").len();

    println!("File size: {} B", size);

    let addr = unsafe {
        let data = libc::mmap(
            ptr::null_mut(),
            size as usize,
            libc::PROT_READ | libc::PROT_WRITE,
            libc::MAP_SHARED,
            f.as_raw_fd(),
            0,
        ) as *mut u8;

        if data.is_null() {
            panic!("Could not access data from memory mapped file")
        };

        println!("Address: {:x}", data as usize);

        data as usize
    };

    addr
}

fn reset_and_init(ixgbe: &ixgbe_device) {
    unsafe {
        // section 4.6.3.1 - disable all interrupts
        set_reg32(ixgbe.addr, IXGBE_EIMC, 0x7FFFFFFF);

        // section 4.6.3.2
        set_reg32(ixgbe.addr, IXGBE_CTRL, IXGBE_CTRL_RST_MASK);
        wait_clear_reg32(ixgbe.addr, IXGBE_CTRL, IXGBE_CTRL_RST_MASK);
        thread::sleep(Duration::from_millis(10));

        // section 4.6.3.1 - disable interrupts again after reset
        set_reg32(ixgbe.addr, IXGBE_EIMC, 0x7FFFFFFF);

        println!("Initializing device");

        // section 4.6.3 - wait for EEPROM auto read completion
        wait_set_reg32(ixgbe.addr, IXGBE_EEC, IXGBE_EEC_ARD);

        println!("DMA");

        // section 4.6.3 - wait for dma initialization done
        wait_set_reg32(ixgbe.addr, IXGBE_RDRXCTL, IXGBE_RDRXCTL_DMAIDONE);

        println!("Initializing link");

        // section 4.6.4 - initialize link (auto negotiation)
        init_link(dev);
    }
}

// see section 4.6.4
fn init_link(ixgbe: &ixgbe_device) {

}

fn get_link_speed(ixgbe: &ixgbe_device) {
    unsafe {
        let speed = read_reg32(ixgbe.addr, IXGBE_LINKS);
        println!("{:x}", speed);
        match speed & IXGBE_LINKS_SPEED_82599 {
            IXGBE_LINKS_SPEED_100_82599 => println!("100 Mbit/s"),
            IXGBE_LINKS_SPEED_1G_82599 => println!("1 Gbit/s"),
            IXGBE_LINKS_SPEED_10G_82599 => println!("10 Gbit/s"),
            _ => println!("Something went wrong :(")
        }
    }
}

unsafe fn read_reg32(base: usize, register: u32) -> u32 {
    ptr::read_volatile((base + register as usize) as *mut u32)
}

unsafe fn set_reg32(data: usize, register: u32, value: u32) {
    ptr::write_volatile((data + register as usize) as *mut u32, value);
}

unsafe fn wait_clear_reg32(data: usize, register: u32, value: u32) {
    //asm!("" :::: "volatile" : "memory");
    loop {
        let current = ptr::read_volatile((data + register as usize) as *mut u32);
        if (current & value) == 0 {
            break;
        }
        println!("Register: {:x}, current: {:x}, value: {:x}, expected: {:x}", register, current, value, 0);
        thread::sleep(Duration::from_millis(100));
        //asm!("" :::: "volatile" : "memory");
    }
}

unsafe fn wait_set_reg32(data: usize, register: u32, value: u32) {
    //asm!("" :::: "volatile" : "memory");
    loop {
        let current = ptr::read_volatile((data + register as usize) as *mut u32);
        if (current & value) == value {
            break;
        }
        println!("Register: {:x}, current: {:x}, value: {:x}, expected: ~{:x}", register, current, value, value);
        thread::sleep(Duration::from_millis(100));
        //asm!("" :::: "volatile" : "memory");
    }
}

/* TODO
 * echo -n "0000:02:00.1" > /sys/bus/pci/drivers/igb_uio/unbind
 * echo -n "0000:03:00.1" > /sys/bus/pci/drivers/ixgbe/unbind
 */
fn unbind_driver() {

}