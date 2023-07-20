//! Notification Types
use bitflags::bitflags;
use cron::Schedule;
use std::{collections::HashMap, time::Duration};
use url::Url;

/// The data for a local or remote notification the system delivers to your app.
#[derive(Debug, Clone)]
pub struct Notification {
    /// The unique identifier for the notification.
    pub(crate) identifier: String,
    /// The conditions that trigger the delivery of the notification.
    pub(crate) trigger: Option<Trigger>,
    /// The localized text that provides the notification’s primary description.
    pub(crate) title: Option<String>,
    /// The localized text that provides the notification’s secondary description.
    pub(crate) subtitle: Option<String>,
    /// The localized text that provides the notification’s main content.
    pub(crate) body: String,
    /// The visual and audio attachments to display alongside
    /// the notification’s main content.
    pub(crate) attachments: Vec<Attachment>,
    /// The custom data to associate with the notification.
    pub(crate) user_info: HashMap<String, String>,
    /// The identifier that groups related notifications.
    pub(crate) thread_identifier: Option<String>,
    /// The identifier of the notification’s category.
    pub(crate) category_identifier: Option<String>,
    /// The text the system adds to the notification summary to
    /// provide additional context.
    pub(crate) summary_argument: Option<String>,
    /// The number the system adds to the notification summary when the
    /// notification represents multiple items.
    pub(crate) summary_argument_count: Option<usize>,
    /// The name of the image or storyboard to use when your app
    /// launches because of the notification.
    pub(crate) launch_image_name: Option<String>,
    /// The number that your app’s icon displays.
    pub(crate) badge: Option<usize>,
    /// The value your app uses to determine which scene
    /// to display to handle the notification.
    pub(crate) target_content_identifier: Option<String>,
    /// The sound that plays when the system delivers the notification.
    pub(crate) sound: Option<Sound>,
    /// The notification’s importance and required delivery timing.
    ///
    /// Platform Support:- macOS 12.0+
    pub(crate) interruption_level: Option<InterruptionLevel>,
    /// The score the system uses to determine if the notification
    /// is the summary’s featured notification.
    ///
    /// Platform Support:- macOS 12.0+
    pub(crate) relevance_score: Option<f32>,
    /// The criteria the system evaluates to determine if it displays
    /// the notification in the current Focus.
    ///
    /// Platform Support:- macOS 13.0+
    pub(crate) filter_criteria: Option<String>,
}

/// The sound played upon delivery of a notification.
#[derive(Debug, Clone, Default)]
pub enum Sound {
    /// default sound for notifications.
    #[default]
    Default,
    /// Creates a sound object that represents a custom sound file.
    /// <https://developer.apple.com/documentation/usernotifications/unnotificationsound/1649031-soundnamed?language=objc>
    Named(String),
    /// The default sound used for critical alerts.
    DefaultCriticalSound,
    /// Creates a sound object that plays the default critical alert
    /// sound at the volume you specify. The volume must be a value
    /// between 0.0 and 1.0.
    DefaultCriticalSoundWithVolume(f32),
    /// Creates a custom sound object for critical alerts.
    CriticalSoundNamed(String),
    /// Custom sound for critical alerts with the volume you specify.
    CriticalSoundNamedWithVolume(String, f32),
    /// Default ringtone of the iPad or iPhone. This method is not supported
    /// for macOS.
    DefaultRingtone,
    /// Custom ringtone sound. This method is not supported for macOS
    RingtoneSoundNamed(String),
}

/// Attachments to display in notification
#[derive(Debug, Clone)]
pub struct Attachment {
    /// The unique identifier for the attachment.
    pub(crate) identifier: String,
    /// The URL of the file for this attachment.
    pub(crate) url: Url,
    pub(crate) options: Vec<AttachmentOptions>,
}

/// The trigger function
#[derive(Debug, Clone)]
pub enum TriggerKind {
    /// A trigger condition that causes the system to deliver
    /// a notification after the amount of time you specify elapses.
    TimeInterval(Duration),
    /// A trigger condition that causes a notification
    /// the system delivers at a specific date and time.
    Calendar(Schedule),
}

/// The common behavior for subclasses that trigger the
/// delivery of a local or remote notification.
#[derive(Debug, Clone)]
pub struct Trigger {
    /// The trigger function
    pub(crate) kind: TriggerKind,
    /// A Boolean value indicating whether the system reschedules
    /// the notification after it’s delivered.
    pub(crate) repeats: bool,
}

/// A type of notification your app supports and the custom
/// actions that the system displays.
#[derive(Debug, Clone)]
pub struct Category {
    /// The unique string assigned to the category.
    pub(crate) identifier: String,
    /// The actions to display when the system delivers
    /// notifications of this type.
    pub(crate) actions: Vec<Action>,
    /// The intents related to notifications of this category.
    pub(crate) intent_identifiers: Vec<String>,
    /// Options for how to handle notifications of this type
    pub(crate) options: CategoryOptions,
    /// The placeholder text to display when the system
    /// disables notification previews for the app.
    pub(crate) hidden_preview_body_placeholder: Option<String>,
    /// A format string for the summary description used when
    /// the system groups the category’s notifications.
    pub(crate) category_summary_format: Option<String>,
}

/// A task your app performs in response to a notification that the system delivers.
#[derive(Debug, Clone)]
pub struct Action {
    /// The unique string that your app uses to identify the action
    pub(crate) identifier: String,
    /// The localized string to use as the title of the action.
    pub(crate) title: String,
    /// The icon associated with the action.
    pub(crate) icon: Option<ActionIcon>,
    /// The behaviors associated with the action.
    pub(crate) options: ActionOptions,
}

/// An icon associated with an action.
#[derive(Debug, Clone)]
pub enum ActionIcon {
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
#[derive(Debug, Clone)]
pub struct ThumbnailClippingRect {
    /// Left bottom coordinate
    pub(crate) origin: (f32, f32),
    /// Size of the rectangle
    pub(crate) size: (f32, f32),
}

/// Thumbnail time hint
#[derive(Debug, Clone)]
pub enum ThumbnailTimeKey {
    /// Frame number of the animated image
    FrameNumber(u32),
    /// End of the movie. (Only supported for movies)
    End,
    /// Start of the movie
    Start,
    /// Time of the movie
    Time(Duration),
}

/// Options for attachments
#[derive(Debug, Clone)]
pub enum AttachmentOptions {
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
    pub struct ActionOptions: u8 {
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
    pub struct CategoryOptions: u8 {
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

/// Constants that indicate the importance and delivery timing of a notification.
///
/// Platform Support:- macOS 12.0+
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum InterruptionLevel {
    /// The system presents the notification immediately, lights up
    /// the screen, and can play a sound.
    Active,
    /// The system presents the notification immediately, lights up the screen,
    /// and bypasses the mute switch to play a sound.
    Critical,
    /// The system adds the notification to the notification list
    /// without lighting up the screen or playing a sound.
    Passive,
    /// The system presents the notification immediately, lights up the screen,
    /// and can play a sound, but won’t break through system notification controls.
    TimeSensitive,
}
