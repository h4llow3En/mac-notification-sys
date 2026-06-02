use mac_notification_sys::*;

fn main() {
    let bundle = get_bundle_identifier_or_default("firefox");
    set_application(&bundle).unwrap();

    Notification::default()
        .title("🔔")
        .message("Ping")
        .sound("Ping")
        .send()
        .unwrap();

    Notification::default()
        .title("🐟")
        .message("Submarine")
        .sound("Submarine")
        .send()
        .unwrap();

    Notification::default()
        .title("🥱")
        .message("Default")
        .sound(Sound::Default)
        .send()
        .unwrap();
}
