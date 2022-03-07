use mac_notification_sys::*;

#[test]
#[should_panic]
fn dont_schedule_in_past() {
    let stamp = time::OffsetDateTime::now_utc().unix_timestamp() as f64 - 5.;
    let _sent = send_notification(
        "Danger",
        Some("Will Robinson"),
        "Run away as fast as you can",
        Some(
            Notification::new()
                .sound("Blow")
                .delivery_date(stamp)
                .asynchronous(true),
        ),
    )
    .unwrap();
}
