extern crate mac_notification_sys;
extern crate chrono;
use chrono::prelude::*;
use mac_notification_sys::*;

fn main() {
    let stamp = UTC::now().timestamp() as f64 + 5.;
    println!("{:?}", stamp);
    let bundle = get_bundle_identifier("firefox");
    let _ = set_application(&bundle).unwrap();
    let _sent = schedule_notification("Danger",
                                      Some("Will Robinson"),
                                      "Run away as fast as you can",
                                      Some("Blow"),
                                      stamp + 5.)
        .unwrap();

}
