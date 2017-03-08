extern crate macos_notifications_sys;
use macos_notifications_sys::*;

fn main() {
    let bundle = util::get_bundle_identifier("firefox");
    let _ = util::set_application(&bundle).unwrap();
    let _sent = send_notification("Danger",
                                  Some("Will Robinson"),
                                  "Run away as fast as you can",
                                  Some("Blow"));
    let _ = send_notification("NOW", None, "Without subtitle", Some("Submarine"));

}
