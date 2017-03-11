//! a very thin wrapper around NSNotifications
#![cfg(target_os = "macos")]
#![allow(improper_ctypes)]

#[macro_use]
extern crate objc_foundation;
extern crate chrono;
mod error;
pub mod util;

use std::ops::Deref;
use objc_foundation::{NSString, INSString};
use chrono::prelude::*;
use error::*;

mod sys {
    use objc_foundation::NSString;
    #[link(name = "notify")]
    extern "C" {
        pub fn scheduleNotification(title: *const NSString,
                                    subtitle: *const NSString,
                                    message: *const NSString,
                                    sound: *const NSString,
                                    deliveryDate: f64)
                                    -> bool;
        pub fn sendNotification(title: *const NSString,
                                subtitle: *const NSString,
                                message: *const NSString,
                                sound: *const NSString)
                                -> bool;
        pub fn setApplication(newbundleIdentifier: *const NSString) -> bool;
        pub fn getBundleIdentifier(appName: *const NSString) -> *const NSString;
    }
}

/// Schedules a new notification in the NotificationCenter
///
/// # Example:
/// ```rust
/// extern crate macos_notifications_sys;
/// use macos_notifications_sys::*;
/// extern crate chrono;
/// use chrono::prelude::*;
///
/// // schedule a notification in 5 seconds
/// let _ = schedule_notification("Title", None, "This is the body", Some("Ping"), UTC::now().timestamp() as f64 + 5.).unwrap();
/// ```
pub fn schedule_notification(title: &str,
                             subtitle: Option<&str>,
                             message: &str,
                             sound: Option<&str>,
                             delivery_date: f64)
                             -> NotificationResult<()> {
    if UTC::now().timestamp() as f64 >= delivery_date {
        Err(NotificationError::ScheduleInThePast.into())
    } else {
        let mut use_sound: &str = "_mute";
        if sound.is_some() {
            if util::check_sound(sound.unwrap()) {
                use_sound = sound.unwrap();
            }
        }
        unsafe {
            if sys::scheduleNotification(NSString::from_str(title).deref(),
                                         NSString::from_str(subtitle.unwrap_or("")).deref(),
                                         NSString::from_str(message).deref(),
                                         NSString::from_str(use_sound).deref(),
                                         delivery_date) {
                Ok(())
            } else {
                Err(NotificationError::UnableToSchedule.into())
            }
        }
    }
}

/// Delivers a new notification
///
/// # Example:
/// ```rust
/// extern crate macos_notifications_sys;
/// use macos_notifications_sys::*;
///
/// // daliver a silent notification
/// let _ = send_notification("Title", None, "This is the body", None).unwrap();
/// ```
pub fn send_notification(title: &str,
                         subtitle: Option<&str>,
                         message: &str,
                         sound: Option<&str>)
                         -> NotificationResult<()> {
    let mut use_sound: &str = "_mute";
    if sound.is_some() {
        if util::check_sound(sound.unwrap()) {
            use_sound = sound.unwrap();
        }
    }
    unsafe {
        if sys::sendNotification(NSString::from_str(title).deref(),
                                 NSString::from_str(subtitle.unwrap_or("")).deref(),
                                 NSString::from_str(message).deref(),
                                 NSString::from_str(use_sound).deref()) {
            Ok(())
        } else {
            Err(NotificationError::UnableToDeliver.into())
        }
    }
}
