use chrono::offset::*;
use mac_notification_sys::*;

#[test]
#[should_panic]
fn dont_schedule_in_past() {
    let stamp = Utc::now().timestamp() as f64 - 5.;
    let _sent = send_notification(
        "Danger",
        Some("Will Robinson"),
        "Run away as fast as you can",
        Some(Notification::new().sound("Blow").delivery_date(stamp, true)),
    )
    .unwrap();
}
