# ixy.rs
[![MIT licensed](https://img.shields.io/badge/license-MIT-blue.svg)](./LICENSE)

ixy.rs is a Rust rewrite of the [ixy](https://github.com/emmericp/ixy) userspace network driver.
It is designed to be readable, idiomatic Rust code.
It supports Intel 82599 10GbE NICs (`ixgbe` family).
Check out [my thesis](https://www.net.in.tum.de/fileadmin/bibtex/publications/theses/2018-ixy-rust.pdf) to read about the details of the implementation.

## Features

* driver for Intel NICs in the `ixgbe` family, i.e. the 82599ES family (aka Intel X520)
* driver for `ixgbe` virtual functions, i.e. `ixgbevf` (SR-IOV)
* driver for paravirtualized virtio NICs
* super fast, can forward > 26 million packets per second on a single 3.3 GHz CPU core
* less than 2000 lines of Rust code for the driver and a packet forwarder
* no kernel modules needed (except `vfio-pci` for the IOMMU)
* can run without root privileges (using the IOMMU)
* packet prefetching
* support for multiple device queues
* very few dependencies
* simple API to use
* documented code
* MIT license

## Build instructions

You will need Rust and its package manager `cargo`.
Install using:

```
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y
source $HOME/.cargo/env
```

ixy.rs uses hugepages. To enable them run:

```
cd ixy.rs
sudo ./setup-hugetlbfs.sh
```

To build the provided sample applications and execute them manually run:

```
cargo build --release --all-targets
```

The built binaries are located in `target/release/examples/`.

To build and execute the examples at once see **Usage**.

Usage of sse and prefetching requires `x86` or `x86_64` and `sse` enabled. This
requires extra buildflags to be passed to `cargo`.

```
RUSTFLAGS="-C target-cpu=native -C target-feature=+sse" cargo build --release --all-targets
```

## Using the IOMMU / VFIO
The usage of the IOMMU via the `vfio-pci` driver is implemented for ixgbe devices (Intel X520, X540, and X550).
To use it, you have to:

0. Enable the IOMMU in the BIOS.
	On most Intel machines, the BIOS entry is called `VT-d` and has to be enabled in addition to any other virtualization technique.

1. Enable the IOMMU in the linux kernel.
	Add `intel_iommu=on` to your cmdline (if you are running grub, the file `/etc/default/grub.cfg` contains a `GRUB_CMDLINE_LINUX` where you can add it).

2. Get PCI address, vendor and device ID of each device to be used:
	`lspci -nn | grep Ether` returns something like `05:00.0 Ethernet controller [0200]: Intel Corporation Ethernet Controller 10-Gigabit X540-AT2 [8086:1528] (rev 01)`.
	In this case, `0000:05:00.0` is the PCI Address, and `8086` and `1528` are the vendor and device id, respectively.

3. Unbind all devices to be used from the current driver:
	```
	sudo sh -c 'echo $PCI_ADDRESS > /sys/bus/pci/devices/$PCI_ADDRESS/driver/unbind'
	```

4. Enable the `vfio-pci` driver:
	```
	sudo modprobe vfio-pci
	```

5. Bind the devices to the `vfio-pci` driver:
	```
	sudo sh -c 'echo $VENDOR_ID $DEVICE_ID > /sys/bus/pci/drivers/vfio-pci/new_id'
	```

6. For each device find its IOMMU group and chown the device group file to the user:
	```
	IOMMU_GROUP=$(readlink /sys/bus/pci/devices/$PCI_ADDRESS/iommu_group | awk -F '/' '{print $NF}')
	sudo chown $USER:$GROUP /dev/vfio/$IOMMU_GROUP
	```

6. That's it!
	Now you can compile and run ixy.rs as stated above!

## Performance

Running the forwarder example on a single core of a Xeon E3-1230 v2 CPU @ 3.3 GHz under full bidirectional load at 20 Gbit/s with 64 byte packets, i.e. 2x 14.88 million packets per second (Mpps), yields these throughput results when varying the batch size:
![Performance with different batch sizes, CPU at 3.3 GHz](performance.png)

For a comparison to the other drivers, have a look at the [performance results](https://github.com/ixy-languages/ixy-languages#Performance) in the ixy-languages repository.

## Usage

There are two sample applications included in the ixy.rs crate.
You can run the packet generator with

```
sudo cargo run --release --example generator 0000:AA:BB.C 
```

and the forwarder with

```
sudo cargo run --release --example forwarder 0000:AA:BB.C 0000:AA:BB.D
```

### API

`src/lib.rs` defines ixy.rs's public API.

### Examples

`examples` contains all sample applications included in this crate.

### Internals

`src/ixgbe.rs` contains the core logic.

## Docs

ixy.rs contains documentation that can be created and viewed by running

```
cargo doc --open
```

## License

ixy.rs is licensed under the MIT license.

## Disclaimer

ixy.rs is not production-ready.
Do not use it in critical environments.
DMA may corrupt memory.

## Other languages

Check out the [other ixy implementations](https://github.com/ixy-languages).

