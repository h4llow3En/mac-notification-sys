use mac_notification_sys::*;

fn main() {
    let bundle = get_bundle_identifier_or_default("firefox");
    set_application(&bundle).unwrap();

    Notification::default()
        .title("Danger")
        .subtitle("Will Robinson")
        .message("Run away as fast as you can")
        .send()
        .unwrap();

    Notification::default()
        .title("NOW")
        .message("Without subtitle")
        .sound("Submarine")
        .send()
        .unwrap();
}
