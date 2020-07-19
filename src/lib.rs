//! A very thin wrapper around NSNotifications
#![deny(
    missing_docs,
    trivial_casts,
    trivial_numeric_casts,
    unused_import_braces,
    unused_qualifications
)]
#![cfg(target_os = "macos")]
#![allow(improper_ctypes)]

extern crate chrono;
extern crate objc_foundation;
extern crate objc_id;
pub mod error;

use chrono::offset::*;
use error::{ApplicationError, NotificationError, NotificationResult};
use objc_foundation::{INSDictionary, INSString, NSDictionary, NSString};
use objc_id::Id;
use std::ops::Deref;
use std::path::PathBuf;

static mut APPLICATION_SET: bool = false;

mod sys {
    use objc_foundation::{NSDictionary, NSString};
    use objc_id::Id;
    #[link(name = "notify")]
    extern "C" {
        pub fn scheduleNotification(
            title: *const NSString,
            subtitle: *const NSString,
            message: *const NSString,
            sound: *const NSString,
            deliveryDate: f64,
        ) -> bool;
        pub fn sendNotification(
            title: *const NSString,
            subtitle: *const NSString,
            message: *const NSString,
            sound: *const NSString,
            options: *const NSDictionary<NSString, NSString>,
        ) -> Id<NSDictionary<NSString, NSString>>;
        pub fn setApplication(newbundleIdentifier: *const NSString) -> bool;
        pub fn getBundleIdentifier(appName: *const NSString) -> *const NSString;
    }
}

/// TODO: DOCUMENTATION
pub enum MainButton<'a> {
    /// TODO: DOCUMENTATION
    SingleAction(&'a str),
    /// TODO: DOCUMENTATION
    DropdownActions(&'a str, &'a [&'a str]),
}

/// TODO: DOCUMENTATION
pub struct NotificationOptions<'a> {
    /// TODO: DOCUMENTATION
    pub main_button: Option<MainButton<'a>>,
    /// TODO: DOCUMENTATION
    pub close_button: Option<&'a str>,
    /// TODO: DOCUMENTATION
    pub app_icon: Option<&'a str>,
    /// TODO: DOCUMENTATION
    pub content_image: Option<&'a str>,
    /// TODO: DOCUMENTATION
    pub group_id: Option<&'a str>,
}

impl<'a> NotificationOptions<'a> {
    /// TODO: Documentation
    pub fn default() -> NotificationOptions<'a> {
        NotificationOptions {
            main_button: None,
            close_button: None,
            app_icon: None,
            content_image: None,
            group_id: None,
        }
    }
    /// TODO: Documentation
    pub fn to_dictionary(&self) -> Id<NSDictionary<NSString, NSString>> {
        let keys = &[
            &*NSString::from_str("mainButtonLabel"),
            &*NSString::from_str("actions"),
            &*NSString::from_str("closeButtonLabel"),
            &*NSString::from_str("appIcon"),
            &*NSString::from_str("contentImage"),
            &*NSString::from_str("groupID"),
        ];
        let (main_button_label, actions): (&str, &[&str]) = match &self.main_button {
            Some(main_button) => match main_button {
                MainButton::SingleAction(main_button_label) => (main_button_label, &[]),
                MainButton::DropdownActions(main_button_label, actions) => {
                    (main_button_label, actions)
                }
            },
            None => ("", &[]),
        };

        let vals = vec![
            NSString::from_str(main_button_label),
            // TODO: Find a way to support NSArray as a NSDictionary Value rather than JUST NSString so I don't have to convert array to string and back
            NSString::from_str(&actions.join(",")),
            NSString::from_str(self.close_button.unwrap_or("")),
            NSString::from_str(self.app_icon.unwrap_or("")),
            NSString::from_str(self.content_image.unwrap_or("")),
            NSString::from_str(self.group_id.unwrap_or("")),
        ];
        NSDictionary::from_keys_and_objects(keys, vals)
    }
}

#[derive(Debug)]
enum NotificationResponse {
    None,
    ActionButton(String),
    CloseButton(String),
    Clicked,
    Replied(String),
}

impl NotificationResponse {
    fn from_dictionary(dictionary: Id<NSDictionary<NSString, NSString>>) -> Self {
        let dictionary = dictionary.deref();

        let activation_type =
            match dictionary.object_for(NSString::from_str("activationType").deref()) {
                Some(str) => Some(str.deref().as_str().to_owned()),
                None => None,
            };

        match activation_type.as_deref() {
            Some("actionClicked") => NotificationResponse::ActionButton(
                match dictionary.object_for(NSString::from_str("activationValue").deref()) {
                    Some(str) => str.deref().as_str().to_owned(),
                    None => String::from(""),
                },
            ),
            Some("closeClicked") => NotificationResponse::CloseButton(
                match dictionary.object_for(NSString::from_str("activationValue").deref()) {
                    Some(str) => str.deref().as_str().to_owned(),
                    None => String::from(""),
                },
            ),
            Some("replied") => NotificationResponse::Replied(
                match dictionary.object_for(NSString::from_str("activationValue").deref()) {
                    Some(str) => str.deref().as_str().to_owned(),
                    None => String::from(""),
                },
            ),
            Some("contentsClicked") => NotificationResponse::Clicked,
            Some(_) => NotificationResponse::None,
            None => NotificationResponse::None,
        }
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
/// use chrono::offset::*;
///
/// // schedule a notification in 5 seconds
/// let _ = schedule_notification("Title", &None, "This is the body", &Some("Ping"),
///                               Utc::now().timestamp() as f64 + 5.).unwrap();
/// ```
pub fn schedule_notification(
    title: &str,
    subtitle: &Option<&str>,
    message: &str,
    sound: &Option<&str>,
    delivery_date: f64,
) -> NotificationResult<()> {
    ensure!(
        delivery_date >= Utc::now().timestamp() as f64,
        NotificationError::ScheduleInThePast
    );

    let use_sound = match sound {
        Some(sound) if check_sound(sound) => sound,
        _ => "_mute",
    };
    unsafe {
        ensure!(
            sys::scheduleNotification(
                NSString::from_str(title).deref(),
                NSString::from_str(subtitle.unwrap_or("")).deref(),
                NSString::from_str(message).deref(),
                NSString::from_str(use_sound).deref(),
                delivery_date,
            ),
            NotificationError::UnableToSchedule
        );
        Ok(())
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
/// let _ = send_notification("Title", &None, "This is the body", &None).unwrap();
/// ```
pub fn send_notification(
    title: &str,
    subtitle: &Option<&str>,
    message: &str,
    sound: &Option<&str>,
    options: &Option<NotificationOptions>,
) -> NotificationResult<()> {
    let use_sound = match sound {
        Some(sound) if check_sound(sound) => sound,
        _ => "_mute",
    };

    let options = match options {
        Some(options) => options.to_dictionary(),
        None => NotificationOptions::default().to_dictionary(),
    };

    unsafe {
        if !APPLICATION_SET {
            let bundle = get_bundle_identifier_or_default("use_default");
            set_application(&bundle).unwrap();
            APPLICATION_SET = true;
        }
        let dictionary_response = sys::sendNotification(
            NSString::from_str(title).deref(),
            NSString::from_str(subtitle.unwrap_or("")).deref(),
            NSString::from_str(message).deref(),
            NSString::from_str(use_sound).deref(),
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

        println!("{:?}", response);
        Ok(())
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
        APPLICATION_SET = true;
        ensure!(
            sys::setApplication(NSString::from_str(bundle_ident).deref()),
            ApplicationError::CouldNotSet(bundle_ident.into())
        );
        Ok(())
    }
}

fn check_sound(sound_name: &str) -> bool {
    dirs::home_dir()
        .map(|path| path.join("/Library/Sounds/"))
        .into_iter()
        .chain(
            [
                "/Library/Sounds/",
                "/Network/Library/Sounds/",
                "/System/Library/Sounds/",
            ]
            .iter()
            .map(PathBuf::from),
        )
        .map(|sound_path| sound_path.join(format!("{}.aiff", sound_name)))
        .any(|some_path| some_path.exists())
}
