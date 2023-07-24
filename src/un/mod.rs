//! Wrapper functions to send notifications using the User Notifications framework

use std::sync::{Mutex, Arc};

use futures::StreamExt;
use icrate::{UserNotifications::UNUserNotificationCenter, block2::{ConcreteBlock, Block}, Foundation::{NSInteger, NSError}};
use objc2::{runtime::Bool, rc::Id, ClassType};

use crate::{os::{APPLE_VERSION, AppleOS}, error::NotificationError};

use self::notification::AuthorizationOptions;

mod bind;
pub mod builder;
mod delegate;
pub mod notification;

pub use chrono::{DateTime, Local, Offset, Utc};
pub use cron::Schedule;
pub use url::Url;

/// Requesting the authorization to send notifications
///
/// Returning an error if not allowed or any other error occured. Ok after accept one or many
/// options.
pub async fn request_authorization(options: AuthorizationOptions) -> Result<(), NotificationError> {
    let av = *APPLE_VERSION;
    if av >= (AppleOS::IOS, 10, 0)
        || av >= (AppleOS::MacOS, 10, 14)
        || av >= (AppleOS::MacCatalyst, 13, 1)
        || av >= (AppleOS::TvOS, 10, 0)
        || av >= (AppleOS::VisionOS, 1, 0)
        || av >= (AppleOS::WatchOS, 3, 0)
    {
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
    } else {
        Ok(())
    }
}

/// Updating the badge count of the app's icon
pub async fn set_badge_count(count: usize) -> Result<(), NotificationError> {
    let av = *APPLE_VERSION;
    #[cfg(not(watchos))]
    if av >= (AppleOS::IOS, 16, 0)
        || av >= (AppleOS::MacOS, 13, 0)
        || av >= (AppleOS::MacCatalyst, 16, 0)
        || av >= (AppleOS::TvOS, 16, 0)
        || av >= (AppleOS::VisionOS, 1, 0)
    {
        let current_notification_center =
            unsafe { UNUserNotificationCenter::currentNotificationCenter() };

        // TODO:- Replace this with oneshot after block2 supported FnOnce
        // @see https://github.com/madsmtm/objc2/issues/168
        let (sender, mut receiver) = futures::channel::mpsc::channel::<Result<(), Id<NSError>>>(1);
        let arc_sender = Arc::new(Mutex::new(sender));

        let completion_handler = ConcreteBlock::new(move |err: *mut NSError| {
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
        let completion_handler = completion_handler.copy();
        let completion_handler: &Block<_, ()> = &completion_handler;

        let ns_int = NSInteger::from(count as isize);

        unsafe {
            current_notification_center
                .setBadgeCount_withCompletionHandler(ns_int, Some(completion_handler));
        }

        let received = receiver.next().await.unwrap();
        receiver.close();

        received.map_err(NotificationError::from)
    } else {
        Ok(())
    }

    #[cfg(watchos)]
    Ok(())
}
