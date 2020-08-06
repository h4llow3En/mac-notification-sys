# mac-notification-sys

[![license](https://img.shields.io/crates/l/mac-notification-sys.svg)](https://crates.io/crates/mac-notification-sys/)
[![version](https://img.shields.io/crates/v/mac-notification-sys.svg)](https://crates.io/crates/mac-notification-sys/)
[![Build Status](https://travis-ci.com/h4llow3En/mac-notification-sys.svg?token=nfC1sQZDhGQq93RfYx3k&branch=master)](https://travis-ci.com/h4llow3En/mac-notification-sys)

A simple wrapper to deliver or schedule macOS Notifications in Rust.

## Usage

```toml
#Cargo.toml
[dependencies]
mac-notification-sys = "0.1"
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
        Some(NotificationOptions {
            app_icon: None,
            content_image: None,
            main_button: None,
            close_button: None,
            group_id: None,
            delivery_date: None,
            sound: Some("Submarine"),
        }),
    )
    .unwrap();
}

```

## TODO

- Refactor code to look more like alerter's
- Add timeout option so notifications can be auto-closed
