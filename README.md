# mac-notification-sys

[![build](https://img.shields.io/github/workflow/status/h4llow3En/mac-notification-sys/Continuous%20Integration)](https://github.com/h4llow3En/mac-notification-sys/actions?query=workflow%3A"Continuous+Integration")
![Crates.io](https://img.shields.io/badge/platform-macOS-lightgrey?style=flat-square)
[![license](https://img.shields.io/crates/l/mac-notification-sys?style=flat-square)](https://crates.io/crates/mac-notification-sys/)
[![version](https://img.shields.io/crates/v/mac-notification-sys?style=flat-square)](https://crates.io/crates/mac-notification-sys/)
![Crates.io](https://img.shields.io/crates/d/mac-notification-sys?style=flat-square)


A simple wrapper to deliver or schedule macOS Notifications in Rust.

## Usage

```toml
#Cargo.toml
[dependencies]
mac-notification-sys = "0.3"
```

## Documentation

The documentation can be found [here](https://h4llow3en.github.io/mac-notification-sys/mac_notification_sys/)

## Example

```rust
use mac_notification_sys::*;

fn main() {
    let bundle = get_bundle_identifier_or_default("firefox");
    set_application(&bundle).unwrap();

    send_notification(
        "Danger",
        Some("Will Robinson"),
        "Run away as fast as you can",
        None,
    )
    .unwrap();

    send_notification(
        "NOW",
        None,
        "Without subtitle",
        Some(Notification::new().sound("Blow")),
    )
    .unwrap();
}

```

## TODO

- [ ] Add timeout option so notifications can be auto-closed
- [ ] Allow NSDictionary to hold various types (perhaps with a union?)
- [ ] Switch to UserNotification if possible

## Contributors

 Thanks goes to these wonderful people:
 - [@hoodie](https://github.com/hoodie)
 - [@PandawanFr](https://github.com/PandawanFr)
