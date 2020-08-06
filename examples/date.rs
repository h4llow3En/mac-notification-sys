use chrono::offset::*;
use mac_notification_sys::*;

fn main() {
    let stamp = Utc::now().timestamp() as f64 + 5.;
    println!("{:?}", stamp);
    let bundle = get_bundle_identifier_or_default("firefox");
    set_application(&bundle).unwrap();
    send_notification(
        "Danger",
        Some("Will Robinson"),
        "Run away as fast as you can",
        Some(NotificationOptions {
            main_button: None,
            close_button: None,
            app_icon: None,
            content_image: None,
            group_id: None,
            sound: Some("Blow"),
            delivery_date: Some((stamp + 5., false)),
        }),
    )
    .unwrap();
}
