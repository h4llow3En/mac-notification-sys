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

    match response {
        // Requires main_button to be a MainButton::SingleAction or MainButton::DropdownActions
        NotificationResponse::ActionButton(action_name) => {
            if action_name == "Action 1" {
                println!("Clicked on Action 1")
            } else if action_name == "Action 2" {
                println!("Clicked on Action 2")
            }
        }
        NotificationResponse::Click => println!("Clicked on the notification itself"),
        NotificationResponse::CloseButton(close_name) => println!(
            "Dismissed the notification with the close button called {}",
            close_name
        ),
        // Requires main_button to be a MainButton::Response
        NotificationResponse::Reply(response) => {
            println!("Replied to the notification with {}", response)
        }
        NotificationResponse::None => println!("No interaction with the notification occured"),
    };
}
