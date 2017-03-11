extern crate mac_notification_sys;
use mac_notification_sys::*;

fn main() {
    let bundle = get_bundle_identifier("firefox");
    let _ = set_application(&bundle).unwrap();
    let _sent = send_notification("Danger",
                                  Some("Will Robinson"),
                                  "Run away as fast as you can",
                                  Some("Blow"));
    let _ = send_notification("NOW", None, "Without subtitle", Some("Submarine"));

}
