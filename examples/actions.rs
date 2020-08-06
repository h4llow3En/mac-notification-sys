use mac_notification_sys::*;

fn main() {
    let bundle = get_bundle_identifier_or_default("firefox");
    set_application(&bundle).unwrap();
    let response = send_notification(
        "Danger",
        Some("Will Robinson"),
        "Run away as fast as you can",
        Some(NotificationOptions {
            main_button: Some(MainButton::DropdownActions(
                "Dropdown",
                &["Action 1", "Action 2"],
            )),
            close_button: Some("Nevermind..."),
            app_icon: None,
            content_image: None,
            group_id: None,
            sound: None,
            delivery_date: None,
        }),
    )
    .unwrap();

    println!("{:?}", response);
}
