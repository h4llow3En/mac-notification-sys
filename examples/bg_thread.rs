use mac_notification_sys::*;
use objc2_foundation::{NSDate, NSRunLoop};

fn main() {
    let bundle = get_bundle_identifier_or_default("safari");
    set_application(&bundle).unwrap();

    let handle = std::thread::spawn(|| {
        send_notification(
            "Background thread notification",
            Some("bg_thread example"),
            "Click the action button",
            Some(Notification::new().main_button(MainButton::SingleAction("OK"))),
        )
        .unwrap()
    });

    // callbacks land on the main thread, so keep the run loop going instead of blocking with join()
    let run_loop = NSRunLoop::mainRunLoop();
    while !handle.is_finished() {
        run_loop.runUntilDate(&NSDate::dateWithTimeIntervalSinceNow(0.1));
    }

    let response = handle.join().unwrap();
    println!("{:?}", response);
}
