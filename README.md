# ixy.rs
[![MIT licensed](https://img.shields.io/badge/license-MIT-blue.svg)](./LICENSE)
[![](https://tokei.rs/b1/github/ixy-languages/ixy.rs?category=code)](https://github.com/ixy-languages/ixy.rs)
[![](https://tokei.rs/b1/github/ixy-languages/ixy.rs?category=comments)](https://github.com/ixy-languages/ixy.rs)

ixy.rs is a Rust rewrite of the [ixy](https://github.com/emmericp/ixy) userspace network driver.
It is designed to be readable, idiomatic Rust code.
It supports Intel 82599 10GbE NICs (`ixgbe` family).
Check out [our paper](https://www.net.in.tum.de/fileadmin/bibtex/publications/theses/2018-ixy-rust.pdf) to read about the details of our implementation.

## Features

* less than 2000 lines of Rust code for the driver and two sample applications
* simple API to use
* super fast, can forward > 26 million packets per second on a single 3.3 GHz CPU core
* documented code

## Build instructions

You will need Rust and its package manager `cargo`.
Install using:

```
curl https://sh.rustup.rs -sSf | sh -s -- -y
source $HOME/.cargo/env
```

Ixy.rs uses hugepages. To enable them run:

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

## Performance

Have a look at our [performance results](https://github.com/ixy-languages/ixy-languages#Performance) in the ixy-languages repository.

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

