//! Builder for notifications

use chrono::{DateTime, Local, TimeZone};
use core::str::FromStr;
use core::time::Duration;
use cron::Schedule;
use std::collections::HashMap;
use url::Url;

use super::notification::{
    Action, ActionIcon, ActionOptions, AnimatedImageAttachmentOptions, Attachment,
    AttachmentOptions, AudioAttachmentOptions, Category, CategoryOptions, ImageAttachmentOptions,
    InterruptionLevel, Notification, Sound, ThumbnailClippingRect,
    ThumbnailedAttachmentOptions, Trigger, TriggerKind, UnifiedAttachmentOptions,
    VideoAttachmentOptions, VideoTime,
};

/// The data for a local or remote notification the system delivers to your app.
#[derive(Default)]
pub struct NotificationBuilder {
    /// The unique identifier for the notification.
    identifier: Option<String>,
    /// The conditions that trigger the delivery of the notification.
    trigger: Option<Trigger>,
    /// The localized text that provides the notification’s primary description.
    title: Option<String>,
    /// The localized text that provides the notification’s secondary description.
    subtitle: Option<String>,
    /// The localized text that provides the notification’s main content.
    body: String,
    /// The visual and audio attachments to display alongside
    /// the notification’s main content.
    attachments: Vec<Attachment>,
    /// The custom data to associate with the notification.
    user_data: HashMap<String, String>,
    /// The identifier that groups related notifications.
    thread_identifier: Option<String>,
    /// The identifier of the notification’s category.
    category_identifier: Option<String>,
    /// The text the system adds to the notification summary to
    /// provide additional context.
    summary_argument: Option<String>,
    /// The number the system adds to the notification summary when the
    /// notification represents multiple items.
    summary_argument_count: Option<usize>,
    /// The name of the image or storyboard to use when your app
    /// launches because of the notification.
    launch_image_name: Option<String>,
    /// The number that your app’s icon displays.
    badge: Option<usize>,
    /// The value your app uses to determine which scene
    /// to display to handle the notification.
    target_content_identifier: Option<String>,
    /// The sound that plays when the system delivers the notification.
    sound: Option<Sound>,
    /// The notification’s importance and required delivery timing.
    ///
    /// Platform Support:- macOS 12.0+
    interruption_level: Option<InterruptionLevel>,
    /// The score the system uses to determine if the notification
    /// is the summary’s featured notification.
    ///
    /// Platform Support:- macOS 12.0+
    relevance_score: Option<f32>,
    /// The criteria the system evaluates to determine if it displays
    /// the notification in the current Focus.
    ///
    /// Platform Support:- macOS 13.0+
    filter_criteria: Option<String>,
}

impl NotificationBuilder {
    /// Creating a new notification builder
    ///
    /// `body` :- The localized text that provides the notification’s main content.
    pub fn new_with_body<S:Into<String>>(body: S) -> NotificationBuilder {
        Self {
            body: body.into(),
            ..Default::default()
        }
    }

    /// The localized text that provides the notification’s primary description.
    pub fn title<'a, S: Into<String>>(&'a mut self, title: S) -> &'a mut Self {
        self.title = Some(title.into());
        self
    }

    /// The localized text that provides the notification’s secondary description.
    pub fn subtitle<'a, S: Into<String>>(&'a mut self, subtitle: S) -> &'a mut Self {
        self.subtitle = Some(subtitle.into());
        self
    }

    /// The localized text that provides the notification’s main content.
    pub fn body<'a, S: Into<String>>(&'a mut self, body: S) -> &'a mut Self {
        self.body = body.into();
        self
    }

    /// The custom data to associate with the notification.
    pub fn user_data<'a, K: Into<String>, V: Into<String>>(
        &'a mut self,
        key: K,
        value: V,
    ) -> &'a mut Self {
        self.user_data.insert(key.into(), value.into());
        self
    }

    /// Unique identifier of the notification
    pub fn identifier<'a, I: Into<String>>(&'a mut self, identifier: I) -> &'a mut Self {
        self.identifier = Some(identifier.into());
        self
    }

    /// Attach a vide/image/audio to the notification.
    /// Use the `AttachmentBuilder` to make an attachment.
    pub fn attachment<'a, T>(&'a mut self, attachment: Attachment) -> &'a mut Self {
        self.attachments.push(attachment);
        self
    }

    /// Specifying the category of the notification
    pub fn category<'a>(&'a mut self, category: &Category) -> &'a mut Self {
        self.category_identifier = Some(category.identifier.clone());
        self
    }

    /// Specifying the category by using the category id
    pub fn category_id<'a, C: Into<String>>(&'a mut self, category_id: C) -> &'a mut Self {
        self.category_identifier = Some(category_id.into());
        self
    }

    /// The name of the image or storyboard to use when your app
    /// launches because of the notification.
    pub fn launch_image<'a, I: Into<String>>(&'a mut self, image_name: I) -> &'a mut Self {
        self.launch_image_name = Some(image_name.into());
        self
    }

    /// The identifier that groups related notifications.
    pub fn thread<'a, I: Into<String>>(&'a mut self, thread_id: I) -> &'a mut Self {
        self.thread_identifier = Some(thread_id.into());
        self
    }

    /// The default sound for notifications played upon delivery of a notification.
    pub fn default_sound<'a>(&'a mut self) -> &'a mut Self {
        self.sound = Some(Sound::Default);
        self
    }

    /// A sound that represents a custom sound file will be played
    /// upon delivery of a notification
    pub fn sound<'a, N: Into<String>>(&'a mut self, name: N) -> &'a mut Self {
        self.sound = Some(Sound::Named(name.into()));
        self
    }

    /// The default critical sound for notifications will be played
    /// upon delivery of a notification
    pub fn default_critical_sound<'a>(&'a mut self) -> &'a mut Self {
        self.sound = Some(Sound::DefaultCriticalSound);
        self
    }

    /// The default critical sound for notifications with specified volume
    /// will be played upon delivery of a notification. The volume range is 0.0-1.0
    pub fn default_critical_sound_with_volume<'a>(&'a mut self, volume: f32) -> &'a mut Self {
        self.sound = Some(Sound::DefaultCriticalSoundWithVolume(volume));
        self
    }

    /// A specified critical sound will be played upon delivery of a notification
    pub fn critical_sound<'a, N: Into<String>>(&'a mut self, sound_name: N) -> &'a mut Self {
        self.sound = Some(Sound::CriticalSoundNamed(sound_name.into()));
        self
    }

    /// A specified cirtical sound with the given volume will be played upon delivery
    /// of a notification. The volumd range is 0.0 - 1.0
    pub fn critical_sound_with_volume<'a, N: Into<String>>(
        &'a mut self,
        sound_name: N,
        volume: f32,
    ) -> &'a mut Self {
        self.sound = Some(Sound::CriticalSoundNamedWithVolume(
            sound_name.into(),
            volume,
        ));
        self
    }

    /// The default ringtone sound will be played upon delivery of a notification.
    /// This option is only available for iOS, iPadOS and visionOS
    pub fn default_ringtone<'a>(&'a mut self) -> &'a mut Self {
        self.sound = Some(Sound::DefaultRingtone);
        self
    }

    /// A specified ringtone sound will be played upon delivery of a notification.
    /// This option is only available for iOS, iPadOS and visionOS
    pub fn ringtone<'a, N: Into<String>>(&'a mut self, ringtone_name: N) -> &'a mut Self {
        self.sound = Some(Sound::RingtoneSoundNamed(ringtone_name.into()));
        self
    }

    /// Schedule the notification to repeat after some delay. The first
    /// notification will be delivered after elapsed the provided duration.
    ///
    /// The minimum duration is 1 minute
    pub fn interval<'a>(&'a mut self, duration: Duration) -> &'a mut Self {
        assert!(
            duration.as_secs() > 60,
            "Duration should be grater than one minute"
        );

        self.trigger = Some(Trigger {
            kind: TriggerKind::TimeInterval(duration),
            repeats: true,
        });
        self
    }

    /// Schedule the notification to send after some delay
    pub fn delay<'a>(&'a mut self, duration: Duration) -> &'a mut Self {
        self.trigger = Some(Trigger {
            kind: TriggerKind::TimeInterval(duration),
            repeats: false,
        });
        self
    }

    /// Schedule the notification for one time using a cron pattern
    /// The pattern should be in local timezone
    ///
    /// Only supporting the `*` mark pattern.
    /// Ranges, Comma(`,`) separated lists and forward slashes(`/`) are not supported.
    pub fn cron_one_time<'a>(&'a mut self, schedule: Schedule) -> &'a mut Self {
        self.trigger = Some(Trigger {
            kind: TriggerKind::Calendar(schedule),
            repeats: false,
        });
        self
    }

    /// Schedule the notification to repeat for a cron pattern
    /// The pattern should be in local timezone
    ///
    /// Only supporting the `*` mark pattern.
    /// Ranges, Comma(`,`) separated lists and forward slashes(`/`) are not supported.
    pub fn cron<'a>(&'a mut self, schedule: Schedule) -> &'a mut Self {
        self.trigger = Some(Trigger {
            kind: TriggerKind::Calendar(schedule),
            repeats: true,
        });
        self
    }

    /// Schedule the notification to deliver at exact datetime
    pub fn schedule<'a, Tz: TimeZone>(&'a mut self, date_time: DateTime<Tz>) -> &'a mut Self {
        self.trigger = Some(Trigger {
            kind: TriggerKind::Calendar(datetime_to_schedule(date_time)),
            repeats: false,
        });
        self
    }

    /// The system presents the notification immediately, lights up
    /// the screen, and can play a sound.
    ///
    /// Supported Platforms:- macOS 12.0
    pub fn as_active<'a>(&'a mut self) -> &'a mut Self {
        self.interruption_level = Some(InterruptionLevel::Active);
        self
    }

    /// The system presents the notification immediately, lights up
    /// the screen, and bypasses the mute switch to play a sound.
    ///
    /// Supported Platforms:- macOS 12.0
    pub fn as_critical<'a>(&'a mut self) -> &'a mut Self {
        self.interruption_level = Some(InterruptionLevel::Critical);
        self
    }

    /// The system adds the notification to the notification list
    /// without lighting up the screen or playing a sound.
    ///
    /// Supported Platforms:- macOS 12.0
    pub fn as_passive<'a>(&'a mut self) -> &'a mut Self {
        self.interruption_level = Some(InterruptionLevel::Passive);
        self
    }

    /// The system presents the notification immediately, lights
    /// up the screen, and can play a sound, but won’t break
    /// through system notification controls.
    ///
    /// Supported Platforms:- macOS 12.0
    pub fn as_time_sensitive<'a>(&'a mut self) -> &'a mut Self {
        self.interruption_level = Some(InterruptionLevel::TimeSensitive);
        self
    }

    /// The text the system adds to the notification summary to
    /// provide additional context.
    pub fn summary_argument<'a, S: Into<String>>(
        &'a mut self,
        summary_argument: S,
    ) -> &'a mut Self {
        self.summary_argument = Some(summary_argument.into());
        self
    }

    /// The number the system adds to the notification summary when the
    /// notification represents multiple items.
    pub fn summary_argument_count<'a>(&'a mut self, summary_argument_count: usize) -> &'a mut Self {
        self.summary_argument_count = Some(summary_argument_count);
        self
    }

    /// The number that your app’s icon displays.
    pub fn badge_count<'a>(&'a mut self, badge: usize) -> &'a mut Self {
        self.badge = Some(badge);
        self
    }

    /// The value your app uses to determine which scene
    /// to display to handle the notification.
    pub fn target_content_identifier<'a, S: Into<String>>(
        &'a mut self,
        identifier: S,
    ) -> &'a mut Self {
        self.target_content_identifier = Some(identifier.into());
        self
    }

    /// The score the system uses to determine if the notification
    /// is the summary’s featured notification.
    ///
    /// Platform Support:- macOS 12.0+
    pub fn relevance_score<'a>(&'a mut self, score: f32) -> &'a mut Self {
        self.relevance_score = Some(score);
        self
    }

    /// The criteria the system evaluates to determine if it displays
    /// the notification in the current Focus.
    ///
    /// Platform Support:- macOS 13.0+
    pub fn filter_criteria<'a, S: Into<String>>(&'a mut self, filter: S) -> &'a mut Self {
        self.filter_criteria = Some(filter.into());
        self
    }

    /// Building the notification
    pub fn build(&self) -> Notification {
        let identifier = if let Some(identifier) = self.identifier.clone().take() {
            identifier
        } else {
            let uuid_v4 = uuid::Uuid::new_v4();
            uuid_v4.to_string()
        };

        Notification {
            identifier,
            trigger: self.trigger.clone(),
            title: self.title.clone(),
            subtitle: self.subtitle.clone(),
            body: self.body.clone(),
            attachments: self.attachments.clone(),
            user_info: self.user_data.clone(),
            thread_identifier: self.thread_identifier.clone(),
            category_identifier: self.category_identifier.clone(),
            summary_argument: self.summary_argument.clone(),
            summary_argument_count: self.summary_argument_count.clone(),
            launch_image_name: self.launch_image_name.clone(),
            badge: self.badge,
            target_content_identifier: self.target_content_identifier.clone(),
            sound: self.sound.clone(),
            interruption_level: self.interruption_level,
            relevance_score: self.relevance_score,
            filter_criteria: self.filter_criteria.clone(),
        }
    }
}

/// The builder for notification attachments
pub struct AttachmentBuilder<T> {
    /// The unique identifier for the attachment.
    identifier: Option<String>,
    /// The URL of the file for this attachment.
    url: Url,
    options: T,
}

impl<T> AttachmentBuilder<T> {
    /// Create a new video attachment
    pub fn video(url: Url) -> AttachmentBuilder<VideoAttachmentOptions> {
        AttachmentBuilder {
            url,
            options: VideoAttachmentOptions::default(),
            identifier: None,
        }
    }

    /// Create a new audio attachment
    pub fn audio(url: Url) -> AttachmentBuilder<AudioAttachmentOptions> {
        AttachmentBuilder {
            url,
            options: AudioAttachmentOptions::default(),
            identifier: None
        }
    }

    /// Create a new image attachment
    pub fn image(url: Url) -> AttachmentBuilder<ImageAttachmentOptions> {
        AttachmentBuilder {
            url,
            options: ImageAttachmentOptions::default(),
            identifier: None
        }
    }

    /// Create a new animated image attachment
    pub fn animated_image(url: Url) -> AttachmentBuilder<AnimatedImageAttachmentOptions> {
        AttachmentBuilder {
            url,
            options: AnimatedImageAttachmentOptions::default(),
            identifier: None
        }
    }

    /// Unique identifier of the attachment
    pub fn identifier<'a, I: Into<String>>(&'a mut self, identififer: I) -> &'a mut Self {
        self.identifier = Some(identififer.into());
        self
    }

    /// Generate an identifier if not provided
    /// otherwise returning the existing id
    fn generate_identifier(&self) -> String {
        if let Some(identifier) = self.identifier.clone() {
            identifier
        } else {
            let uuid_v4 = uuid::Uuid::new_v4();
            uuid_v4.to_string()
        }
    }
}

impl<T: UnifiedAttachmentOptions> AttachmentBuilder<T> {
    /// Specifying the encoded format when URL does not contain a file extension
    pub fn format<'a>(&'a mut self, format: T::Format) -> &'a mut Self {
        self.options.set_format(format);
        self
    }
}

impl<T: ThumbnailedAttachmentOptions> AttachmentBuilder<T> {
    /// Showing a cropped part as the thumbnail
    ///
    /// The coordinate system is starting from the lower left corner(0.0, 0.0). And
    /// the (1.0, 1.0) is the top right corner. The `origin` is also
    /// referring the lower left corner in cropped part. See more:-
    /// <https://developer.apple.com/documentation/corefoundation/cgrect?language=objc>
    pub fn crop_thumbnail<'a>(&'a mut self, origin: (f32, f32), size: (f32, f32)) -> &'a mut Self {
        self.options
            .crop_thumbnail(ThumbnailClippingRect { origin, size });
        self
    }

    /// Show/ hide the thumbnail of the attachment
    pub fn hide_thumbnail<'a>(&'a mut self, hide: bool) -> &'a mut Self {
        self.options.hide_thumbnail(hide);
        self
    }
}

impl AttachmentBuilder<AnimatedImageAttachmentOptions> {
    /// Showing a specific frame using the frame number
    pub fn thumbnail_frame<'a>(&'a mut self, frame_number: u64) -> &'a mut Self {
        self.options.thumbnail_frame = Some(frame_number);
        self
    }

    /// Build to an attachment
    pub fn build(self) -> Attachment {
        Attachment {
            identifier: self.generate_identifier(),
            url: self.url,
            options: Some(AttachmentOptions::AnimatedImage(self.options)),
        }
    }
}

impl AttachmentBuilder<VideoAttachmentOptions> {

    /// Show the thumbnail from the start scene of the video
    pub fn thumbnail_from_start<'a>(&'a mut self) -> &'a mut Self {
        self.options.thumbnail_time = Some(VideoTime::Start);
        self
    }

    /// Show the thumbnail from the end scene of the video
    pub fn thumbnail_from_end<'a>(&'a mut self) -> &'a mut Self {
        self.options.thumbnail_time = Some(VideoTime::End);
        self
    }

    /// Show the thumbnail at a specific time
    pub fn thumbnail_time<'a>(&'a mut self, dur: Duration) -> &'a mut Self {
        self.options.thumbnail_time = Some(VideoTime::Time(dur));
        self
    }

    /// Build to an attachment
    pub fn build(self) -> Attachment {
        Attachment {
            identifier: self.generate_identifier(),
            url: self.url,
            options: Some(AttachmentOptions::Video(self.options)),
        }
    }
}

impl AttachmentBuilder<ImageAttachmentOptions> {
    /// Build to an attachment
    pub fn build(self) -> Attachment {
        Attachment {
            identifier: self.generate_identifier(),
            url: self.url,
            options: Some(AttachmentOptions::Image(self.options)),
        }
    }
}

impl AttachmentBuilder<AudioAttachmentOptions> {
    /// Build to an attachment
    pub fn build(self) -> Attachment {
        Attachment {
            identifier: self.generate_identifier(),
            url: self.url,
            options: Some(AttachmentOptions::Audio(self.options)),
        }
    }
}

/// Builder for notification categories
pub struct CategoryBuilder {
    /// The unique string assigned to the category.
    identifier: Option<String>,
    /// The actions to display when the system delivers
    /// notifications of this type.
    actions: Vec<Action>,
    /// The intents related to notifications of this category.
    intent_identifiers: Vec<String>,
    /// Options for how to handle notifications of this type
    options: CategoryOptions,
    /// The placeholder text to display when the system
    /// disables notification previews for the app.
    hidden_preview_body_placeholder: Option<String>,
    /// A format string for the summary description used when
    /// the system groups the category’s notifications.
    category_summary_format: Option<String>,
}

impl CategoryBuilder {
    /// Create a new category builder
    pub fn new() -> CategoryBuilder {
        CategoryBuilder {
            options: CategoryOptions::None,
            identifier: None,
            actions: Vec::new(),
            intent_identifiers: Vec::new(),
            hidden_preview_body_placeholder: None,
            category_summary_format: None,
        }
    }

    /// The unique string assigned to the category.
    pub fn identifier<'a, S: Into<String>>(&'a mut self, identifier: S) -> &'a mut Self {
        self.identifier = Some(identifier.into());
        self
    }

    /// Adding an identifier of an intent related to
    /// notifications of this category.
    pub fn intent_identifier<'a, S: Into<String>>(&'a mut self, identifier: S) -> &'a mut Self {
        self.intent_identifiers.push(identifier.into());
        self
    }

    /// The placeholder text to display when the system
    /// disables notification previews for the app.
    pub fn hidden_preview_body_placeholder<'a, S: Into<String>>(
        &'a mut self,
        placeholder: S,
    ) -> &'a mut Self {
        self.hidden_preview_body_placeholder = Some(placeholder.into());
        self
    }

    /// A format string for the summary description used when
    /// the system groups the category’s notifications.
    pub fn category_summary_format<'a, S: Into<String>>(&'a mut self, format: S) -> &'a mut Self {
        self.category_summary_format = Some(format.into());
        self
    }

    /// Send dismiss actions to the UNUserNotificationCenter object’s
    /// delegate for handling.
    pub fn has_custom_dismiss_action<'a>(&'a mut self) -> &'a mut Self {
        self.options |= CategoryOptions::CustomDismissAction;
        self
    }

    /// Allow CarPlay to display notifications of this type.
    pub fn display_in_car_play<'a>(&'a mut self) -> &'a mut Self {
        self.options |= CategoryOptions::AllowInCarPlay;
        self
    }

    /// Show the notification’s title, even if the user has disabled notification
    /// previews for the app.
    pub fn show_title_when_hidden<'a>(&'a mut self) -> &'a mut Self {
        self.options |= CategoryOptions::HiddenPreviewShowTitle;
        self
    }

    /// Show the notification’s subtitle, even if the user has disabled
    /// notification previews for the app.
    pub fn show_subtitle_when_hidden<'a>(&'a mut self) -> &'a mut Self {
        self.options |= CategoryOptions::HiddenPreviewShowSubtitle;
        self
    }

    /// An option that grants Siri permission to read incoming messages out loud when
    /// the user has a compatible audio output device connected.
    #[deprecated]
    #[allow(deprecated)]
    pub fn allow_announcements<'a>(&'a mut self) -> &'a mut Self {
        self.options |= CategoryOptions::AllowAnnouncement;
        self
    }

    /// A task your app performs in response to a notification that the system delivers.
    ///
    /// You can build actions using the `ActionBuilder`
    pub fn action<'a>(&'a mut self, action: Action) -> &'a mut Self {
        self.actions.push(action);
        self
    }

    /// Building the category
    pub fn build(&self) -> Category {
        let identifier = if let Some(identifier) = self.identifier.clone().take() {
            identifier
        } else {
            let uuid_v4 = uuid::Uuid::new_v4();
            uuid_v4.to_string()
        };
        Category {
            identifier,
            actions: self.actions.clone(),
            intent_identifiers: self.intent_identifiers.clone(),
            options: self.options,
            hidden_preview_body_placeholder: self.hidden_preview_body_placeholder.clone(),
            category_summary_format: self.category_summary_format.clone(),
        }
    }
}

/// Builder for action buttons
pub struct ActionBuilder {
    /// The unique string that your app uses to identify the action
    identifier: Option<String>,
    /// The localized string to use as the title of the action.
    title: String,
    /// The icon associated with the action.
    icon: Option<ActionIcon>,
    /// The behaviors associated with the action.
    options: ActionOptions,
}

impl ActionBuilder {
    /// Create an action builder using the required title
    pub fn new_with_title<S: Into<String>>(title: S) -> Self {
        Self {
            identifier: None,
            title: title.into(),
            icon: None,
            options: ActionOptions::None,
        }
    }

    /// The unique string that your app uses to identify the action
    pub fn identifier<'a, S: Into<String>>(&'a mut self, identifier: S) -> &'a mut Self {
        self.identifier = Some(identifier.into());
        self
    }

    /// Creates an action icon based on an image in your app’s bundle,
    /// preferably in an asset catalog.
    pub fn bundle_image<'a, S: Into<String>>(&'a mut self, image_name: S) -> &'a mut Self {
        self.icon = Some(ActionIcon::TemplateImageName(image_name.into()));
        self
    }

    /// Creates an action icon by using a system symbol image.
    pub fn system_image<'a, S: Into<String>>(&'a mut self, image_name: S) -> &'a mut Self {
        self.icon = Some(ActionIcon::SystemImageName(image_name.into()));
        self
    }

    /// The action can be performed only on an unlocked device.
    pub fn require_authentication<'a>(&'a mut self) -> &'a mut Self {
        self.options |= ActionOptions::AuthenticationRequired;
        self
    }

    /// The action performs a destructive task.
    pub fn destructive<'a>(&'a mut self) -> &'a mut Self {
        self.options |= ActionOptions::Destructive;
        self
    }

    /// The action causes the app to launch in the foreground.
    pub fn foreground<'a>(&'a mut self) -> &'a mut Self {
        self.options |= ActionOptions::Foreground;
        self
    }

    /// Converting this builder to an action
    pub fn build(self) -> Action {
        let identifier = if let Some(identifier) = self.identifier.clone().take() {
            identifier
        } else {
            let uuid_v4 = uuid::Uuid::new_v4();
            uuid_v4.to_string()
        };

        Action {
            identifier,
            title: self.title,
            icon: self.icon,
            options: self.options,
        }
    }
}

fn datetime_to_schedule<Tz: TimeZone>(datetime: DateTime<Tz>) -> Schedule {
    // Converting the date to a cron schedule
    let local_tz = Local;
    let local_date = datetime.with_timezone(&local_tz);
    let pattern = local_date.format("%S %M %H %-d %b * %Y").to_string();
    let cron = Schedule::from_str(&pattern).unwrap();
    cron
}

#[cfg(test)]
mod tests {
    use super::datetime_to_schedule;
    use chrono::{Duration, FixedOffset, Local, Timelike, Utc};

    #[test]
    fn test_datetime_to_schedule() {
        let local_tz = Local;
        let utc_now = Utc::now();
        let local_now = utc_now.with_timezone(&local_tz);

        let before_time = local_now - Duration::hours(1);

        let schedule = datetime_to_schedule(local_now);
        let mut iter = schedule.after(&before_time);
        let first_date = iter.next();
        assert!(first_date.is_some());
        assert_eq!(local_now.with_nanosecond(0).unwrap(), first_date.unwrap());
        let finished = iter.next();
        assert!(finished.is_none());
    }

    #[test]
    fn test_datetime_to_schedule_with_different_timezone() {
        std::env::set_var("TZ", "IST");
        let aus_tz = FixedOffset::east_opt(36000).unwrap();
        let local_tz = Local;
        let utc_now = Utc::now();
        let aus_now = utc_now.with_timezone(&aus_tz);
        let local_now = utc_now.with_timezone(&local_tz);

        let before_time = local_now - Duration::hours(1);

        let schedule = datetime_to_schedule(aus_now);
        let mut iter = schedule.after(&before_time);
        let first_date = iter.next();
        assert!(first_date.is_some());

        assert_eq!(local_now.with_nanosecond(0).unwrap(), first_date.unwrap());

        let finished = iter.next();
        assert!(finished.is_none());
    }
}
