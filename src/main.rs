extern crate mac_notification_sys;
use mac_notification_sys::*;

fn main() {
    let bundle = get_bundle_identifier_or_default("Safari");
    set_application(&bundle).unwrap();
    send_notification(
        "Danger",
        &Some("Will Robinson"),
        "Run away as fast as you can",
        &Some("Blow"),
        &Some(NotificationOptions {
            action_button_title: Some("MAIN"),
            other_button_title: Some("ALT"),
            app_icon: Some("/Applications/Discord.app/Contents/Resources/electron.icns"),
        }),
    )
    .unwrap();
}
