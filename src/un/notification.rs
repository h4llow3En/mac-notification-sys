//! Notification Types
use bitflags::bitflags;
use cron::{Schedule, TimeUnitSpec};
use icrate::Foundation::{
    CGPoint, CGRect, CGSize, NSDateComponents, NSMutableArray, NSMutableDictionary, NSNumber, NSURL,
};
use icrate::UniformTypeIdentifiers::{
    UTType, UTTypeAIFF, UTTypeAVI, UTTypeGIF, UTTypeJPEG, UTTypeMP3, UTTypeMPEG, UTTypeMPEG2Video,
    UTTypeMPEG4Audio, UTTypeMPEG4Movie, UTTypePNG, UTTypeWAV,
};
use icrate::UserNotifications::{
    UNCalendarNotificationTrigger, UNMutableNotificationContent, UNNotificationAction,
    UNNotificationActionIcon, UNNotificationAttachment,
    UNNotificationAttachmentOptionsThumbnailClippingRectKey,
    UNNotificationAttachmentOptionsThumbnailHiddenKey,
    UNNotificationAttachmentOptionsThumbnailTimeKey, UNNotificationAttachmentOptionsTypeHintKey,
    UNNotificationCategory, UNNotificationInterruptionLevel, UNNotificationInterruptionLevelActive,
    UNNotificationInterruptionLevelCritical, UNNotificationInterruptionLevelPassive,
    UNNotificationInterruptionLevelTimeSensitive, UNNotificationSound, UNNotificationTrigger,
    UNTimeIntervalNotificationTrigger,
};
use icrate::{Foundation::NSString, UserNotifications::UNNotificationRequest};
use objc2::runtime::Object;
use objc2::{rc::Id, ClassType};
use std::{collections::HashMap, time::Duration};
use url::Url;

use super::bind::CoreFoundation::kCFAllocatorDefault;
use super::bind::CoreGraphics::CGRectCreateDictionaryRepresentation;
use super::bind::CoreMedia::{
    kCMTimePositiveInfinity, kCMTimeZero, CMTime, CMTimeCopyAsDictionary, CMTimeMakeWithSeconds,
    NSEC_PER_SEC,
};
use crate::error::NotificationError;
use crate::os::{AppleOS, APPLE_VERSION};

/// The data for a local or remote notification the system delivers to your app.
#[derive(Debug, Clone, Default)]
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

impl TryInto<Id<UNNotificationRequest>> for Notification {
    type Error = NotificationError;

    fn try_into(self) -> Result<Id<UNNotificationRequest>, NotificationError> {
        let av = *APPLE_VERSION;
        unsafe {
            let notification_content =
                UNMutableNotificationContent::init(UNMutableNotificationContent::alloc());

            if let Some(title) = self.title {
                notification_content.setTitle(NSString::from_str(&title).as_ref());
            }

            if let Some(subtitle) = self.subtitle {
                notification_content.setSubtitle(NSString::from_str(&subtitle).as_ref());
            }

            notification_content.setBody(NSString::from_str(&self.body).as_ref());

            #[cfg(not(tvos))]
            {
                let mut un_attachments =
                    NSMutableArray::<UNNotificationAttachment>::init(NSMutableArray::alloc());
                for attachment in self.attachments {
                    let un_attachment_res: Result<Id<UNNotificationAttachment>, NotificationError> =
                        attachment.try_into();
                    let un_attachment = un_attachment_res?;
                    un_attachments.addObject(&un_attachment);
                }
                notification_content.setAttachments(&un_attachments);

                let mut ns_user_info =
                    NSMutableDictionary::<Object, Object>::init(NSMutableDictionary::alloc());
                for (k, v) in self.user_info {
                    let ns_k = NSString::from_str(&k);
                    let ns_v = NSString::from_str(&v);
                    ns_user_info.setObject_forKey(&ns_v, &ns_k);
                }
                notification_content.setUserInfo(ns_user_info.as_super());

                if let Some(thread) = self.thread_identifier {
                    let ns_str = NSString::from_str(&thread);
                    notification_content.setThreadIdentifier(&ns_str);
                }

                if let Some(category) = self.category_identifier {
                    let ns_str = NSString::from_str(&category);
                    notification_content.setCategoryIdentifier(&ns_str);
                }

                if let Some(sound) = self.sound {
                    let un_sound: Id<UNNotificationSound> = sound.into();
                    notification_content.setSound(Some(&un_sound));
                }
            }

            #[cfg(not(any(tvos, macos)))]
            {
                if let Some(launch_image_name) = self.launch_image_name {
                    let ns_str = NSString::from_str(&launch_image_name);
                    notification_content.setLaunchImageName(&ns_str);
                }
            }

            if let Some(badge) = self.badge {
                let ns_badge = NSNumber::new_usize(badge);
                notification_content.setBadge(Some(&ns_badge));
            }

            if av >= (AppleOS::IOS, 13, 0)
                || av >= (AppleOS::MacOS, 10, 15)
                || av >= (AppleOS::MacCatalyst, 13, 1)
                || av >= (AppleOS::TvOS, 13, 0)
                || av >= (AppleOS::WatchOS, 6, 0)
                || av >= (AppleOS::VisionOS, 1, 0)
            {
                if let Some(content_identifier) = self.target_content_identifier {
                    let ns_str = NSString::from_str(&content_identifier);
                    notification_content.setTargetContentIdentifier(Some(&ns_str));
                }
            }

            if (av >= (AppleOS::IOS, 12, 0) && av <= (AppleOS::IOS, 15, 0))
                || av >= (AppleOS::MacOS, 10, 14)
                || (av >= (AppleOS::MacCatalyst, 13, 1) && av <= (AppleOS::MacCatalyst, 15, 0))
                || (av >= (AppleOS::TvOS, 12, 0) && av <= (AppleOS::TvOS, 15, 0))
                || (av >= (AppleOS::WatchOS, 5, 0) && av <= (AppleOS::WatchOS, 8, 0))
                || (av >= (AppleOS::VisionOS, 1, 0) && av <= (AppleOS::VisionOS, 1, 0))
            {
                if let Some(sac) = self.summary_argument_count {
                    #[allow(deprecated)]
                    notification_content.setSummaryArgumentCount(sac);
                }

                if let Some(sa) = self.summary_argument {
                    let ns_str = NSString::from_str(&sa);
                    #[allow(deprecated)]
                    notification_content.setSummaryArgument(&ns_str);
                }
            }

            if av >= (AppleOS::IOS, 15, 0)
                || av >= (AppleOS::MacOS, 12, 0)
                || av >= (AppleOS::MacCatalyst, 15, 0)
                || av >= (AppleOS::TvOS, 15, 0)
                || av >= (AppleOS::WatchOS, 8, 0)
                || av >= (AppleOS::VisionOS, 1, 0)
            {
                if let Some(interruption_level) = self.interruption_level {
                    notification_content.setInterruptionLevel(interruption_level.into());
                }

                if let Some(relevance_score) = self.relevance_score {
                    notification_content.setRelevanceScore(relevance_score as f64);
                }
            }

            if av >= (AppleOS::IOS, 16, 0)
                || av >= (AppleOS::MacOS, 13, 0)
                || av >= (AppleOS::MacCatalyst, 16, 0)
                || av >= (AppleOS::TvOS, 16, 0)
                || av >= (AppleOS::WatchOS, 9, 0)
                || av >= (AppleOS::VisionOS, 1, 0)
            {
                if let Some(filter_criteria) = self.filter_criteria {
                    let ns_str = NSString::from_str(&filter_criteria);
                    notification_content.setFilterCriteria(Some(&ns_str));
                }
            }

            let un_trigger: Option<Id<UNNotificationTrigger>> = self.trigger.map(|t| t.into());
            let un_trigger = un_trigger.as_ref().map(|t| t.as_ref());

            let ns_identifier = NSString::from_str(&self.identifier);
            let request = UNNotificationRequest::requestWithIdentifier_content_trigger(
                &ns_identifier,
                &notification_content,
                un_trigger,
            );

            Ok(request)
        }
    }
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

impl Into<Id<UNNotificationSound>> for Sound {
    fn into(self) -> Id<UNNotificationSound> {
        let av = *APPLE_VERSION;
        unsafe {
            use Sound::*;
            match self {
                Default => UNNotificationSound::defaultSound(),
                DefaultCriticalSound => {
                    if av >= (AppleOS::IOS, 12, 0)
                        || av >= (AppleOS::MacOS, 10, 14)
                        || av >= (AppleOS::MacCatalyst, 13, 1)
                        || av >= (AppleOS::VisionOS, 1, 0)
                        || av >= (AppleOS::WatchOS, 5, 0)
                    {
                        UNNotificationSound::defaultCriticalSound()
                    } else {
                        UNNotificationSound::defaultSound()
                    }
                }
                DefaultRingtone => {
                    #[cfg(ios)]
                    let sound = if av >= (AppleOS::IOS, 15, 2) || av >= (AppleOS::IOS, 15, 2) {
                        UNNotificationSound::defaultRingtoneSound()
                    } else {
                        UNNotificationSound::defaultSound()
                    };
                    #[cfg(not(ios))]
                    let sound = UNNotificationSound::defaultSound();
                    sound
                }
                DefaultCriticalSoundWithVolume(volume) => {
                    if av >= (AppleOS::IOS, 12, 0)
                        || av >= (AppleOS::MacOS, 10, 14)
                        || av >= (AppleOS::MacCatalyst, 13, 1)
                        || av >= (AppleOS::VisionOS, 1, 0)
                        || av >= (AppleOS::WatchOS, 5, 0)
                    {
                        UNNotificationSound::defaultCriticalSoundWithAudioVolume(volume)
                    } else {
                        UNNotificationSound::defaultSound()
                    }
                }
                Named(name) => {
                    #[cfg(not(watchos))]
                    let sound = {
                        let ns_str = NSString::from_str(&name);
                        UNNotificationSound::soundNamed(&ns_str)
                    };
                    #[cfg(watchos)]
                    let sound = UNNotificationSound::defaultSound();
                    sound
                }
                CriticalSoundNamed(name) => {
                    #[cfg(not(watchos))]
                    let sound = if av >= (AppleOS::IOS, 12, 0)
                        || av >= (AppleOS::MacOS, 10, 14)
                        || av >= (AppleOS::MacCatalyst, 13, 1)
                        || av >= (AppleOS::VisionOS, 1, 0)
                    {
                        let ns_str = NSString::from_str(&name);
                        UNNotificationSound::criticalSoundNamed(&ns_str)
                    } else {
                        UNNotificationSound::defaultSound()
                    };
                    #[cfg(watchos)]
                    let sound = UNNotificationSound::defaultSound();
                    sound
                }
                CriticalSoundNamedWithVolume(name, volume) => {
                    #[cfg(not(watchos))]
                    let sound = if av >= (AppleOS::IOS, 12, 0)
                        || av >= (AppleOS::MacOS, 10, 14)
                        || av >= (AppleOS::MacCatalyst, 13, 1)
                        || av >= (AppleOS::VisionOS, 1, 0)
                    {
                        let ns_str = NSString::from_str(&name);
                        UNNotificationSound::criticalSoundNamed_withAudioVolume(&ns_str, volume)
                    } else {
                        UNNotificationSound::defaultSound()
                    };
                    #[cfg(watchos)]
                    let sound = UNNotificationSound::defaultSound();
                    sound
                }
                RingtoneSoundNamed(_name) => {
                    #[cfg(ios)]
                    let sound = if av >= (AppleOS::IOS, 15, 2) || av >= (AppleOS::IOS, 15, 2) {
                        let ns_str = NSString::from_str(&_name);
                        UNNotificationSound::ringtoneSoundNamed(&ns_str)
                    } else {
                        UNNotificationSound::defaultSound()
                    };
                    #[cfg(not(ios))]
                    let sound = UNNotificationSound::defaultSound();
                    sound
                }
            }
        }
    }
}

/// Supported audio types as attachments
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum AudioFormat {
    /// Waveform Audio File Format
    WaveformAudio,
    /// MP3
    MP3,
    /// MPEG-4 includes a system for handling a diverse group of
    /// audio formats in a uniform matter.
    MPEG4Audio,
    /// Audio Interchange File Format
    AudioInterchangeFileFormat,
}

impl AudioFormat {
    unsafe fn to_uttype(&self) -> &'static UTType {
        match self {
            AudioFormat::WaveformAudio => UTTypeWAV,
            AudioFormat::MP3 => UTTypeMP3,
            AudioFormat::MPEG4Audio => UTTypeMPEG4Audio,
            AudioFormat::AudioInterchangeFileFormat => UTTypeAIFF,
        }
    }
}

/// Supported video types as attachments
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum VideoFormat {
    /// MPEG-1 or MPEG-2 movie
    MPEG,
    /// MPEG-2 video
    MPEG2Video,
    /// MPEG-4 video
    MPEG4,
    /// AVI movie format
    AVIMovie,
}

impl VideoFormat {
    unsafe fn to_uttype(&self) -> &'static UTType {
        match self {
            VideoFormat::MPEG => UTTypeMPEG,
            VideoFormat::AVIMovie => UTTypeAVI,
            VideoFormat::MPEG4 => UTTypeMPEG4Movie,
            VideoFormat::MPEG2Video => UTTypeMPEG2Video,
        }
    }
}

/// Supported non-animated image types as attachments
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ImageFormat {
    /// PNG images
    PNG,
    /// JPEG images
    JPEG,
}

impl ImageFormat {
    unsafe fn to_uttype(&self) -> &'static UTType {
        match self {
            ImageFormat::PNG => UTTypePNG,
            ImageFormat::JPEG => UTTypeJPEG,
        }
    }
}

/// Supported animated image types as attachments
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum AnimatedImageFormat {
    /// Animated portable images
    PNG,
    /// Animated GIF images
    GIF,
}

impl AnimatedImageFormat {
    unsafe fn to_uttype(&self) -> &'static UTType {
        match self {
            AnimatedImageFormat::PNG => UTTypePNG,
            AnimatedImageFormat::GIF => UTTypeGIF,
        }
    }
}

/// A time to capture a frame for a
/// thumbnail from a attached video
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum VideoTime {
    /// First frame of the video
    Start,
    /// Last frame of the video
    End,
    /// Frame at a specific time
    Time(Duration),
}

/// Common functionalities for all the attachment option types
pub trait UnifiedAttachmentOptions {
    /// Allowed Uniform Type Identifiers
    type Format;

    /// Manually specifying the encoded format when
    /// the file extension was not provided in the url
    fn set_format(&mut self, format: Self::Format);
}

/// Common functionalities for attachment options with thumbnails
pub trait ThumbnailedAttachmentOptions {
    /// Hiding the thumbnail of the attachment
    fn hide_thumbnail(&mut self, hide: bool);

    /// Crop the thumbnail to show a specific area of the attachment
    fn crop_thumbnail(&mut self, rect: ThumbnailClippingRect);
}

/// Audio attachment options
#[derive(Debug, Clone, Default)]
pub struct AudioAttachmentOptions {
    /// Specify the audio encoded format if the URL does not
    /// containing a file extension
    format: Option<AudioFormat>,
}

impl UnifiedAttachmentOptions for AudioAttachmentOptions {
    type Format = AudioFormat;

    fn set_format(&mut self, format: Self::Format) {
        self.format = Some(format);
    }
}

/// Video attachment options
#[derive(Debug, Clone, Default)]
pub struct VideoAttachmentOptions {
    /// Specify the video encoded format if the URL does not
    /// containing a file extension
    format: Option<VideoFormat>,
    /// Time snapshot to take the frame for thumbnail
    pub(crate) thumbnail_time: Option<VideoTime>,
    /// Which part should display in the thumbnail
    thumbnail_crop: Option<ThumbnailClippingRect>,
    /// Hide the the thumbnail
    thumbnail_hide: Option<bool>,
}

impl UnifiedAttachmentOptions for VideoAttachmentOptions {
    type Format = VideoFormat;

    fn set_format(&mut self, format: Self::Format) {
        self.format = Some(format);
    }
}

impl ThumbnailedAttachmentOptions for VideoAttachmentOptions {
    fn hide_thumbnail(&mut self, hide: bool) {
        self.thumbnail_hide = Some(hide);
    }

    fn crop_thumbnail(&mut self, rect: ThumbnailClippingRect) {
        self.thumbnail_crop = Some(rect);
    }
}

/// Image attachment options
#[derive(Debug, Clone, Default)]
pub struct ImageAttachmentOptions {
    /// Specify the image encoded format if the URL does not
    /// containing a file extension
    format: Option<ImageFormat>,
    /// Which part should display in the thumbnail
    thumbnail_crop: Option<ThumbnailClippingRect>,
    /// Hide the the thumbnail
    thumbnail_hide: Option<bool>,
}

impl UnifiedAttachmentOptions for ImageAttachmentOptions {
    type Format = ImageFormat;

    fn set_format(&mut self, format: Self::Format) {
        self.format = Some(format);
    }
}

impl ThumbnailedAttachmentOptions for ImageAttachmentOptions {
    fn hide_thumbnail(&mut self, hide: bool) {
        self.thumbnail_hide = Some(hide);
    }

    fn crop_thumbnail(&mut self, rect: ThumbnailClippingRect) {
        self.thumbnail_crop = Some(rect);
    }
}

/// Animated image attachment options
#[derive(Debug, Clone, Default)]
pub struct AnimatedImageAttachmentOptions {
    /// Specify the image encoded format if the URL does not
    /// containing a file extension
    format: Option<AnimatedImageFormat>,
    /// The frame number to display as the thumbnail
    pub(crate) thumbnail_frame: Option<u64>,
    /// Which part should display in the thumbnail
    thumbnail_crop: Option<ThumbnailClippingRect>,
    /// Hide the the thumbnail
    thumbnail_hide: Option<bool>,
}

impl UnifiedAttachmentOptions for AnimatedImageAttachmentOptions {
    type Format = AnimatedImageFormat;

    fn set_format(&mut self, format: Self::Format) {
        self.format = Some(format);
    }
}

impl ThumbnailedAttachmentOptions for AnimatedImageAttachmentOptions {
    fn hide_thumbnail(&mut self, hide: bool) {
        self.thumbnail_hide = Some(hide);
    }

    fn crop_thumbnail(&mut self, rect: ThumbnailClippingRect) {
        self.thumbnail_crop = Some(rect);
    }
}

/// Options to change the behavior of the
/// attachment visualization
#[derive(Debug, Clone)]
pub enum AttachmentOptions {
    /// Audio attachment options
    Audio(AudioAttachmentOptions),
    /// Video attachment options
    Video(VideoAttachmentOptions),
    /// Image attachment options
    Image(ImageAttachmentOptions),
    /// Animated image attachment options
    AnimatedImage(AnimatedImageAttachmentOptions),
}

/// Attachments to display in notification
#[derive(Debug, Clone)]
pub struct Attachment {
    /// The unique identifier for the attachment.
    pub(crate) identifier: String,
    /// The URL of the file for this attachment.
    pub(crate) url: Url,
    pub(crate) options: Option<AttachmentOptions>,
}

impl TryInto<Id<UNNotificationAttachment>> for Attachment {
    type Error = NotificationError;

    fn try_into(self) -> Result<Id<UNNotificationAttachment>, NotificationError> {
        unsafe {
            let identifier = NSString::from_str(&self.identifier);
            let url_str: String = self.url.into();
            let ns_url_str = NSString::from_str(&url_str);
            let ns_url = NSURL::URLWithString(&ns_url_str).unwrap();

            let options = if let Some(opt) = self.options {
                let mut options: Id<NSMutableDictionary<Object, Object>> =
                    NSMutableDictionary::init(NSMutableDictionary::alloc());
                let ut_type: Option<&UTType> = match &opt {
                    AttachmentOptions::Audio(audio_opt) => audio_opt.format.map(|f| f.to_uttype()),
                    AttachmentOptions::Video(vide_opt) => vide_opt.format.map(|f| f.to_uttype()),
                    AttachmentOptions::Image(img_opt) => img_opt.format.map(|f| f.to_uttype()),
                    AttachmentOptions::AnimatedImage(aimg_opt) => {
                        aimg_opt.format.map(|f| f.to_uttype())
                    }
                };

                if let Some(ut_type) = ut_type {
                    let ns_str = ut_type.identifier();
                    options.setObject_forKey(
                        ns_str.as_super(),
                        UNNotificationAttachmentOptionsTypeHintKey,
                    );
                }

                match &opt {
                    AttachmentOptions::Video(VideoAttachmentOptions {
                        thumbnail_hide: Some(hide),
                        ..
                    })
                    | AttachmentOptions::Image(ImageAttachmentOptions {
                        thumbnail_hide: Some(hide),
                        ..
                    })
                    | AttachmentOptions::AnimatedImage(AnimatedImageAttachmentOptions {
                        thumbnail_hide: Some(hide),
                        ..
                    }) => {
                        let ns_num = NSNumber::new_bool(*hide);
                        let obj = ns_num.as_super().as_super();
                        options.setObject_forKey(
                            &obj,
                            UNNotificationAttachmentOptionsThumbnailHiddenKey,
                        );
                    }
                    _ => {}
                }

                match &opt {
                    AttachmentOptions::Video(VideoAttachmentOptions {
                        thumbnail_crop: Some(clipping_rect),
                        ..
                    })
                    | AttachmentOptions::Image(ImageAttachmentOptions {
                        thumbnail_crop: Some(clipping_rect),
                        ..
                    })
                    | AttachmentOptions::AnimatedImage(AnimatedImageAttachmentOptions {
                        thumbnail_crop: Some(clipping_rect),
                        ..
                    }) => {
                        let cg_point = CGPoint::new(
                            clipping_rect.origin.0 as f64,
                            clipping_rect.origin.1 as f64,
                        );
                        let cg_size =
                            CGSize::new(clipping_rect.size.0 as f64, clipping_rect.size.1 as f64);
                        let cg_rect = CGRect::new(cg_point, cg_size);
                        let raw_rect: *const CGRect = &cg_rect;
                        let ns_dic = CGRectCreateDictionaryRepresentation(raw_rect)
                            .as_ref()
                            .unwrap();
                        let obj = ns_dic.as_super().as_super();
                        options.setObject_forKey(
                            obj,
                            UNNotificationAttachmentOptionsThumbnailClippingRectKey,
                        );
                    }
                    _ => {}
                };

                if let AttachmentOptions::AnimatedImage(AnimatedImageAttachmentOptions {
                    thumbnail_frame: Some(frame),
                    ..
                }) = &opt
                {
                    let ns_num = NSNumber::new_u64(*frame);
                    let obj = ns_num.as_super().as_super().as_super();
                    options.setObject_forKey(obj, UNNotificationAttachmentOptionsThumbnailTimeKey);
                }

                if let AttachmentOptions::Video(VideoAttachmentOptions {
                    thumbnail_time: Some(time),
                    ..
                }) = &opt
                {
                    match time {
                        VideoTime::Start => {
                            if !(*APPLE_VERSION < (AppleOS::WatchOS, 6, 0)) {
                                let time = kCMTimeZero;
                                let time_ptr: *const CMTime = &time;
                                let dict_ref =
                                    CMTimeCopyAsDictionary(time_ptr, kCFAllocatorDefault);
                                let dict = dict_ref.as_ref().unwrap();
                                let obj = dict.as_super().as_super();
                                options.setObject_forKey(
                                    obj,
                                    UNNotificationAttachmentOptionsThumbnailTimeKey,
                                );
                            }
                        }
                        VideoTime::End => {
                            if !(*APPLE_VERSION < (AppleOS::WatchOS, 6, 0)) {
                                let time = kCMTimePositiveInfinity;
                                let time_ptr: *const CMTime = &time;
                                let dict_ref =
                                    CMTimeCopyAsDictionary(time_ptr, kCFAllocatorDefault);
                                let dict = dict_ref.as_ref().unwrap();
                                let obj = dict.as_super().as_super();
                                options.setObject_forKey(
                                    obj,
                                    UNNotificationAttachmentOptionsThumbnailTimeKey,
                                );
                            }
                        }
                        VideoTime::Time(dur) => {
                            if !(*APPLE_VERSION < (AppleOS::WatchOS, 6, 0)) {
                                let nsec_per_sec: *const i32 = &(NSEC_PER_SEC as i32);
                                let time = CMTimeMakeWithSeconds(
                                    (dur.as_nanos() as f64) / (NSEC_PER_SEC as f64),
                                    nsec_per_sec,
                                );
                                let time_ptr: *const CMTime = &time;
                                let dict_ref =
                                    CMTimeCopyAsDictionary(time_ptr, kCFAllocatorDefault);
                                let dict = dict_ref.as_ref().unwrap();
                                let obj = dict.as_super().as_super();
                                options.setObject_forKey(
                                    obj,
                                    UNNotificationAttachmentOptionsThumbnailTimeKey,
                                );
                            }
                        }
                    }
                }

                Some(options)
            } else {
                None
            };

            let result = UNNotificationAttachment::attachmentWithIdentifier_URL_options_error(
                &identifier,
                &ns_url,
                options.as_ref().map(|o| o.as_super()),
            );
            result.map_err(NotificationError::from)
        }
    }
}

/// The trigger function
#[derive(Debug, Clone)]
pub enum TriggerKind {
    /// A trigger condition that causes the system to deliver
    /// a notification after the amount of time you specify elapses.
    ///
    /// The minimum time interval is 1 minute if you are using this
    /// with the repeat mode. It was not documented anywhere. But
    /// there was an internal assertion in the objective C API
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

impl Into<Id<UNNotificationTrigger>> for Trigger {
    fn into(self) -> Id<UNNotificationTrigger> {
        fn get_first_cron_unit<TU: TimeUnitSpec>(tu: &TU) -> Option<u32> {
            if !tu.is_all() {
                let mut unit_iter = tu.iter();
                let first_unit = unit_iter.next().expect("invalid cron pattern");
                let second_unit = unit_iter.next();
                // Since we are not supporting ranges and backslashes,
                // there should be no second unit
                if second_unit.is_some() {
                    panic!("Unsupported cron pattern");
                }
                return Some(first_unit);
            }
            return None;
        }

        unsafe {
            match self.kind {
                TriggerKind::TimeInterval(dur) => {
                    let time_interval_trigger =
                        UNTimeIntervalNotificationTrigger::triggerWithTimeInterval_repeats(
                            dur.as_secs_f64(),
                            self.repeats,
                        );
                    let trigger = time_interval_trigger.as_super();
                    trigger.retain()
                }
                TriggerKind::Calendar(schedule) => {
                    let ns_date = NSDateComponents::new();
                    if let Some(year) = get_first_cron_unit(schedule.years()) {
                        ns_date.setYear(year as isize);
                    }

                    if let Some(month) = get_first_cron_unit(schedule.months()) {
                        ns_date.setMonth(month as isize);
                    }

                    if let Some(day) = get_first_cron_unit(schedule.days_of_month()) {
                        ns_date.setMonth(day as isize);
                    }

                    if let Some(hour) = get_first_cron_unit(schedule.hours()) {
                        ns_date.setHour(hour as isize);
                    }

                    if let Some(minute) = get_first_cron_unit(schedule.minutes()) {
                        ns_date.setMinute(minute as isize);
                    }

                    if let Some(second) = get_first_cron_unit(schedule.seconds()) {
                        ns_date.setSecond(second as isize);
                    }

                    if let Some(day_of_week) = get_first_cron_unit(schedule.days_of_week()) {
                        ns_date.setWeekdayOrdinal(day_of_week as isize);
                    }

                    let calendar_trigger =
                        UNCalendarNotificationTrigger::triggerWithDateMatchingComponents_repeats(
                            &ns_date,
                            self.repeats,
                        );

                    let trigger = calendar_trigger.as_super();
                    trigger.retain()
                }
            }
        }
    }
}

/// A type of notification your app supports and the custom
/// actions that the system displays.
#[derive(Debug, Clone, Default)]
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

impl Into<Id<UNNotificationCategory>> for Category {
    fn into(self) -> Id<UNNotificationCategory> {
        unsafe {
            let ns_identifier = NSString::from_str(&self.identifier);
            let opt = self.options.bits();

            #[cfg(not(tvos))]
            let (intent_identifiers, actions) = {
                let mut intent_identifiers =
                    NSMutableArray::<NSString>::init(NSMutableArray::alloc());
                for intent_ident in self.intent_identifiers {
                    let ns_str = NSString::from_str(&intent_ident);
                    intent_identifiers.addObject(&ns_str);
                }

                let mut actions =
                    NSMutableArray::<UNNotificationAction>::init(NSMutableArray::alloc());
                for action in self.actions {
                    let un_action: Id<UNNotificationAction> = action.into();
                    actions.addObject(&un_action);
                }

                (intent_identifiers, actions)
            };
            #[cfg(tvos)]
            let (intent_identifiers, actions) = {
                let intent_identifiers = NSMutableArray::init(NSMutableArray::alloc());
                let actions = NSMutableArray::init(NSMutableArray::alloc());

                (intent_identifiers, actions)
            };

            let ns_hpbp = self
                .hidden_preview_body_placeholder
                .as_ref()
                .map(|s| NSString::from_str(s));
            let ns_hpbp = ns_hpbp.as_ref().map(|s| s.as_ref());

            let ns_csf = self
                .category_summary_format
                .as_ref()
                .map(|s| NSString::from_str(s));
            let ns_csf = ns_csf.as_ref().map(|s| s.as_ref());

            UNNotificationCategory::categoryWithIdentifier_actions_intentIdentifiers_hiddenPreviewsBodyPlaceholder_categorySummaryFormat_options(
                &ns_identifier, &actions, &intent_identifiers, ns_hpbp, ns_csf, opt as usize)
        }
    }
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

impl Into<Id<UNNotificationAction>> for Action {
    fn into(self) -> Id<UNNotificationAction> {
        unsafe {
            let ns_identifier = NSString::from_str(&self.identifier);
            let ns_title = NSString::from_str(&self.title);
            let options = self.options.bits();
            let av = *APPLE_VERSION;
            let ns_icon = if av >= (AppleOS::IOS, 15, 0)
                || av >= (AppleOS::MacOS, 12, 0)
                || av >= (AppleOS::MacCatalyst, 15, 0)
                || av >= (AppleOS::TvOS, 15, 0)
                || av >= (AppleOS::WatchOS, 8, 0)
                || av >= (AppleOS::VisionOS, 1, 0)
            {
                if let Some(icon) = self.icon {
                    let ns_action_icon = match icon {
                        ActionIcon::SystemImageName(system_image) => {
                            let ns_system_image = NSString::from_str(&system_image);
                            UNNotificationActionIcon::iconWithSystemImageName(&ns_system_image)
                        }
                        ActionIcon::TemplateImageName(template_image) => {
                            let ns_template_image = NSString::from_str(&template_image);
                            UNNotificationActionIcon::iconWithTemplateImageName(&ns_template_image)
                        }
                    };

                    Some(ns_action_icon)
                } else {
                    None
                }
            } else {
                None
            };

            if let Some(ns_icon) = ns_icon {
                UNNotificationAction::actionWithIdentifier_title_options_icon(
                    &ns_identifier,
                    &ns_title,
                    options as usize,
                    Some(&ns_icon),
                )
            } else {
                UNNotificationAction::actionWithIdentifier_title_options(
                    &ns_identifier,
                    &ns_title,
                    options as usize,
                )
            }
        }
    }
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
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, Default)]
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

impl Into<UNNotificationInterruptionLevel> for InterruptionLevel {
    fn into(self) -> UNNotificationInterruptionLevel {
        use InterruptionLevel::*;
        match self {
            Active => UNNotificationInterruptionLevelActive,
            Critical => UNNotificationInterruptionLevelCritical,
            Passive => UNNotificationInterruptionLevelPassive,
            TimeSensitive => UNNotificationInterruptionLevelTimeSensitive,
        }
    }
}

#[cfg(test)]
mod tests {
    use std::str::FromStr;
    use std::collections::HashMap;

    use super::{
        Action, ActionIcon, ActionOptions, AnimatedImageAttachmentOptions, AnimatedImageFormat,
        Attachment, AttachmentOptions, AudioAttachmentOptions, AudioFormat, ImageAttachmentOptions,
        ImageFormat, Notification, Sound, ThumbnailClippingRect, Trigger, TriggerKind,
        VideoAttachmentOptions, VideoFormat, VideoTime, InterruptionLevel, Category, CategoryOptions
    };
    use crate::error::NotificationError;
    use cron::Schedule;
    use icrate::UserNotifications::{
        UNNotificationAction, UNNotificationAttachment, UNNotificationRequest, UNNotificationSound,
        UNNotificationTrigger, UNNotificationCategory
    };
    use objc2::rc::Id;

    fn execute_cron_conversion<'a>(pattern: &'a str) {
        let cron_schedule = Schedule::from_str(pattern).unwrap();
        let trigger = Trigger {
            kind: TriggerKind::Calendar(cron_schedule),
            repeats: false,
        };
        let _: Id<UNNotificationTrigger> = trigger.into();
    }

    #[test]
    pub fn test_trigger_conversion_with_any() {
        execute_cron_conversion("* * * * * * *");
    }

    #[test]
    pub fn test_trigger_conversion_with_specific_date() {
        execute_cron_conversion("* * * 1 Dec * 2019");
    }

    #[test]
    pub fn test_trigger_conversion_with_specific_datetime() {
        execute_cron_conversion("30 45 15 1 Dec * 2019");
    }

    #[test]
    #[should_panic]
    pub fn test_trigger_conversion_with_interval() {
        execute_cron_conversion("30 45 15 1 Dec * 2019/2");
    }

    #[test]
    #[should_panic]
    pub fn test_trigger_conversion_with_range() {
        execute_cron_conversion("30 45 15 1 Dec * 2019-2023");
    }

    #[test]
    #[should_panic]
    pub fn test_trigger_conversion_with_all_interval() {
        execute_cron_conversion("30 45 15 1 Dec * */2");
    }

    #[test]
    #[cfg(not(otheros))]
    pub fn test_convert_sound_default() {
        let sound = Sound::Default;
        let _un_sound: Id<UNNotificationSound> = sound.into();
    }

    #[test]
    #[cfg(not(otheros))]
    pub fn test_convert_sound_named() {
        let sound = Sound::Named(String::from("test"));
        let _un_sound: Id<UNNotificationSound> = sound.into();
    }

    #[test]
    #[cfg(not(otheros))]
    pub fn test_convert_sound_default_critical() {
        let sound = Sound::DefaultCriticalSound;
        let _un_sound: Id<UNNotificationSound> = sound.into();
    }

    #[test]
    #[cfg(not(otheros))]
    pub fn test_convert_sound_named_critical() {
        let sound = Sound::CriticalSoundNamed(String::from("test"));
        let _un_sound: Id<UNNotificationSound> = sound.into();
    }

    #[test]
    #[cfg(not(otheros))]
    pub fn test_convert_sound_named_volume_critical() {
        let sound = Sound::CriticalSoundNamedWithVolume(String::from("test"), 0.1);
        let _un_sound: Id<UNNotificationSound> = sound.into();
    }

    #[test]
    #[cfg(not(otheros))]
    pub fn test_convert_sound_default_ringtone() {
        let sound = Sound::DefaultRingtone;
        let _un_sound: Id<UNNotificationSound> = sound.into();
    }

    #[test]
    #[cfg(not(otheros))]
    pub fn test_convert_sound_named_ringtone() {
        let sound = Sound::RingtoneSoundNamed(String::from("test"));
        let _un_sound: Id<UNNotificationSound> = sound.into();
    }

    #[test]
    #[cfg(not(otheros))]
    pub fn test_convert_sound_default_critical_with_volume() {
        let sound = Sound::DefaultCriticalSoundWithVolume(0.1);
        let _un_sound: Id<UNNotificationSound> = sound.into();
    }

    #[test]
    #[cfg(not(otheros))]
    pub fn test_convert_trigger_with_time_interval() {
        let trigger = Trigger {
            kind: TriggerKind::TimeInterval(std::time::Duration::from_secs(30)),
            repeats: false,
        };
        let _un_trigger: Id<UNNotificationTrigger> = trigger.into();
    }

    #[test]
    #[cfg(not(otheros))]
    pub fn test_convert_trigger_with_time_interval_repeats() {
        // Failed if the duration less than 1 minute
        let trigger = Trigger {
            kind: TriggerKind::TimeInterval(std::time::Duration::from_secs(60)),
            repeats: true,
        };
        let _un_trigger: Id<UNNotificationTrigger> = trigger.into();
    }

    #[test]
    #[cfg(not(otheros))]
    pub fn test_convert_trigger_with_cron() {
        let cron_schedule = Schedule::try_from("30 45 18 * * * *").unwrap();
        let trigger = Trigger {
            kind: TriggerKind::Calendar(cron_schedule),
            repeats: false,
        };
        let _un_trigger: Id<UNNotificationTrigger> = trigger.into();
    }

    #[test]
    #[cfg(not(otheros))]
    pub fn test_convert_trigger_with_cron_repeats() {
        let cron_schedule = Schedule::try_from("30 45 18 * * * *").unwrap();
        let trigger = Trigger {
            kind: TriggerKind::Calendar(cron_schedule),
            repeats: true,
        };
        let _un_trigger: Id<UNNotificationTrigger> = trigger.into();
    }

    fn execute_attachment_test(
        file_name_str: &str,
        options: Option<AttachmentOptions>,
    ) -> Result<Id<UNNotificationAttachment>, NotificationError> {
        let path = format!(
            "file://{}/resources/{}",
            std::env::current_dir().unwrap().to_str().unwrap(),
            file_name_str
        );
        let image_url = url::Url::parse(&path).unwrap();
        let attachment = Attachment {
            url: image_url,
            identifier: String::from("Test"),
            options,
        };
        let un_trigger: Result<Id<UNNotificationAttachment>, NotificationError> =
            attachment.try_into();
        un_trigger
    }

    #[test]
    #[cfg(not(otheros))]
    pub fn test_convert_attachment_image() {
        assert!(execute_attachment_test("test.png", None).is_ok());
    }

    #[test]
    #[cfg(not(otheros))]
    pub fn test_convert_attachment_image_crop() {
        let res = execute_attachment_test(
            "test.png",
            Some(AttachmentOptions::Image(ImageAttachmentOptions {
                thumbnail_crop: Some(ThumbnailClippingRect {
                    origin: (0.25, 0.25),
                    size: (0.5, 0.5),
                }),
                ..Default::default()
            })),
        );
        assert!(res.is_ok());
    }

    #[test]
    #[cfg(not(otheros))]
    pub fn test_convert_attachment_image_hidden() {
        let res = execute_attachment_test(
            "test.png",
            Some(AttachmentOptions::Image(ImageAttachmentOptions {
                thumbnail_hide: Some(true),
                ..Default::default()
            })),
        );
        assert!(res.is_ok());

        let res = execute_attachment_test(
            "test.png",
            Some(AttachmentOptions::Image(ImageAttachmentOptions {
                thumbnail_hide: Some(false),
                ..Default::default()
            })),
        );
        assert!(res.is_ok());
    }

    #[test]
    #[cfg(not(otheros))]
    pub fn test_convert_attachment_image_type_hint() {
        let res = execute_attachment_test(
            "test",
            Some(AttachmentOptions::Image(ImageAttachmentOptions {
                format: Some(ImageFormat::JPEG),
                ..Default::default()
            })),
        );
        assert!(res.is_ok());

        let res = execute_attachment_test(
            "test",
            Some(AttachmentOptions::Image(ImageAttachmentOptions {
                format: Some(ImageFormat::PNG),
                ..Default::default()
            })),
        );
        assert!(res.is_ok());
    }

    #[test]
    #[cfg(not(otheros))]
    pub fn test_convert_attachment_animated_image_cropped() {
        let res = execute_attachment_test(
            "test.gif",
            Some(AttachmentOptions::AnimatedImage(
                AnimatedImageAttachmentOptions {
                    thumbnail_crop: Some(ThumbnailClippingRect {
                        origin: (0.25, 0.25),
                        size: (0.5, 0.5),
                    }),
                    ..Default::default()
                },
            )),
        );
        assert!(res.is_ok());
    }

    #[test]
    #[cfg(not(otheros))]
    pub fn test_convert_attachment_animated_image_hidden() {
        let res = execute_attachment_test(
            "test.gif",
            Some(AttachmentOptions::AnimatedImage(
                AnimatedImageAttachmentOptions {
                    thumbnail_hide: Some(true),
                    ..Default::default()
                },
            )),
        );
        assert!(res.is_ok());

        let res = execute_attachment_test(
            "test.gif",
            Some(AttachmentOptions::AnimatedImage(
                AnimatedImageAttachmentOptions {
                    thumbnail_hide: Some(false),
                    ..Default::default()
                },
            )),
        );
        assert!(res.is_ok());
    }

    #[test]
    #[cfg(not(otheros))]
    pub fn test_convert_attachment_animated_image_type_hint() {
        let res = execute_attachment_test(
            "test.gif",
            Some(AttachmentOptions::AnimatedImage(
                AnimatedImageAttachmentOptions {
                    format: Some(AnimatedImageFormat::PNG),
                    ..Default::default()
                },
            )),
        );
        assert!(res.is_ok());

        let res = execute_attachment_test(
            "test.gif",
            Some(AttachmentOptions::AnimatedImage(
                AnimatedImageAttachmentOptions {
                    format: Some(AnimatedImageFormat::GIF),
                    ..Default::default()
                },
            )),
        );
        assert!(res.is_ok());
    }

    #[test]
    #[cfg(not(otheros))]
    pub fn test_convert_attachment_animated_image_frame_number() {
        let res = execute_attachment_test(
            "test.gif",
            Some(AttachmentOptions::AnimatedImage(
                AnimatedImageAttachmentOptions {
                    thumbnail_frame: Some(1000),
                    ..Default::default()
                },
            )),
        );
        assert!(res.is_ok());
    }

    #[test]
    #[cfg(not(otheros))]
    pub fn test_convert_attachment_animated_image_cropped_frame_number() {
        let res = execute_attachment_test(
            "test.gif",
            Some(AttachmentOptions::AnimatedImage(
                AnimatedImageAttachmentOptions {
                    thumbnail_crop: Some(ThumbnailClippingRect {
                        origin: (0.25, 0.25),
                        size: (0.5, 0.5),
                    }),
                    thumbnail_frame: Some(1000),
                    ..Default::default()
                },
            )),
        );
        assert!(res.is_ok());
    }

    #[test]
    #[cfg(not(otheros))]
    pub fn test_convert_attachment_movie() {
        let res = execute_attachment_test("test.mp4", None);
        assert!(res.is_ok());
    }

    #[test]
    #[cfg(not(otheros))]
    pub fn test_convert_attachment_movie_time_start() {
        let res = execute_attachment_test(
            "test.mp4",
            Some(AttachmentOptions::Video(VideoAttachmentOptions {
                thumbnail_time: Some(VideoTime::Start),
                ..Default::default()
            })),
        );
        assert!(res.is_ok());
    }

    #[test]
    #[cfg(not(otheros))]
    pub fn test_convert_attachment_movie_time_end() {
        let res = execute_attachment_test(
            "test.mp4",
            Some(AttachmentOptions::Video(VideoAttachmentOptions {
                thumbnail_time: Some(VideoTime::End),
                ..Default::default()
            })),
        );
        assert!(res.is_ok());
    }

    #[test]
    #[cfg(not(otheros))]
    pub fn test_convert_attachment_movie_time_duration() {
        let res = execute_attachment_test(
            "test.mp4",
            Some(AttachmentOptions::Video(VideoAttachmentOptions {
                thumbnail_time: Some(VideoTime::Time(std::time::Duration::from_secs(60 * 10))),
                ..Default::default()
            })),
        );
        assert!(res.is_ok());
    }

    #[test]
    #[cfg(not(otheros))]
    pub fn test_convert_attachment_movie_cropped() {
        let res = execute_attachment_test(
            "test.mp4",
            Some(AttachmentOptions::Video(VideoAttachmentOptions {
                thumbnail_crop: Some(ThumbnailClippingRect {
                    origin: (0.25, 0.25),
                    size: (0.5, 0.5),
                }),
                ..Default::default()
            })),
        );
        assert!(res.is_ok());
    }

    #[test]
    #[cfg(not(otheros))]
    pub fn test_convert_attachment_movie_hide() {
        let res = execute_attachment_test(
            "test.mp4",
            Some(AttachmentOptions::Video(VideoAttachmentOptions {
                thumbnail_hide: Some(true),
                ..Default::default()
            })),
        );
        assert!(res.is_ok());

        let res = execute_attachment_test(
            "test.mp4",
            Some(AttachmentOptions::Video(VideoAttachmentOptions {
                thumbnail_hide: Some(false),
                ..Default::default()
            })),
        );
        assert!(res.is_ok());
    }

    #[test]
    #[cfg(not(otheros))]
    pub fn test_convert_attachment_movie_type_hint() {
        let formats = vec![
            VideoFormat::MPEG,
            VideoFormat::MPEG4,
            VideoFormat::MPEG2Video,
            VideoFormat::AVIMovie,
        ];
        for f in formats {
            let res = execute_attachment_test(
                "test",
                Some(AttachmentOptions::Video(VideoAttachmentOptions {
                    format: Some(f),
                    ..Default::default()
                })),
            );
            assert!(res.is_ok());
        }
    }

    #[test]
    #[cfg(not(otheros))]
    pub fn test_convert_attachment_audio() {
        let res = execute_attachment_test("test.mp3", None);
        assert!(res.is_ok());
    }

    #[test]
    #[cfg(not(otheros))]
    pub fn test_convert_attachment_audio_type_hint() {
        let formats = vec![
            AudioFormat::MP3,
            AudioFormat::MPEG4Audio,
            AudioFormat::WaveformAudio,
            AudioFormat::AudioInterchangeFileFormat,
        ];
        for f in formats {
            let res = execute_attachment_test(
                "test",
                Some(AttachmentOptions::Audio(AudioAttachmentOptions {
                    format: Some(f),
                })),
            );
            assert!(res.is_ok());
        }
    }

    #[test]
    #[cfg(not(otheros))]
    pub fn test_convert_action() {
        let action = Action {
            identifier: String::from("test"),
            title: String::from("Test"),
            icon: None,
            options: ActionOptions::None,
        };

        let _un_action: Id<UNNotificationAction> = action.into();
    }

    #[test]
    #[cfg(not(otheros))]
    pub fn test_convert_action_with_system_icon() {
        let action = Action {
            identifier: String::from("test"),
            title: String::from("Test"),
            icon: Some(ActionIcon::SystemImageName(String::from("test"))),
            options: ActionOptions::None,
        };

        let _un_action: Id<UNNotificationAction> = action.into();
    }

    #[test]
    #[cfg(not(otheros))]
    pub fn test_convert_action_with_template_icon() {
        let action = Action {
            identifier: String::from("test"),
            title: String::from("Test"),
            icon: Some(ActionIcon::TemplateImageName(String::from("test"))),
            options: ActionOptions::None,
        };

        let _un_action: Id<UNNotificationAction> = action.into();
    }

    #[test]
    #[cfg(not(otheros))]
    pub fn test_convert_action_with_options() {
        let action = Action {
            identifier: String::from("test"),
            title: String::from("Test"),
            icon: None,
            options: ActionOptions::all(),
        };

        let _un_action: Id<UNNotificationAction> = action.into();
    }

    #[test]
    #[cfg(not(otheros))]
    pub fn test_convert_notification() {
        let notification = Notification {
            body: String::from("Test"),
            ..Default::default()
        };

        let un_notification: Result<Id<UNNotificationRequest>, NotificationError> =
            notification.try_into();
        assert!(un_notification.is_ok());
    }

    #[test]
    #[cfg(not(otheros))]
    pub fn test_convert_notification_all() {
        let mut user_info = HashMap::new();
        user_info.insert(String::from("test"), String::from("test"));

        let notification = Notification {
            body: String::from("Test"),
            identifier: String::from("test"),
            trigger: Some(Trigger {
                kind: TriggerKind::TimeInterval(std::time::Duration::from_secs(300)),
                repeats: true,
            }),
            title: Some(String::from("Test")),
            subtitle: Some(String::from("Test")),
            attachments: vec![Attachment {
                identifier: String::from("test1"),
                url: url::Url::parse("file:///test.jpg").unwrap(),
                options: Some(AttachmentOptions::Video(VideoAttachmentOptions {
                    format: Some(VideoFormat::MPEG4),
                    thumbnail_time: Some(VideoTime::Time(std::time::Duration::from_secs(600))),
                    thumbnail_crop: Some(ThumbnailClippingRect { origin: (0.1, 0.2), size: (0.4,0.6) }),
                    thumbnail_hide: Some(false),
                })),
            }],
            user_info,
            thread_identifier: Some(String::from("threadid")),
            category_identifier: Some(String::from("categoryid")),
            summary_argument: Some(String::from("summary_test")),
            summary_argument_count: Some(3),
            launch_image_name: Some(String::from("test_image")),
            badge: Some(3),
            target_content_identifier: Some(String::from("contentid")),
            sound: Some(Sound::DefaultCriticalSound),
            interruption_level: Some(InterruptionLevel::Critical),
            relevance_score: Some(1.3),
            filter_criteria: Some(String::from("testcriteria"))
        };


        let un_notification: Result<Id<UNNotificationRequest>, NotificationError> =
            notification.try_into();
        assert!(un_notification.is_ok());
    }

    #[test]
    #[cfg(not(otheros))]
    pub fn test_convert_category() {
        let category = Category {
            identifier: String::from("test"),
            ..Default::default()
        };

        let _un_category: Id<UNNotificationCategory> = category.into();
    }

    #[test]
    #[cfg(not(otheros))]
    pub fn test_convert_category_all() {
        let category = Category {
            identifier: String::from("test"),
            actions: vec![
                Action {
                    identifier: String::from("testaction"),
                    title: String::from("test"),
                    icon: Some(ActionIcon::SystemImageName(String::from("test"))),
                    options: ActionOptions::all()
                }
            ],
            intent_identifiers: vec![String::from("test")],
            hidden_preview_body_placeholder: Some(String::from("test preview")),
            category_summary_format: Some(String::from("test format")),
            options: CategoryOptions::all(),
        };

        let _un_category: Id<UNNotificationCategory> = category.into();
    }
}
