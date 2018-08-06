# ixy.rs

ixy.rs is a Rust rewrite of the [ixy](https://github.com/emmericp/ixy) userspace network driver.
It is designed to be readable, idiomatic Rust code.
It supports Intel 82599 10GbE NICs (`ixgbe` family).

## Features

* less than 2000 lines of Rust code for the driver and two example applications
* simple API to use, see this README
* super fast, can saturate a 10 Gbit/s connection with 60 byte packets on a single cpu core

## Build instructions

You will need a nightly version of Rust and its package manager `cargo`.
Install using:

```
curl https://sh.rustup.rs -sSf | sh -s -- -y --default-toolchain nightly
source $HOME/.cargo/env
```

Build the binaries:

```
cd ixy.rs
cargo build --all-targets
```

Ixy.rs uses hugepages so you have to enable them:

```
sudo ./setup-hugetlbfs.sh
```

## Usage

There are two demo applications included in the ixy.rs crate.
You can run the packet generator with

```
sudo cargo run --release --example generator <pci bus id>
```

and the forwarder with

```
sudo cargo run --release --example forwarder <pci bus id1> <pci bus id2>
```

### API

`src/lib.rs` defines ixy.rs's public API.

### Example

`examples` contains all demo applications included in this crate.

## Internals

`src/ixgbe.rs` contains the core logic.

## License

ixy.rs is licensed under the MIT license.

## Disclaimer

ixy.rs is not production-ready.
Do not use it in critical environments.
DMA may corrupt memory.

## Other languages

Check out the [other ixy implementations](https://github.com/ixy-languages).
