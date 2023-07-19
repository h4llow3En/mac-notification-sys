//! A very thin wrapper around UNNotification
//!
//! Only supported for `macOS 10.14+`
#![warn(
    missing_docs,
    trivial_casts,
    trivial_numeric_casts,
    unused_import_braces,
    unused_qualifications
)]
#![cfg(target_os = "macos")]
#![allow(improper_ctypes)]

use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use std::time::Duration;

use bitflags::bitflags;
use cron::Schedule;
use error::NotificationError;
use futures::StreamExt;
use icrate::block2::{Block, ConcreteBlock};
use icrate::Foundation::{NSDate, NSError, NSRunLoop, NSString, NSDefaultRunLoopMode};
use icrate::UserNotifications::{
    UNMutableNotificationContent, UNNotificationRequest, UNUserNotificationCenter,
};
use objc2::rc::Id;
use objc2::runtime::Bool;
use objc2::ClassType;
use url::Url;

mod delegate;
pub mod error;

/// Attachments to display in notification
pub struct NotificationAttachment {
    /// The unique identifier for the attachment.
    identifier: String,
    /// The URL of the file for this attachment.
    url: Url,
    options: Vec<NotificationAttachmentOptions>,
}

/// The trigger function
pub enum NotificationTriggerKind {
    /// A trigger condition that causes the system to deliver
    /// a notification after the amount of time you specify elapses.
    TimeInterval(Duration),
    /// A trigger condition that causes a notification
    /// the system delivers at a specific date and time.
    Calendar(Schedule),
}

/// The common behavior for subclasses that trigger the
/// delivery of a local or remote notification.
pub struct NotificationTrigger {
    /// The trigger function
    kind: NotificationTriggerKind,
    /// A Boolean value indicating whether the system reschedules
    /// the notification after it’s delivered.
    repeats: bool,
}

/// The data for a local or remote notification the system delivers to your app.
pub struct Notification {
    /// The unique identifier for the notification.
    identifier: String,
    /// The conditions that trigger the delivery of the notification.
    trigger: Option<NotificationTrigger>,
    /// The localized text that provides the notification’s primary description.
    title: String,
    /// The localized text that provides the notification’s secondary description.
    subtitle: String,
    /// The localized text that provides the notification’s main content.
    body: String,
    /// The visual and audio attachments to display alongside
    /// the notification’s main content.
    attachments: Vec<NotificationAttachment>,
    options: HashMap<String, String>,
}

impl Notification {
    pub(crate) fn into_raw(&self) -> Id<UNNotificationRequest> {
        let identifier = NSString::from_str(&self.identifier);
        let title = NSString::from_str(&self.title);
        let subtitle = NSString::from_str(&self.subtitle);
        let body = NSString::from_str(&self.body);
        unsafe {
            let content = UNMutableNotificationContent::init(UNMutableNotificationContent::alloc());
            content.setTitle(title.as_ref());
            content.setSubtitle(subtitle.as_ref());
            content.setBody(body.as_ref());
            let notification_request = UNNotificationRequest::requestWithIdentifier_content_trigger(
                &identifier,
                &content,
                None,
            );
            notification_request
        }
    }
}

/// A type of notification your app supports and the custom
/// actions that the system displays.
pub struct NotificationCategory {
    /// The unique string assigned to the category.
    identifier: String,
    /// The actions to display when the system delivers
    /// notifications of this type.
    actions: Vec<NotificationAction>,
    /// The intents related to notifications of this category.
    intent_identifiers: Vec<String>,
    /// Options for how to handle notifications of this type
    options: u8,
    /// The placeholder text to display when the system
    /// disables notification previews for the app.
    hidden_preview_body_placeholder: String,
    /// A format string for the summary description used when
    /// the system groups the category’s notifications.
    category_summary_format: String,
}

/// A task your app performs in response to a notification that the system delivers.
pub struct NotificationAction {
    /// The unique string that your app uses to identify the action
    identifier: String,
    /// The localized string to use as the title of the action.
    title: String,
    /// The icon associated with the action.
    icon: NotificationActionIcon,
    /// The behaviors associated with the action.
    options: u8,
}

/// An icon associated with an action.
pub enum NotificationActionIcon {
    /// Creates an action icon based on an image in your app’s bundle,
    /// preferably in an asset catalog.
    TemplateImageName(String),
    /// Creates an action icon by using a system symbol image.
    SystemImageName(String),
}

/// Which area need to show in the thumbnail
///
/// All the units should be 0-1. The origin of the coordinate
/// system is the left bottom
pub struct ThumbnailClippingRect {
    /// Left bottom coordinate
    origin: (f32, f32),
    /// Size of the rectangle
    size: (f32, f32),
}

/// Thumbnail time hint
pub enum ThumbnailTimeKey {
    /// Frame number of the animated image
    FrameNumber(u32),
    /// End of the movie. (Only supported for movies)
    End,
    /// Time of the movie
    Time {
        /// Whether the provided value is a rounded one
        rounded: bool,
        /// Number of hours elapsed
        hour: u32,
        /// Number of minutes elapsed
        minute: u8,
        /// Number of seconds elapsed
        second: u8,
    },
}

/// Options for attachments
pub enum NotificationAttachmentOptions {
    /// A hint about an attachment’s file type.
    TypeHintKey(String),
    /// A Boolean value indicating whether the system hides
    /// the attachment’s thumbnail.
    ThumbnailHiddenKey(bool),
    /// The clipping rectangle for a thumbnail image.
    ThumbnailClippingRectKey(ThumbnailClippingRect),
    /// The frame number of an animation to use as a thumbnail image.
    ThumbnailTimeKey(ThumbnailTimeKey),
}

bitflags! {
    /// The behaviors you can apply to an action.
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
    pub struct NotificationActionOptions: u8 {
        /// No option
        const None = 0b000;
        /// The action can be performed only on an unlocked device.
        const AuthenticationRequired = 0b001;
        /// The action performs a destructive task.
        const Destructive = 0b010;
        /// The action causes the app to launch in the foreground.
        const Foreground = 0b100;
    }

    /// Constants indicating how to handle notifications associated
    /// with this category.
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
    pub struct NotificationCategoryOptions: u8 {
        /// No options.
        const None = 0b00000;
        /// Send dismiss actions to the UNUserNotificationCenter object’s
        /// delegate for handling.
        const CustomDismissAction = 0b00001;
        /// Allow CarPlay to display notifications of this type.
        const AllowInCarPlay = 0b00010;
        /// Show the notification’s title, even if the user has disabled notification
        /// previews for the app.
        const HiddenPreviewShowTitle = 0b00100;
        /// Show the notification’s subtitle, even if the user has disabled
        /// notification previews for the app.
        const HiddenPreviewShowSubtitle = 0b01000;
        /// An option that grants Siri permission to read incoming messages out loud when
        /// the user has a compatible audio output device connected.
        #[deprecated]
        const AllowAnnouncement = 0b10000;
    }

    /// Options that determine the authorized features of
    /// local and remote notifications.
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
    pub struct AuthorizationOptions: u8 {
        /// No authorization options.
        const None = 0b0000000;
        /// The ability to update the app’s badge.
        const Badge = 0b0000001;
        /// The ability to play sounds.
        const Sound = 0b0000010;
        /// The ability to display alerts.
        const Alert = 0b0000100;
        /// The ability to display notifications in a CarPlay environment.
        const CarPlay = 0b0001000;
        /// The ability to play sounds for critical alerts.
        const CriticalAlert = 0b0010000;
        /// An option indicating the system should display a
        /// button for in-app notification settings.
        const ProvidesAppNotificationSettings = 0b0100000;
        /// The ability to post noninterrupting notifications provisionally
        /// to the Notification Center.
        const Provisional = 0b1000000;
    }
}

/// Run the RunLoop once
pub fn run_ns_run_loop_once() {
    unsafe {
        let main_loop = NSRunLoop::mainRunLoop();
        let limit_date = NSDate::dateWithTimeIntervalSinceNow(0.1);
        main_loop.acceptInputForMode_beforeDate(NSDefaultRunLoopMode, &limit_date);
    }
}

/// Requesting the authorization to send notifications
///
/// Returning an error if not allowed or any other error occured. Ok after accept one or many
/// options.
pub async fn request_authorization(
    options: AuthorizationOptions,
) -> Result<(), NotificationError> {
    let current_notification_center =
        unsafe { UNUserNotificationCenter::currentNotificationCenter() };

    // TODO:- Replace this with oneshot after block2 supported FnOnce
    // @see https://github.com/madsmtm/objc2/issues/168
    let (sender, mut receiver) = futures::channel::mpsc::channel::<Result<(), Id<NSError>>>(1);
    let arc_sender = Arc::new(Mutex::new(sender));

    let auth_handler = ConcreteBlock::new(move |_granted: Bool, err: *mut NSError| {
        let err = unsafe { err.as_ref() };
        let mut sender_locked = arc_sender.lock().unwrap();
        match err {
            Some(err) => {
                sender_locked.try_send(Err(err.retain())).unwrap();
            }
            None => {
                sender_locked.try_send(Ok(())).unwrap();
            }
        }
        sender_locked.close_channel();
    });
    let auth_handler = auth_handler.copy();
    let auth_handler: &Block<(Bool, *mut NSError), ()> = &auth_handler;

    unsafe {
        current_notification_center.requestAuthorizationWithOptions_completionHandler(
            options.bits() as usize,
            auth_handler,
        );
    }

    let received = receiver.next().await.unwrap();
    receiver.close();

    received.map_err(NotificationError::from)
}
