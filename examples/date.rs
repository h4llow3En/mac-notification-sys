use mac_notification_sys::*;

fn main() {
    let stamp = time::OffsetDateTime::now_utc().unix_timestamp() as f64 + 5.;
    println!("{:?}", stamp);
    let bundle = get_bundle_identifier_or_default("firefox");
    set_application(&bundle).unwrap();
    send_notification(
        "Danger",
        Some("Will Robinson"),
        "Run away as fast as you can",
        Some(Notification::new().sound("Blow").delivery_date(stamp + 5.)),
    )
    .unwrap();
}
