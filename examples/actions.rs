use mac_notification_sys::*;

fn main() {
    let response = send_notification(
        "main button with drop down",
        None,
        "choose wisely",
        Some(Notification::new().main_button(MainButton::DropdownActions(
            "Dropdown",
            &["Action 1", "Action 2"],
        ))),
    )
    .unwrap();
    handle_repsonse(response);

    let response = send_notification(
        "take response",
        None,
        "type what you want",
        Some(Notification::new().main_button(MainButton::Response(r#"you want "foobar""#))),
    )
    .unwrap();
    handle_repsonse(response);

    let response = send_notification(
        "Single Action",
        None,
        "ok?",
        Some(Notification::new().main_button(MainButton::SingleAction("Ok"))),
    )
    .unwrap();
    handle_repsonse(response);

    let response = send_notification(
        "close button only",
        None,
        "close it well",
        Some(Notification::new().close_button("Nevermind...")),
    )
    .unwrap();
    handle_repsonse(response);
}

fn handle_repsonse(response: NotificationResponse) {
    match dbg!(response) {
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
