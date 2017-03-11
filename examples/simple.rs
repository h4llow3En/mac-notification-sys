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
