# ixy.rs

ixy.rs is a Rust rewrite of the [ixy](https://github.com/emmericp/ixy) userspace network driver.
It is designed to be readable, idiomatic Rust code.
It supports Intel 82599 10GbE NICs (`ixgbe` family).

## Features

* less than 2000 lines of Rust code for the driver and two sample applications
* simple API to use, see this README
* super fast, can saturate a 10 Gbit/s connection with 60 byte packets on a single cpu core

## Build instructions

You will need a nightly version of Rust and its package manager `cargo`.
Install using:

```
curl https://sh.rustup.rs -sSf | sh -s -- -y --default-toolchain nightly
source $HOME/.cargo/env
```

Ixy.rs uses hugepages. To enable them run:

```
cd ixy.rs
sudo ./setup-hugetlbfs.sh
```

You can then either build the binaries and run them manually or use `cargo` to build and run them at once (see **Usage**). To build the binaries run:

```
cargo build --release --all-targets
```

The built binaries are located in `targets/release/examples/`.

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

### Example

`examples` contains all sample applications included in this crate.

## Docs

ixy.rs contains documentation that can be created and viewed by running

```
cargo doc --open
```

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
