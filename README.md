# mac-notification-sys

[![license](https://img.shields.io/crates/l/mac-notification-sys.svg)](https://crates.io/crates/mac-notification-sys/)
[![version](https://img.shields.io/crates/v/mac-notification-sys.svg)](https://crates.io/crates/mac-notification-sys/)
[![Build Status](https://travis-ci.com/h4llow3En/mac-notification-sys.svg?token=nfC1sQZDhGQq93RfYx3k&branch=master)](https://travis-ci.com/h4llow3En/mac-notification-sys)

A simple wrapper to deliver or schedule macOS Notifications in Rust.

## Usage

```toml
#Cargo.toml
[dependencies]
mac-notification-sys = "0.1.0"
```

## Example

```rust
extern crate mac_notification_sys;
use mac_notification_sys::*;

fn main() {
    let bundle = get_bundle_identifier_or_default("firefox");
    set_application(&bundle).unwrap();
    send_notification("Danger",
                      Some("Will Robinson"),
                      "Run away as fast as you can",
                      Some("Blow"))
        .unwrap();
    send_notification("NOW", None, "Without subtitle", Some("Submarine")).unwrap();

}
```
