//! a very thin wrapper around NSNotifications
#![cfg(target_os = "macos")]
#![allow(improper_ctypes)]

#[macro_use]
extern crate objc_foundation;
extern crate chrono;
mod error;

use std::ops::Deref;
use objc_foundation::{NSString, INSString};
use chrono::prelude::*;
use error::*;


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

pub mod util {
    use super::sys;
    use error::*;
    use std::ops::Deref;
    use objc_foundation::{NSString, INSString};

    pub fn get_bundle_identifier(app_name: &str) -> String {
        get_bundle_identifier_or(app_name).unwrap_or("com.apple.Terminal".to_string())
    }

    pub fn get_bundle_identifier_or(app_name: &str) -> Option<String> {
        unsafe {
            sys::getBundleIdentifier(NSString::from_str(app_name).deref()) // *const NSString
                .as_ref() // Option<NSString>
                .map(|nstr| nstr.as_str().to_owned())
        }
    }

    /// ACHTUNG
    pub fn set_application(bundle_ident: &str) -> NotificationResult<()> {
        unsafe {
            if super::APPLICATION_SET {
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
}

pub fn schedule_notification(title: &str,
                             subtitle: Option<&str>,
                             message: &str,
                             sound: Option<&str>,
                             delivery_date: f64)
                             -> NotificationResult<()> {
    if UTC::now().timestamp() as f64 >= delivery_date {
        Err(NotificationError::ScheduleInThePast.into())
    } else {
        unsafe {
            if sys::scheduleNotification(NSString::from_str(title).deref(),
                                         NSString::from_str(subtitle.unwrap_or("")).deref(),
                                         NSString::from_str(message).deref(),
                                         NSString::from_str(sound.unwrap_or("_mute")).deref(),
                                         delivery_date) {
                Ok(())
            } else {
                Err(NotificationError::UnableToSchedule.into())
            }
        }
    }
}

pub fn send_notification(title: &str,
                         subtitle: Option<&str>,
                         message: &str,
                         sound: Option<&str>)
                         -> NotificationResult<()> {
    unsafe {
        if sys::sendNotification(NSString::from_str(title).deref(),
                                 NSString::from_str(subtitle.unwrap_or("")).deref(),
                                 NSString::from_str(message).deref(),
                                 NSString::from_str(sound.unwrap_or("_mute")).deref()) {
            Ok(())
        } else {
            Err(NotificationError::UnableToDeliver.into())
        }
    }
}
