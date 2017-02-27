//! sound under /System/Library/Sounds
#![allow(improper_ctypes)]
#![allow(unused_imports)]

#[macro_use]
extern crate objc_foundation;

use std::ops::Deref;
use objc_foundation::{NSString, INSString};

mod sys {
    use std::ops::Deref;
    use objc_foundation::{NSString, INSString};
    #[link(name = "notify")]
    extern "C" {
        pub fn scheduleNotification( title: *const NSString,
                                     subtitle: *const NSString,
                                     message: *const NSString,
                                     sound: *const NSString,
                                     deliveryDate: f64) -> bool;
        pub fn sendNotification( title: *const NSString,
                                 subtitle: *const NSString,
                                 message: *const NSString,
                                 sound: *const NSString) -> bool;
        pub fn setApplication(newbundleIdentifier: *const NSString) -> bool;
        pub fn getBundleIdentifier(appName: *const NSString) -> *const NSString;
    }
}

pub mod util {
    use super::sys;
    use std::ops::Deref;
    use objc_foundation::{NSString, INSString};

    pub fn get_bundle_identifier(app_name: &str) -> String {
        get_bundle_identifier_or(app_name, "com.apple.Terminal")
    }

    pub fn get_bundle_identifier_or(app_name: &str, default:&str) -> String {
        unsafe {
            String::from(
                sys::getBundleIdentifier(NSString::from_str(app_name).deref()) // *const NSString
                    .as_ref() // Option<NSString>
                    .map(|nstr| nstr.as_str())
                    .unwrap_or(default)
            )
        }
    }

    /// ACHTUNG
    pub fn set_application(bundle_ident: &str) -> bool {
        unsafe {
            sys::setApplication(NSString::from_str(bundle_ident).deref())
        }
    }
}


pub fn schedule_notification(title: &str, subtitle: &str, message: &str, sound: &str) -> bool{
    unsafe {
        sys::scheduleNotification(NSString::from_str(title).deref(),
                                  NSString::from_str(subtitle).deref(),
                                  NSString::from_str(message).deref(),
                                  NSString::from_str(sound).deref(),
                              0.0
                              )
    }
}

pub fn send_notification(title: &str, subtitle: &str, message: &str, sound: &str) -> bool {
    unsafe {
       sys::sendNotification(NSString::from_str(title).deref(),
                             NSString::from_str(subtitle).deref(),
                             NSString::from_str(message).deref(),
                             NSString::from_str(sound).deref())
    }
}

