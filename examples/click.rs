use mac_notification_sys::*;

fn main() {
    Notification::default()
        .title("Click This Notification")
        .subtitle("This will not close unless you interact")
        .message("believe me")
        .wait_for_click(true)
        .send()
        .unwrap();
}
