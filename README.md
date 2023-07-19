<div align="center">

# mac-notification-sys

![platform](https://img.shields.io/badge/platform-macOS-lightgrey)
[![version](https://img.shields.io/crates/v/mac-notification-sys)](https://crates.io/crates/mac-notification-sys/)
[![license](https://img.shields.io/crates/l/mac-notification-sys)](https://crates.io/crates/mac-notification-sys/)
[![contributors](https://img.shields.io/github/contributors/h4llow3En/mac-notification-sys)](https://github.com/h4llow3En/mac-notification-sys/graphs/contributors)


[![build](https://img.shields.io/github/workflow/status/h4llow3En/mac-notification-sys/Continuous%20Integration)](https://github.com/h4llow3En/mac-notification-sys/actions?query=workflow%3A"Continuous+Integration")
![downloads](https://img.shields.io/crates/d/mac-notification-sys)
[![documentation](https://img.shields.io/badge/docs-latest-blue.svg)](https://docs.rs/mac-notification-sys/)

</div>

A simple wrapper to deliver or schedule macOS Notifications in Rust.

## Usage

```toml
#Cargo.toml
[dependencies]
mac-notification-sys = "0.5"
```

## Documentation

The documentation can be found [here](https://h4llow3en.github.io/mac-notification-sys/mac_notification_sys/)

## RunLoop

Core Foundation API requiring to run the RunLoop always in the main thread. There will be no effect to the
notification delegates if you ran the NS loop from another thread. `winit` is already running this RunLoop
on their main thread. So you do not have to deal with the run loop if you are using tauri/egui and other
`winit` based application frameworks. Otherwise you have to manually run the run loop by using provided
`run_ns_run_loop_once` method.

## Bundling and Code Signing

You must bundle your application to enable the new User Notification framework. Because
`[UNUserNotificationCenter currentNotificationCenter]` method is looking for the bundle informations and
you will get `bundleProxyForCurrentProcess is nil` error if you didn't run the application from a bundle.

Also you have to code sign the bundle to enable the user notification. Otherwise you will get the
`Notifications are not allowed for this application` error message.

You can use Xcode sandbox entitlements to enable user notifications when you are developing an application.
See the `bundle/run.sh` file to get an idea about the bundling a development app.

## Example

Please refer the `examples/simple.rs` file.

Use below commands to run the example project:-

```
export CERTIFICATE=mycertificatename
./bundle/run.sh
```

`CERTIFICATE` variable should be a name of a self signed certificate. If you have an apple developer id, modify
the script and run again. This command will create the `simple.app` in the project root folder and open it for you.
Also a file called `simple.log` will create with the output.

## TODO

- [ ] Add timeout option so notifications can be auto-closed
- [ ] Allow NSDictionary to hold various types (perhaps with a union?)
- [ ] Switch to UserNotification if possible

## Contributors

 Thanks goes to these wonderful people:
 - [@hoodie](https://github.com/hoodie)
 - [@PandawanFr](https://github.com/PandawanFr)
 - [Didrik Nordström](https://github.com/betamos)

Any help in form of descriptive and friendly [issues](https://github.com/h4llow3En/mac-notification-sys/issues) or comprehensive pull requests are welcome! 


Unless you explicitly state otherwise, any contribution intentionally submitted for inclusion in mac-notification-sys by you, as defined in the Apache-2.0 license, shall be dual licensed as above, without any additional terms or conditions.

### Conventions
The Changelog of this library is generated from its commit log, there any commit message must conform with https://www.conventionalcommits.org/en/v1.0.0/. For simplicity you could make your commits with [convco](https://crates.io/crates/convco).
