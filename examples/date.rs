extern crate macos_notifications_sys;
extern crate chrono;
use chrono::prelude::*;
use macos_notifications_sys::*;

fn main() {
    let stamp = UTC::now().timestamp() as f64 + 5.;
    println!("{:?}", stamp);
    let bundle = util::get_bundle_identifier("firefox");
    let _ = util::set_application(&bundle).unwrap();
    let _sent = schedule_notification("Danger",
                                      Some("Will Robinson"),
                                      "Run away as fast as you can",
                                      Some("Blow"),
                                      stamp + 5.).unwrap();

}
