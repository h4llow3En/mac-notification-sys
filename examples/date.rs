extern crate mac_notification_sys;
extern crate chrono;
use chrono::prelude::*;
use mac_notification_sys::*;

fn main() {
    let stamp = UTC::now().timestamp() as f64 + 5.;
    println!("{:?}", stamp);
    let bundle = get_bundle_identifier_or_default("firefox");
    set_application(&bundle).unwrap();
    schedule_notification("Danger",
                          Some("Will Robinson"),
                          "Run away as fast as you can",
                          Some("Blow"),
                          stamp + 5.)
        .unwrap();

}
