use mac_notification_sys::*;
use objc2_foundation::{NSDate, NSRunLoop};
use std::{thread, time::Duration};

const NOTIFICATION_COUNT: usize = 3;
const SPIN_UP_DELAY: Duration = Duration::from_secs(2);

fn main() {
    let bundle = get_bundle_identifier_or_default("safari");
    set_application(&bundle).unwrap();

    let mut handles = Vec::with_capacity(NOTIFICATION_COUNT);

    for index in 0..NOTIFICATION_COUNT {
        let handle = thread::spawn(move || {
            // A short stagger so the banners appear one-by-one and are easier to observe.
            thread::sleep(Duration::from_millis(index as u64 * 500));

            let result = Notification::new()
                .title("multi action example")
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

    println!("\nAll {NOTIFICATION_COUNT} notifications delivered and waiting.");

    let run_loop = NSRunLoop::mainRunLoop();
    while handles.iter().any(|h| !h.is_finished()) {
        run_loop.runUntilDate(&NSDate::dateWithTimeIntervalSinceNow(0.1));
    }

    println!("Done.");
}
