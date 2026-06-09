//! A very thin wrapper around NSNotifications
#![deny(deref_nullptr)]
#![deny(invalid_value)]
#![deny(invalid_from_utf8)]
#![deny(never_type_fallback_flowing_into_unsafe)]
#![deny(ptr_to_integer_transmute_in_consts)]
#![deny(static_mut_refs)]
#![warn(
    missing_docs,
    trivial_casts,
    trivial_numeric_casts,
    unused_import_braces,
    unused_qualifications
)]
#![cfg(target_os = "macos")]
#![allow(improper_ctypes)]
// The extern "C" callbacks called from ObjC unavoidably take raw pointer arguments.
// They cannot be marked `unsafe` (Rust forbids unsafe extern "C" fn that are exported),
// yet they must dereference those pointers — suppress the lint crate-wide for this pattern.
#![allow(clippy::not_unsafe_ptr_arg_deref)]

pub mod error;
mod notification;

use error::{ApplicationError, NotificationError, NotificationResult};
pub use notification::{MainButton, Notification, NotificationResponse, Sound};
use objc2_foundation::NSString;
use std::collections::HashMap;
use std::ffi::CStr;
use std::ops::Deref;
use std::os::raw::c_char;
use std::sync::{Arc, Condvar, LazyLock, Mutex, Once};
use std::sync::atomic::{AtomicBool, Ordering};

static INIT_APPLICATION_SET: Once = Once::new();

struct PendingEntry {
    /// Interaction result — written by `complete_notification`, read after ObjC returns.
    result: Mutex<NotificationResponse>,
    /// Set to true once a response callback fires (first-wins).
    done: AtomicBool,
    /// Wakes `rust_wait_for_notification` when `done` becomes true.
    condvar: Condvar,
    /// Set to true once `didDeliverNotification:` fires.
    /// Kept in a separate Mutex+Condvar so the delivery path does not borrow
    /// `result`'s lock and the two wait conditions stay independent.
    delivered: Mutex<bool>,
    delivered_cv: Condvar,
}

static PENDING: LazyLock<Mutex<HashMap<[u8; 16], Arc<PendingEntry>>>> =
    LazyLock::new(|| Mutex::new(HashMap::new()));

fn pending() -> &'static Mutex<HashMap<[u8; 16], Arc<PendingEntry>>> {
    &PENDING
}

/// RAII guard — removes the `PENDING` entry for `id` on drop.
/// Ensures the entry is cleaned up even when `send_notification` panics between
/// the `insert` and the explicit `remove`.
struct PendingGuard {
    id: [u8; 16],
}

impl Drop for PendingGuard {
    fn drop(&mut self) {
        pending().lock().unwrap().remove(&self.id);
    }
}

mod sys {
    use objc2_foundation::{NSDictionary, NSString};
    #[link(name = "notify")]
    unsafe extern "C" {
        pub fn sendNotification(
            title: *const NSString,
            subtitle: *const NSString,
            message: *const NSString,
            options: *const NSDictionary<NSString, NSString>,
            notification_id: *const u8,
            should_wait: bool,
        );
        pub fn setApplication(newbundleIdentifier: *const NSString) -> bool;
        pub fn getBundleIdentifier(appName: *const NSString) -> *const NSString;
        pub fn ensureDelegateInitiated();
    }
}

// --- Rust callbacks called from ObjC delegate ---

unsafe fn str_from_ptr<'a>(ptr: *const c_char) -> Option<&'a str> {
    if ptr.is_null() {
        return None;
    }
    unsafe { CStr::from_ptr(ptr) }.to_str().ok()
}

unsafe fn uuid_from_ptr(ptr: *const u8) -> Option<[u8; 16]> {
    if ptr.is_null() {
        return None;
    }
    unsafe { std::slice::from_raw_parts(ptr, 16) }.try_into().ok()
}

fn complete_notification(id: &[u8; 16], response: NotificationResponse) {
    if let Some(entry) = pending().lock().unwrap().get(id).cloned() {
        // Acquire result lock BEFORE checking done so the check-and-set is atomic.
        // Without this, two concurrent callers could both observe done=false,
        // both enter the block, and the second would overwrite the first's result.
        let mut result = entry.result.lock().unwrap();
        if !entry.done.load(Ordering::Acquire) {
            *result = response;
            entry.done.store(true, Ordering::Release);
            entry.condvar.notify_all();
        }
    }
}

/// Called from ObjC delegate when a notification is activated (clicked/replied/action).
/// `activation_type`: 0=none, 1=actionClicked, 2=contentsClicked, 3=replied
/// `_action_value_index`: selected dropdown index, or -1 if not applicable
#[unsafe(no_mangle)]
pub extern "C" fn rust_notification_activated(
    uuid: *const u8,
    activation_type: u8,
    action_value: *const c_char,
    _action_value_index: i64,
) {
    let _ = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        let id = match unsafe { uuid_from_ptr(uuid) } {
            Some(b) => b,
            None => return,
        };
        let action_value = unsafe { str_from_ptr(action_value) }.unwrap_or("").to_owned();

        log::debug!("notification activated: type={activation_type}");

        let response = match activation_type {
            1 => NotificationResponse::ActionButton(action_value),
            2 => NotificationResponse::Click,
            3 => NotificationResponse::Reply(action_value),
            _ => NotificationResponse::None,
        };
        complete_notification(&id, response);
    }));
}

/// Called from ObjC delegate when a notification is dismissed via the close button.
#[unsafe(no_mangle)]
pub extern "C" fn rust_notification_dismissed(
    uuid: *const u8,
    button_title: *const c_char,
) {
    let _ = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        let id = match unsafe { uuid_from_ptr(uuid) } {
            Some(b) => b,
            None => return,
        };
        let title = unsafe { str_from_ptr(button_title) }.unwrap_or("").to_owned();
        log::debug!("notification dismissed");
        complete_notification(&id, NotificationResponse::CloseButton(title));
    }));
}

/// Called from ObjC when a notification disappears without explicit user interaction.
#[unsafe(no_mangle)]
pub extern "C" fn rust_notification_auto_dismissed(uuid: *const u8) {
    let _ = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        let id = match unsafe { uuid_from_ptr(uuid) } {
            Some(b) => b,
            None => return,
        };
        log::debug!("notification auto-dismissed");
        complete_notification(&id, NotificationResponse::None);
    }));
}

/// Called from ObjC main-thread RunLoop spin to check whether waiting should stop.
#[unsafe(no_mangle)]
pub extern "C" fn rust_notification_is_done(uuid: *const u8) -> bool {
    // On panic return `true` so the RunLoop spin terminates instead of looping forever.
    std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        let id = match unsafe { uuid_from_ptr(uuid) } {
            Some(b) => b,
            None => return true,
        };
        pending()
            .lock()
            .unwrap()
            .get(&id)
            .map(|e| e.done.load(Ordering::Acquire))
            .unwrap_or(true)
    }))
    .unwrap_or(true)
}

/// Called from ObjC background threads to block until the notification completes.
#[unsafe(no_mangle)]
pub extern "C" fn rust_wait_for_notification(uuid: *const u8) {
    // On panic: return immediately so the ObjC background thread isn't stuck forever.
    let _ = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        let id = match unsafe { uuid_from_ptr(uuid) } {
            Some(b) => b,
            None => return,
        };
        let entry = match pending().lock().unwrap().get(&id).cloned() {
            Some(e) => e,
            None => return,
        };
        let guard = entry.result.lock().unwrap();
        let _unused = entry
            .condvar
            .wait_while(guard, |_| !entry.done.load(Ordering::Acquire));
    }));
}

/// Called from the ObjC delegate when `NSUserNotificationCenter` confirms delivery via
/// `didDeliverNotification:`. Used by fire-and-forget sends to ensure the process stays
/// alive until the notification actually reaches the notification daemon.
#[unsafe(no_mangle)]
pub extern "C" fn rust_notification_delivered(uuid: *const u8) {
    let _ = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        let id = match unsafe { uuid_from_ptr(uuid) } {
            Some(b) => b,
            None => return,
        };
        if let Some(entry) = pending().lock().unwrap().get(&id).cloned() {
            *entry.delivered.lock().unwrap() = true;
            entry.delivered_cv.notify_all();
        }
    }));
}

/// Polled from the ObjC main-thread run-loop spin (fire-and-forget path) to check
/// whether `didDeliverNotification:` has fired. Returns `true` on unknown uuid or
/// panic so the spin terminates instead of looping forever.
#[unsafe(no_mangle)]
pub extern "C" fn rust_notification_is_delivered(uuid: *const u8) -> bool {
    std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        let id = match unsafe { uuid_from_ptr(uuid) } {
            Some(b) => b,
            None => return true,
        };
        pending()
            .lock()
            .unwrap()
            .get(&id)
            .map(|e| *e.delivered.lock().unwrap())
            .unwrap_or(true)
    }))
    .unwrap_or(true)
}

/// Called from ObjC background threads (fire-and-forget path): block until
/// `didDeliverNotification:` fires, bounded by a 2-second safety timeout so the
/// caller can't hang indefinitely if the callback never arrives.
#[unsafe(no_mangle)]
pub extern "C" fn rust_wait_for_delivery(uuid: *const u8) {
    // On panic: return immediately so the ObjC background thread isn't stuck forever.
    let _ = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        let id = match unsafe { uuid_from_ptr(uuid) } {
            Some(b) => b,
            None => return,
        };
        let entry = match pending().lock().unwrap().get(&id).cloned() {
            Some(e) => e,
            None => return,
        };
        let guard = entry.delivered.lock().unwrap();
        let _ = entry.delivered_cv.wait_timeout_while(
            guard,
            std::time::Duration::from_secs(2),
            |delivered| !*delivered,
        );
    }));
}

// --- Public API ---

/// Delivers a new notification
///
/// Returns a `NotificationError` if a notification could not be delivered
///
/// # Example:
///
/// ```no_run
/// # use mac_notification_sys::*;
/// // deliver a silent notification
/// let _ = send_notification("Title", None, "This is the body", None).unwrap();
/// ```
pub fn send_notification(
    title: &str,
    subtitle: Option<&str>,
    message: &str,
    options: Option<&Notification>,
) -> NotificationResult<NotificationResponse> {
    if let Some(options) = &options {
        if let Some(delivery_date) = options.delivery_date {
            ensure!(
                delivery_date >= time::OffsetDateTime::now_utc().unix_timestamp() as f64,
                NotificationError::ScheduleInThePast
            );
        }
    };

    ensure_application_set()?;
    ensure_delegate_initiated();

    let should_wait = options.map(|o| o.needs_response()).unwrap_or(false);
    let options_dict = options.unwrap_or(&Notification::new()).to_dictionary();

    let id: [u8; 16] = uuid::Uuid::new_v4().into_bytes();

    let entry = Arc::new(PendingEntry {
        result: Mutex::new(NotificationResponse::None),
        done: AtomicBool::new(!should_wait),
        condvar: Condvar::new(),
        delivered: Mutex::new(false),
        delivered_cv: Condvar::new(),
    });
    pending().lock().unwrap().insert(id, Arc::clone(&entry));
    // PendingGuard performs the sole remove — both on the normal path and on panic.
    // Reading the result from `entry` directly avoids a second remove call.
    let _guard = PendingGuard { id };

    unsafe {
        sys::sendNotification(
            NSString::from_str(title).deref(),
            NSString::from_str(subtitle.unwrap_or("")).deref(),
            NSString::from_str(message).deref(),
            options_dict.deref(),
            id.as_ptr(),
            should_wait,
        );
    }

    let result = entry.result.lock().unwrap().clone();
    Ok(result)
}

/// Search for a possible BundleIdentifier of a given appname.
/// Defaults to "com.apple.Finder" if no BundleIdentifier is found.
pub fn get_bundle_identifier_or_default(app_name: &str) -> String {
    get_bundle_identifier(app_name).unwrap_or_else(|| "com.apple.Finder".to_string())
}

/// Search for a BundleIdentifier of an given appname.
pub fn get_bundle_identifier(app_name: &str) -> Option<String> {
    unsafe {
        sys::getBundleIdentifier(NSString::from_str(app_name).deref())
            .as_ref()
    }
    .map(NSString::to_string)
}

/// Sets the application if not already set
fn ensure_application_set() -> NotificationResult<()> {
    if INIT_APPLICATION_SET.is_completed() {
        return Ok(());
    };
    let bundle = get_bundle_identifier_or_default("use_default");
    set_application(&bundle)
}

fn ensure_delegate_initiated() {
    // `sharedDelegate` in ObjC is already guarded by `dispatch_once`; calling it here
    // is idempotent and thread-safe without an extra Rust-side Once.
    unsafe { sys::ensureDelegateInitiated() };
}

/// Set the application which delivers or schedules a notification
pub fn set_application(bundle_ident: &str) -> NotificationResult<()> {
    let mut result = Err(ApplicationError::AlreadySet(bundle_ident.into()).into());
    INIT_APPLICATION_SET.call_once(|| {
        let was_set = unsafe { sys::setApplication(NSString::from_str(bundle_ident).deref()) };
        result = if was_set {
            Ok(())
        } else {
            Err(ApplicationError::CouldNotSet(bundle_ident.into()).into())
        };
    });
    result
}

// ---- Bridge tests (no OS, no ObjC) -------------------------------------------
#[cfg(test)]
mod tests {
    use super::*;
    use std::ffi::CString;

    /// Insert a bare entry into PENDING so tests can drive callbacks directly.
    fn insert_test_entry(id: [u8; 16]) -> Arc<PendingEntry> {
        let entry = Arc::new(PendingEntry {
            result: Mutex::new(NotificationResponse::None),
            done: AtomicBool::new(false),
            condvar: Condvar::new(),
            delivered: Mutex::new(false),
            delivered_cv: Condvar::new(),
        });
        pending().lock().unwrap().insert(id, Arc::clone(&entry));
        entry
    }

    fn remove_entry(id: [u8; 16]) -> Option<Arc<PendingEntry>> {
        pending().lock().unwrap().remove(&id)
    }

    // --- needs_response truth table ---

    #[test]
    fn needs_response_false_by_default() {
        assert!(!Notification::new().needs_response());
    }

    #[test]
    fn needs_response_true_for_main_button() {
        assert!(
            Notification::new()
                .main_button(MainButton::SingleAction("ok"))
                .needs_response()
        );
    }

    #[test]
    fn needs_response_true_for_close_button() {
        assert!(Notification::new().close_button("X").needs_response());
    }

    #[test]
    fn needs_response_true_for_wait_for_click() {
        assert!(Notification::new().wait_for_click(true).needs_response());
    }

    #[test]
    fn needs_response_false_for_delivery_date() {
        // Scheduled notifications are fire-and-forget — delivery_date alone must not
        // cause the caller to block waiting for interaction.
        assert!(!Notification::new().delivery_date(1.0).needs_response());
    }

    #[test]
    fn asynchronous_overrides_needs_response() {
        let mut n = Notification::new();
        n.main_button(MainButton::SingleAction("ok"));
        n.asynchronous(true);
        assert!(!n.needs_response());
    }

    // --- round-trip callback tests ---

    #[test]
    fn bridge_reply() {
        let id: [u8; 16] = [0x9a, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1];
        insert_test_entry(id);

        let value = CString::new("hello world").unwrap();

        rust_notification_activated(id.as_ptr(), 3, value.as_ptr(), -1);

        assert!(rust_notification_is_done(id.as_ptr()));
        let entry = remove_entry(id).unwrap();
        assert_eq!(
            *entry.result.lock().unwrap(),
            NotificationResponse::Reply("hello world".into())
        );
    }

    #[test]
    fn bridge_action_button() {
        let id: [u8; 16] = [0x9b, 2, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 2];
        insert_test_entry(id);

        let value = CString::new("Delete").unwrap();

        rust_notification_activated(id.as_ptr(), 1, value.as_ptr(), -1);

        assert!(rust_notification_is_done(id.as_ptr()));
        let entry = remove_entry(id).unwrap();
        assert_eq!(
            *entry.result.lock().unwrap(),
            NotificationResponse::ActionButton("Delete".into())
        );
    }

    #[test]
    fn bridge_close_button() {
        let id: [u8; 16] = [0x9c, 3, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 3];
        insert_test_entry(id);

        let button = CString::new("Nevermind").unwrap();

        rust_notification_dismissed(id.as_ptr(), button.as_ptr());

        assert!(rust_notification_is_done(id.as_ptr()));
        let entry = remove_entry(id).unwrap();
        assert_eq!(
            *entry.result.lock().unwrap(),
            NotificationResponse::CloseButton("Nevermind".into())
        );
    }

    #[test]
    fn bridge_auto_dismissed() {
        let id: [u8; 16] = [0x9d, 4, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 4];
        insert_test_entry(id);

        rust_notification_auto_dismissed(id.as_ptr());

        assert!(rust_notification_is_done(id.as_ptr()));
        let entry = remove_entry(id).unwrap();
        assert_eq!(*entry.result.lock().unwrap(), NotificationResponse::None);
    }

    // --- first-wins guard ---

    #[test]
    fn first_wins_guard() {
        let id: [u8; 16] = [0x9e, 5, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 5];
        insert_test_entry(id);

        let first = CString::new("first").unwrap();
        let second = CString::new("second").unwrap();

        rust_notification_activated(id.as_ptr(), 3, first.as_ptr(), -1);
        // Second call must be ignored because done=true after the first.
        rust_notification_activated(id.as_ptr(), 3, second.as_ptr(), -1);

        let entry = remove_entry(id).unwrap();
        assert_eq!(
            *entry.result.lock().unwrap(),
            NotificationResponse::Reply("first".into())
        );
    }

    // --- delivery confirmation bridge ---

    #[test]
    fn bridge_delivered_fires_signal() {
        let id: [u8; 16] = [0x9f, 6, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 6];
        let entry = insert_test_entry(id);

        assert!(!rust_notification_is_delivered(id.as_ptr()));

        rust_notification_delivered(id.as_ptr());

        assert!(rust_notification_is_delivered(id.as_ptr()));
        assert!(*entry.delivered.lock().unwrap());
        // done must NOT be set — delivered is orthogonal to the interaction/done flow.
        assert!(!entry.done.load(Ordering::Acquire));

        remove_entry(id);
    }

    #[test]
    fn unknown_uuid_is_delivered_returns_true() {
        let unknown: [u8; 16] = [0xff; 16];
        assert!(rust_notification_is_delivered(unknown.as_ptr()));
    }

    #[test]
    fn unknown_uuid_wait_for_delivery_returns_immediately() {
        let unknown: [u8; 16] = [0xfe; 16];
        rust_wait_for_delivery(unknown.as_ptr());
    }

    // --- unknown-uuid safety ---

    #[test]
    fn unknown_uuid_is_done_returns_true() {
        let unknown: [u8; 16] = [0xfd; 16];
        assert!(rust_notification_is_done(unknown.as_ptr()));
    }

    #[test]
    fn unknown_uuid_complete_is_noop() {
        let unknown: [u8; 16] = [0xfc; 16];
        complete_notification(&unknown, NotificationResponse::None);
    }

    #[test]
    fn unknown_uuid_wait_returns_immediately() {
        let unknown: [u8; 16] = [0xfb; 16];
        rust_wait_for_notification(unknown.as_ptr());
    }
}
