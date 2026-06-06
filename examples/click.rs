use mac_notification_sys::*;

fn main() {
    let bundle = get_bundle_identifier_or_default("safari");
    set_application(&bundle).unwrap();

    Notification::default()
        .title("Click This Notification")
        .subtitle("This will not close unless you interact")
        .message("believe me")
        .wait_for_click(true)
        .send()
        .unwrap();
}
