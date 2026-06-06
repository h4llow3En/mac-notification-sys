//! Reproducer for https://github.com/h4llow3En/mac-notification-sys/issues/86
//!
//! issue:
//! `wait_for_click(true)` busy-spins an empty Cocoa run loop, pinning one CPU core per un-actioned notification.
//!
//! how to use:
//! Run this example and *do not* click or dismiss any of the notifications.
//! Open Activity Monitor (or run `top -pid <PID>`) and watch the process CPU usage climb by ~100% for every notification that appears.
//! After all three notifications have been delivered you should see roughly 300% CPU,
//! with three threads each spinning inside:
//!

use mac_notification_sys::*;
use std::thread;
use std::time::Duration;

const NOTIFICATION_COUNT: usize = 3;
const SPIN_UP_DELAY: Duration = Duration::from_secs(2);
const TEST_DURATION: Duration = Duration::from_secs(30);

fn main() {
    let bundle = get_bundle_identifier_or_default("safari");
    set_application(&bundle).unwrap();

    println!(
        "Spawning {NOTIFICATION_COUNT} threads, each sending one notification with \
         wait_for_click(true)."
    );
    println!("Do NOT click or dismiss the notifications.");
    println!(
        "After ~{} seconds, check Activity Monitor or `top` for this process.",
        SPIN_UP_DELAY.as_secs()
    );

    let mut handles = Vec::with_capacity(NOTIFICATION_COUNT);

    for index in 0..NOTIFICATION_COUNT {
        let handle = thread::spawn(move || {
            // A short stagger so the banners appear one-by-one and are easier to observe.
            thread::sleep(Duration::from_millis(index as u64 * 500));

            let result = Notification::new()
                .title("busy-spin reproducer")
                .message(&format!("Notification {}", index + 1))
                .main_button(MainButton::DropdownActions(
                    "Actions",
                    &["Option A", "Option B"],
                ))
                .wait_for_click(true)
                .send();

            match result {
                Ok(response) => println!("Thread {index}: got response: {response:?}"),
                Err(err) => eprintln!("Thread {index}: send error: {err}"),
            }
        });

        handles.push(handle);
    }

    // Give all threads time to spin up so the CPU spike is fully visible.
    thread::sleep(SPIN_UP_DELAY);

    println!(
        "\nAll {NOTIFICATION_COUNT} notifications delivered and waiting. \
         You should now observe ~{:.0}% CPU in Activity Monitor.",
        NOTIFICATION_COUNT as f64 * 100.0
    );
    println!(
        "Keeping the process alive for {} seconds — dismiss a notification to see \
         one thread exit and CPU drop by ~100%.",
        TEST_DURATION.as_secs()
    );

    thread::sleep(TEST_DURATION);

    println!("Observe window elapsed. Waiting for all threads to finish...");
    for handle in handles {
        let _ = handle.join();
    }
    println!("Done.");
}
