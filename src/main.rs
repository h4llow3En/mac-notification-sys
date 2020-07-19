extern crate mac_notification_sys;
use mac_notification_sys::*;

fn main() {
    let bundle = get_bundle_identifier_or_default("Safari");
    set_application(&bundle).unwrap();
    send_notification(
        "Danger",
        &Some("Will Robinson"),
        "Run away as fast as you can",
        &Some("Blow"),
        &Some(NotificationOptions {
            main_button: Some(MainButton::DropdownActions("MAIN", &["TEST1", "TEST2"])),
            close_button: Some("ALT"),
            app_icon: Some("/Applications/Discord.app/Contents/Resources/electron.icns"),
            content_image: Some(
                "/Users/migueltenant/Desktop/Screen Shot 2020-06-13 at 6.51.20 PM.png",
            ),
            group_id: Some("test1"),
        }),
    )
    .unwrap();

    send_notification(
        "title",
        &Some("subtitle"),
        "message",
        &None,
        &Some(NotificationOptions {
            main_button: None,
            close_button: None,
            app_icon: None,
            content_image: None,
            group_id: Some("test1"),
        }),
    )
    .unwrap();
}
