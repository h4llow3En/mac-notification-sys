use mac_notification_sys::*;

fn main() {
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
