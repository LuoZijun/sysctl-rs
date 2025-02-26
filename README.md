This crate provides a safe interface for reading and writing information to the kernel using the sysctl interface.

[![Current Version](https://img.shields.io/crates/v/sysctl.svg)](https://crates.io/crates/sysctl)


*FreeBSD, Linux and macOS are supported.*
*Contributions for improvements and other platforms are welcome.*

### Documentation

Documentation is available here: https://johalun.github.io/sysctl-rs/

or, to generate documentation locally do:
```sh
$ git clone https://github.com/johalun/sysctl-rs && cd sysctl-rs
$ cargo doc --no-deps
$ firefox target/doc/sysctl/index.html
```

### Usage

Add to `Cargo.toml`

```toml
[dependencies]
sysctl = "0.4.0"
```

### macOS

* Due to limitations in the sysctl(3) API, many of the methods of
  the `Ctl` take a mutable reference to `self` on macOS.
* Sysctl descriptions are not available on macOS and Linux.
* Some tests failures are ignored, as the respective sysctls do not
  exist on macos.

### Example

sysctl comes with several examples, see the examples folder:

* `value.rs`: shows how to get a sysctl value
* `value_as.rs`: parsing values as structures
* `value_string.rs`: parsing values as string. Use this for cross platform compatibility since all sysctls are strings on Linux.
* `value_oid_as.rs`: getting a sysctl from OID constants from the `libc` crate.
* `set_value.rs`: shows how to set a sysctl value
* `struct.rs`: reading data into a struct
* `temperature.rs`: parsing temperatures
* `iterate.rs`: showcases iteration over the sysctl tree

Run with:

```sh
$ cargo run --example iterate
```

Or to use in your program:

```rust
extern crate sysctl;
use sysctl::Sysctl;

fn main() {
    let ctl = sysctl::Ctl::new("kern.osrevision").unwrap();
    println!("Description: {}", ctl.description().unwrap());
    println!("Value: {}", ctl.value_string().unwrap());
}
```

