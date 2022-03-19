//! A very thin wrapper around NSNotifications
#![warn(
    missing_docs,
    trivial_casts,
    trivial_numeric_casts,
    unused_import_braces,
    unused_qualifications
)]
#![cfg(target_os = "macos")]
#![allow(improper_ctypes)]

pub mod error;
mod notification;

use error::{ApplicationError, NotificationError, NotificationResult};
pub use notification::{MainButton, Notification, NotificationResponse};
use objc_foundation::{INSDictionary, INSString, NSString};
use std::ops::Deref;
use std::sync::Once;

static mut APPLICATION_SET: bool = false;
static INIT_APPLICATION_SET: Once = Once::new();

mod sys {
    use objc_foundation::{NSDictionary, NSString};
    use objc_id::Id;
    #[link(name = "notify")]
    extern "C" {
        pub fn sendNotification(
            title: *const NSString,
            subtitle: *const NSString,
            message: *const NSString,
            options: *const NSDictionary<NSString, NSString>,
        ) -> Id<NSDictionary<NSString, NSString>>;
        pub fn setApplication(newbundleIdentifier: *const NSString) -> bool;
        pub fn getBundleIdentifier(appName: *const NSString) -> *const NSString;
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
/// // deliver a silent notification
/// let _ = send_notification("Title", None, "This is the body", None).unwrap();
/// ```
// #[deprecated(note="use `Notification::send`")]
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

    let options = options.unwrap_or(&Notification::new()).to_dictionary();

    unsafe {
        if !APPLICATION_SET {
            let bundle = get_bundle_identifier_or_default("use_default");
            set_application(&bundle).unwrap();
        }
        let dictionary_response = sys::sendNotification(
            NSString::from_str(title).deref(),
            NSString::from_str(subtitle.unwrap_or("")).deref(),
            NSString::from_str(message).deref(),
            options.deref(),
        );
        ensure!(
            dictionary_response
                .deref()
                .object_for(NSString::from_str("error").deref())
                .is_none(),
            NotificationError::UnableToDeliver
        );

        let response = NotificationResponse::from_dictionary(dictionary_response);

        Ok(response)
    }
}

/// Search for a possible BundleIdentifier of a given appname.
/// Defaults to "com.apple.Finder" if no BundleIdentifier is found.
pub fn get_bundle_identifier_or_default(app_name: &str) -> String {
    get_bundle_identifier(app_name).unwrap_or_else(|| "com.apple.Finder".to_string())
}

/// Search for a BundleIdentifier of an given appname.
pub fn get_bundle_identifier(app_name: &str) -> Option<String> {
    unsafe {
        sys::getBundleIdentifier(NSString::from_str(app_name).deref()) // *const NSString
            .as_ref() // Option<NSString>
            .map(NSString::as_str)
            .map(String::from)
    }
}

/// Set the application which delivers or schedules a notification
pub fn set_application(bundle_ident: &str) -> NotificationResult<()> {
    unsafe {
        ensure!(
            !APPLICATION_SET,
            ApplicationError::AlreadySet(bundle_ident.into())
        );
        ensure!(
            sys::setApplication(NSString::from_str(bundle_ident).deref()),
            ApplicationError::CouldNotSet(bundle_ident.into())
        );
        INIT_APPLICATION_SET.call_once(|| {
            APPLICATION_SET = true;
        });
        Ok(())
    }
}
