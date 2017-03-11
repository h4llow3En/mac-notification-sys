//! a very thin wrapper around NSNotifications
#![deny(missing_docs,
        missing_copy_implementations,
        trivial_casts, trivial_numeric_casts,
        unused_import_braces, unused_qualifications)]
#![warn(missing_debug_implementations)]

#![cfg(target_os = "macos")]
#![allow(improper_ctypes)]

#[macro_use]
extern crate objc_foundation;
extern crate chrono;
pub mod error;

use std::ops::Deref;
use objc_foundation::{NSString, INSString};
use chrono::prelude::*;
use error::*;
use std::path::PathBuf;
use std::env;

static mut APPLICATION_SET: bool = false;

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
/// Returns a `NotificationError` if a notification could not be scheduled
/// or is scheduled in the past
///
/// # Example:
///
/// ```ignore
/// extern crate chrono;
/// # use mac_notification_sys::*;
/// use chrono::prelude::*;
///
/// // schedule a notification in 5 seconds
/// let _ = schedule_notification("Title", None, "This is the body", Some("Ping"),
///                               UTC::now().timestamp() as f64 + 5.).unwrap();
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
            if check_sound(sound.unwrap()) {
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
/// Returns a `NotificationError` if a notification could not be delivered
///
/// # Example:
///
/// ```no_run
/// # use mac_notification_sys::*;
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
        if check_sound(sound.unwrap()) {
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

/// Search for a possible BundleIdentifier of a given appname.
/// Defaults to "com.apple.Terminal" if no BundleIdentifier is found.
pub fn get_bundle_identifier_or_default(app_name: &str) -> String {
    get_bundle_identifier(app_name).unwrap_or("com.apple.Terminal".to_string())
}

/// Search for a BundleIdentifier of an given appname.
pub fn get_bundle_identifier(app_name: &str) -> Option<String> {
    unsafe {
        sys::getBundleIdentifier(NSString::from_str(app_name).deref()) // *const NSString
            .as_ref() // Option<NSString>
            .map(|nstr| nstr.as_str().to_owned())
    }
}

/// Set the application which delivers or schedules a notification
pub fn set_application(bundle_ident: &str) -> NotificationResult<()> {
    unsafe {
        if APPLICATION_SET {
            Err(ApplicationError::AlreadySet.into())
        } else {
            if sys::setApplication(NSString::from_str(bundle_ident).deref()) {
                Ok(())
            } else {
                Err(ApplicationError::CouldNotSet.into())
            }
        }
    }
}

fn check_sound(sound_name: &str) -> bool {
    let mut file_exists: bool = false;
    let mut sound_paths: Vec<PathBuf> = vec![PathBuf::from("/Library/Sounds/"),
                                             PathBuf::from("/Network/Library/Sounds/"),
                                             PathBuf::from("/System/Library/Sounds/")];
    match env::home_dir() {
        Some(path) => {
            let new_path = PathBuf::from(path).join("/Library/Sounds/");
            sound_paths.insert(0, new_path);
        }
        None => print!("No home path found.", ),
    }
    for mut check_path in sound_paths {
        check_path.push(sound_name);
        check_path.push(".aiff");
        if check_path.exists() {
            file_exists = true;
        }
    }
    file_exists
}
