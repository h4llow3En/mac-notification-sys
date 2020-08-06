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

/// Possible actions accessible through the main button of the notification
pub enum MainButton<'a> {
    /// Display a single action with the given name
    ///
    /// # Example:
    ///
    /// ```no_run
    /// # use mac_notification_sys::*;
    /// let _ = MainButton::SingleAction("Action name");
    /// ```
    SingleAction(&'a str),
    /// Display a dropdown with the given title, with a list of actions with given names
    ///
    /// # Example:
    ///
    /// ```no_run
    /// # use mac_notification_sys::*;
    /// let _ = MainButton::DropdownActions("Dropdown name", &["Action 1", "Action 2"]);
    /// ```
    DropdownActions(&'a str, &'a [&'a str]),
    /// Display a text input field with the given placeholder
    ///
    /// # Example:
    ///
    /// ```no_run
    /// # use mac_notification_sys::*;
    /// let _ = MainButton::Response("Enter some text...");
    /// ```
    Response(&'a str),
}

/// Options to further customize the notification
pub struct NotificationOptions<'a> {
    /// Allow actions through a main button
    ///
    /// # Example:
    ///
    /// ```no_run
    /// # use mac_notification_sys::*;
    /// let _ = NotificationOptions {
    ///     main_button: Some(MainButton::SingleAction("Main button")),
    ///
    ///     ..Default::default()
    /// };
    /// ```
    pub main_button: Option<MainButton<'a>>,
    /// Display a close button with the given name
    ///
    /// # Example:
    ///
    /// ```no_run
    /// # use mac_notification_sys::*;
    /// let _ = NotificationOptions {
    ///     close_button: Some("Close"),
    ///
    ///     ..Default::default()
    /// };
    /// ```
    pub close_button: Option<&'a str>,
    /// Display an icon on the left side of the notification
    ///
    /// NOTE: The icon of the app associated to the bundle will be displayed next to the notification title
    ///
    /// # Example:
    ///
    /// ```no_run
    /// # use mac_notification_sys::*;
    /// let _ = NotificationOptions {
    ///     app_icon: Some("/path/to/icon.icns"),
    ///
    ///     ..Default::default()
    /// };
    /// ```
    pub app_icon: Option<&'a str>,
    /// Display an image on the right side of the notification
    ///
    /// # Example:
    ///
    /// ```no_run
    /// # use mac_notification_sys::*;
    /// let _ = NotificationOptions {
    ///     content_image: Some("/path/to/image.png"),
    ///
    ///     ..Default::default()
    /// };
    /// ```
    pub content_image: Option<&'a str>,
    /// Set an identifier to group multiple notifications together
    /// Notifications in group will dismiss each other so only the latest one is displayed
    ///
    /// # Example:
    ///
    /// ```no_run
    /// # use mac_notification_sys::*;
    /// let _ = NotificationOptions {
    ///     group_id: Some("my_notification_group"),
    ///
    ///     ..Default::default()
    /// };
    /// ```
    pub group_id: Option<&'a str>,
    /// Schedule the notification to be delivered at a later time
    ///
    /// The first parameter is the time at which to schedule the notification
    ///
    /// The second parameter defines whether or not the notification
    ///     should be fired synchronously (if no action is set)
    ///
    /// # Example:
    ///
    /// ```no_run
    /// # use mac_notification_sys::*;
    /// # use chrono::offset::*;
    /// let stamp = Utc::now().timestamp() as f64 + 5.;
    ///
    /// let _ = NotificationOptions {
    ///     // Synchronous is true, this will wait until the user
    ///     // interacts with the notification before returning
    ///     delivery_date: Some((stamp, true)),
    ///
    ///     ..Default::default()
    /// };
    /// ```
    pub delivery_date: Option<(f64, bool)>,
    /// Play a system sound when the notification is delivered
    ///
    /// # Example:
    ///
    /// ```no_run
    /// # use mac_notification_sys::*;
    /// let _ = NotificationOptions {
    ///     sound: Some("Blow"),
    ///
    ///     ..Default::default()
    /// };
    /// ```
    pub sound: Option<&'a str>,
}

impl<'a> Default for NotificationOptions<'a> {
    fn default() -> NotificationOptions<'a> {
        NotificationOptions {
            main_button: None,
            close_button: None,
            app_icon: None,
            content_image: None,
            group_id: None,
            delivery_date: None,
            sound: None,
        }
    }
}

impl<'a> NotificationOptions<'a> {
    /// Convert the NotificationOptions into an Objective C NSDictionary
    fn to_dictionary(&self) -> Id<NSDictionary<NSString, NSString>> {
        // TODO: If possible, find a way to simplify this so I don't have to manually convert struct to NSDictionary
        let keys = &[
            &*NSString::from_str("mainButtonLabel"),
            &*NSString::from_str("actions"),
            &*NSString::from_str("closeButtonLabel"),
            &*NSString::from_str("appIcon"),
            &*NSString::from_str("contentImage"),
            &*NSString::from_str("groupID"),
            &*NSString::from_str("response"),
            &*NSString::from_str("deliveryDate"),
            &*NSString::from_str("synchronous"),
            &*NSString::from_str("sound"),
        ];
        let (main_button_label, actions, is_response): (&str, &[&str], bool) =
            match &self.main_button {
                Some(main_button) => match main_button {
                    MainButton::SingleAction(main_button_label) => (main_button_label, &[], false),
                    MainButton::DropdownActions(main_button_label, actions) => {
                        (main_button_label, actions, false)
                    }
                    MainButton::Response(response) => (response, &[], true),
                },
                None => ("", &[], false),
            };

        let vals = vec![
            NSString::from_str(main_button_label),
            // TODO: Find a way to support NSArray as a NSDictionary Value rather than JUST NSString so I don't have to convert array to string and back
            NSString::from_str(&actions.join(",")),
            NSString::from_str(self.close_button.unwrap_or("")),
            NSString::from_str(self.app_icon.unwrap_or("")),
            NSString::from_str(self.content_image.unwrap_or("")),
            NSString::from_str(self.group_id.unwrap_or_default()),
            // TODO: Same as above, if NSDictionary could support multiple types, this could be a boolean
            NSString::from_str(if is_response { "yes" } else { "" }),
            NSString::from_str(&match self.delivery_date {
                Some((delivery_date, _)) => delivery_date.to_string(),
                _ => String::new(),
            }),
            // TODO: Same as above, if NSDictionary could support multiple types, this could be a boolean
            NSString::from_str(match self.delivery_date {
                Some((_, true)) => "yes",
                _ => "",
            }),
            NSString::from_str(match self.sound {
                Some(sound) if check_sound(sound) => sound,
                _ => "_mute",
            }),
        ];
        NSDictionary::from_keys_and_objects(keys, vals)
    }
}

/// Response from the Notification
#[derive(Debug)]
pub enum NotificationResponse {
    /// No interaction has occured
    None,
    /// User clicked on an action button with the given name
    ActionButton(String),
    /// User clicked on the close button with the given name
    CloseButton(String),
    /// User clicked the notification directly
    Clicked,
    /// User submitted text to the input text field
    Replied(String),
}

impl NotificationResponse {
    /// Create a NotificationResponse from the given Objective C NSDictionary
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
            _ => NotificationResponse::None,
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
/// // deliver a silent notification
/// let _ = send_notification("Title", None, "This is the body", None).unwrap();
/// ```
pub fn send_notification(
    title: &str,
    subtitle: Option<&str>,
    message: &str,
    options: Option<NotificationOptions>,
) -> NotificationResult<NotificationResponse> {
    if let Some(options) = &options {
        if let Some((delivery_date, _)) = options.delivery_date {
            ensure!(
                delivery_date >= Utc::now().timestamp() as f64,
                NotificationError::ScheduleInThePast
            );
        }
    };

    let options = options
        .unwrap_or(NotificationOptions::default())
        .to_dictionary();

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
        // TODO: Is there a point to restricting set_application to only once?
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
