//! Demonstrates distinguishing "dismissed" (close button) from "expired"
//! (auto-dismiss / timed out / system-cleared) on the `NSUserNotificationCenter`
//! stack using only `mac-notification-sys` APIs.
//!
//! Mapping:
//! - `NotificationResponse::CloseButton(_)` -> dismissed by user
//! - `NotificationResponse::None`           -> auto-dismissed / expired
//! - other variants                         -> user interacted, not a close
//!
//! To exercise the "dismissed" path, click the close button on the alert.
//! To exercise the "expired" path, let the notification time out or clear it
//! from Notification Center without clicking either button.

use std::thread;

use mac_notification_sys::*;

#[derive(Debug)]
enum CloseReason {
    Dismissed,
    Expired,
}

fn main() {
    let bundle = get_bundle_identifier_or_default("safari");
    set_application(&bundle).unwrap();

    on_close();

    let bg_thread = thread::spawn(|| {
        on_close();
    });

    run_main_loop_while(bg_thread).unwrap();
}

fn on_close() {
    let response = Notification::default()
        .title("on_close demo")
        .subtitle("Click the close button, or let it auto-dismiss")
        .message("waiting for close...")
        .close_button("Close")
        .send()
        .unwrap();

    let close_reason = match response {
        NotificationResponse::CloseButton(ref label) => {
            println!("❎ close button clicked: {label:?}");
            Some(CloseReason::Dismissed)
        }
        NotificationResponse::None => {
            println!("⏰ auto-dismissed (no interaction)");
            Some(CloseReason::Expired)
        }
        other => {
            println!("⚠️ user interacted, not a close: {other:?}");
            None
        }
    };

    if let Some(reason) = close_reason {
        println!("on_close -> {reason:?}");
    }
}

pub fn run_main_loop_while<T>(thread: thread::JoinHandle<T>) -> thread::Result<T> {
    use objc2_foundation::{NSDate, NSDefaultRunLoopMode, NSRunLoop};
    let run_loop = NSRunLoop::mainRunLoop();
    while !thread.is_finished() {
        let until = NSDate::dateWithTimeIntervalSinceNow(0.05);
        unsafe { run_loop.runMode_beforeDate(NSDefaultRunLoopMode, &until) };
    }
    thread.join()
}
