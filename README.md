# ixy.rs

ixy.rs is a Rust rewrite of the [ixy](https://github.com/emmericp/ixy) userspace network driver.
It is designed to be readable, idiomatic Rust code.
It supports Intel 82599 10GbE NICs (`ixgbe` family).

## Features

* tbd

## Build instructions

```
make
make install
```

You will need `tbd`. Install using:

```
tbd
```

## Usage

tbd

### API

`lib/ixy.rs` defines ixy.rs's public API.

### Example

`example/some_example.rs` is some example.

## Internals

`lib/ixy.rs` contains the core logic.

## License

ixy.rs is licensed under the MIT license.

## Disclaimer

ixy.rs is not production-ready.
Do not use it in critical environments.
DMA may corrupt memory.

## Other languages

Check out the [other ixy implementations](https://github.com/ixy-languages).
