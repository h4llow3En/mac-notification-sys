//! Builder for notifications

use chrono::{DateTime, Local, TimeZone};
use cron::Schedule;
use std::{collections::HashMap, str::FromStr, time::Duration};
use url::Url;

use crate::notification::{
    Action, ActionIcon, ActionOptions, Attachment, AttachmentOptions, Category, CategoryOptions,
    InterruptionLevel, Notification, Sound, ThumbnailClippingRect, ThumbnailTimeKey, Trigger,
    TriggerKind,
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
    attachments: Vec<AttachmentBuilder>,
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
    pub fn new_with_body(body: String) -> NotificationBuilder {
        Self {
            body,
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

    /// Adding an attachment to the notification to display to user
    ///
    /// You can add local attachments by using `file` scheme.
    pub fn attachment<'a, U: Into<Url>>(&'a mut self, url: U) -> &'a mut AttachmentBuilder {
        let len = self.attachments.len();
        self.attachments.push(AttachmentBuilder::new(url.into()));
        self.attachments.get_mut(len).unwrap()
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
    pub fn interval<'a>(&'a mut self, duration: Duration) -> &'a mut Self {
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
    pub fn cron_one_time<'a>(&'a mut self, schedule: Schedule) -> &'a mut Self {
        self.trigger = Some(Trigger {
            kind: TriggerKind::Calendar(schedule),
            repeats: false,
        });
        self
    }

    /// Schedule the notification to repeat for a cron pattern
    /// The pattern should be in local timezone
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
    pub fn build(self) -> Notification {
        let identifier = if let Some(identifier) = self.identifier.clone().take() {
            identifier
        } else {
            let uuid_v4 = uuid::Uuid::new_v4();
            uuid_v4.to_string()
        };

        Notification {
            identifier,
            trigger: self.trigger,
            title: self.title,
            subtitle: self.subtitle,
            body: self.body,
            attachments: self
                .attachments
                .into_iter()
                .map(|a| a.build())
                .collect::<Vec<_>>(),
            user_info: self.user_data,
            thread_identifier: self.thread_identifier,
            category_identifier: self.category_identifier,
            summary_argument: self.summary_argument,
            summary_argument_count: self.summary_argument_count,
            launch_image_name: self.launch_image_name,
            badge: self.badge,
            target_content_identifier: self.target_content_identifier,
            sound: self.sound,
            interruption_level: self.interruption_level,
            relevance_score: self.relevance_score,
            filter_criteria: self.filter_criteria,
        }
    }
}

/// The builder for notification attachments
pub struct AttachmentBuilder {
    /// The unique identifier for the attachment.
    identifier: Option<String>,
    /// The URL of the file for this attachment.
    url: Url,
    options: Vec<AttachmentOptions>,
}

impl AttachmentBuilder {
    pub(crate) fn new(url: Url) -> AttachmentBuilder {
        AttachmentBuilder {
            url,
            options: Vec::new(),
            identifier: None,
        }
    }

    /// Unique identifier of the attachment
    pub fn identifier<'a, I: Into<String>>(&'a mut self, identififer: I) -> &'a mut Self {
        self.identifier = Some(identififer.into());
        self
    }

    fn overwrite_option<F>(&mut self, predicate: F, new: AttachmentOptions)
    where
        F: Fn(&AttachmentOptions) -> bool,
    {
        self.options.retain(|o| !predicate(o));
        self.options.push(new);
    }

    /// Hinting about the attachment type using a Uniform Type Identifier
    /// <https://en.wikipedia.org/wiki/Uniform_Type_Identifier>
    pub fn type_hint<'a, T: Into<String>>(&'a mut self, type_hint: T) -> &'a mut Self {
        self.overwrite_option(
            |o| matches!(o, AttachmentOptions::TypeHintKey(_)),
            AttachmentOptions::TypeHintKey(type_hint.into()),
        );
        self
    }

    /// A Boolean value indicating whether the system hides
    /// the attachment’s thumbnail.
    pub fn hide_thumbnail<'a>(&'a mut self, hidden: bool) -> &'a mut Self {
        self.overwrite_option(
            |o| matches!(o, AttachmentOptions::ThumbnailHiddenKey(_)),
            AttachmentOptions::ThumbnailHiddenKey(hidden),
        );
        self
    }

    /// Specify the time of the movie/video to capture the thumbnail
    pub fn video_thumbnail_at_time<'a>(&'a mut self, duration: Duration) -> &'a mut Self {
        self.overwrite_option(
            |o| matches!(o, AttachmentOptions::ThumbnailTimeKey(_)),
            AttachmentOptions::ThumbnailTimeKey(ThumbnailTimeKey::Time(duration)),
        );
        self
    }

    /// Capture the thumbnail from the begining of the video
    pub fn video_thumbnail_at_start<'a>(&'a mut self) -> &'a mut Self {
        self.overwrite_option(
            |o| matches!(o, AttachmentOptions::ThumbnailTimeKey(_)),
            AttachmentOptions::ThumbnailTimeKey(ThumbnailTimeKey::Start),
        );
        self
    }

    /// Capture the thumbnail from the end of the video
    pub fn video_thumbnail_at_end<'a>(&'a mut self) -> &'a mut Self {
        self.overwrite_option(
            |o| matches!(o, AttachmentOptions::ThumbnailTimeKey(_)),
            AttachmentOptions::ThumbnailTimeKey(ThumbnailTimeKey::End),
        );
        self
    }

    /// Capture the specific frame for thumbnail from the animated image
    pub fn animation_thumbnail_frame<'a>(&'a mut self, frame: u32) -> &'a mut Self {
        self.overwrite_option(
            |o| matches!(o, AttachmentOptions::ThumbnailTimeKey(_)),
            AttachmentOptions::ThumbnailTimeKey(ThumbnailTimeKey::FrameNumber(frame)),
        );
        self
    }

    /// Crop the thumbnail
    ///
    /// The coordinate system is starting from the left bottom corner. Right top corner is (1.0,
    /// 1.0)
    pub fn crop_thumbnail<'a>(&'a mut self, origin: (f32, f32), size: (f32, f32)) -> &'a mut Self {
        self.overwrite_option(
            |o| matches!(o, AttachmentOptions::ThumbnailClippingRectKey(_)),
            AttachmentOptions::ThumbnailClippingRectKey(ThumbnailClippingRect { origin, size }),
        );
        self
    }

    /// Building the attachment
    pub(crate) fn build(self) -> Attachment {
        let identifier = if let Some(identifier) = self.identifier.clone().take() {
            identifier
        } else {
            let uuid_v4 = uuid::Uuid::new_v4();
            uuid_v4.to_string()
        };

        Attachment {
            identifier,
            url: self.url,
            options: self.options,
        }
    }
}

/// Builder for notification categories
pub struct CategoryBuilder {
    /// The unique string assigned to the category.
    identifier: Option<String>,
    /// The actions to display when the system delivers
    /// notifications of this type.
    actions: Vec<ActionBuilder>,
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
    pub fn allow_announcements<'a>(&'a mut self) -> &'a mut Self {
        self.options |= CategoryOptions::AllowAnnouncement;
        self
    }

    /// A task your app performs in response to a notification that the system delivers.
    pub fn action<'a, S: Into<String>>(&'a mut self, title: S) -> &'a mut ActionBuilder {
        let action_builder = ActionBuilder::new_with_title(title);
        let len = self.actions.len();
        self.actions.push(action_builder);
        self.actions.get_mut(len).unwrap()
    }

    /// Building the category
    pub fn build(self) -> Category {
        let identifier = if let Some(identifier) = self.identifier.clone().take() {
            identifier
        } else {
            let uuid_v4 = uuid::Uuid::new_v4();
            uuid_v4.to_string()
        };
        Category {
            identifier,
            actions: self.actions.into_iter().map(|a| a.build()).collect(),
            intent_identifiers: self.intent_identifiers,
            options: self.options,
            hidden_preview_body_placeholder: self.hidden_preview_body_placeholder,
            category_summary_format: self.category_summary_format,
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
    pub(crate) fn new_with_title<S: Into<String>>(title: S) -> Self {
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

    pub(crate) fn build(self) -> Action {
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
    dbg!(&pattern);
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
