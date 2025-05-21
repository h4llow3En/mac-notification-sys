use mac_notification_sys::*;

fn main() {
    let response = send_notification(
        "clickable notification",
        None,
        "click me",
        Some(Notification::new().click(true)),
    )
    .unwrap();

    if matches!(response, NotificationResponse::Click) {
        println!("Clicked the notification");
    } else {
        println!("No interaction");
    }
}
