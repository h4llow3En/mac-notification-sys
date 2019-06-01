extern crate mac_notification_sys;
use mac_notification_sys::*;

fn main() {
    // not setting the application bundle here, should use default
    // let bundle = get_bundle_identifier_or_default("use_default");
    // set_application(&bundle).unwrap();
    
    send_notification("title", &None, "message", &None).unwrap();
}
