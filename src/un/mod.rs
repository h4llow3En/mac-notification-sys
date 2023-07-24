//! Wrapper functions to send notifications using the User Notifications framework

use std::{
    ptr::NonNull,
    sync::{Arc, Mutex},
};

use futures::{channel::mpsc::Receiver, StreamExt};
use icrate::{
    block2::{Block, ConcreteBlock, RcBlock},
    Foundation::{NSError, NSInteger, NSSet},
    UserNotifications::{UNNotificationCategory, UNNotificationRequest, UNUserNotificationCenter},
};
use objc2::{rc::Id, runtime::Bool, ClassType};

use crate::{
    error::NotificationError,
    os::{AppleOS, APPLE_VERSION},
};

use self::notification::{AuthorizationOptions, Category, Notification};

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

        let ns_int = NSInteger::from(count as isize);

        let (completion_handler, mut receiver) = result_completion_handler();

        unsafe {
            current_notification_center
                .setBadgeCount_withCompletionHandler(ns_int, Some(&completion_handler));
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

/// Setting the notification categories
///
/// Use this function at the first launch. use the `add_category` method when
/// you need to add additional categories
pub fn set_categories(categories: Vec<Category>) {
    #[cfg(not(tvos))]
    {
        let current_notification_center =
            unsafe { UNUserNotificationCenter::currentNotificationCenter() };

        let un_cats: Vec<Id<UNNotificationCategory>> =
            categories.into_iter().map(|c| c.into()).collect();
        let un_set = NSSet::from_vec(un_cats);
        unsafe {
            current_notification_center.setNotificationCategories(&un_set);
        }
    }
}

/// Adding a single category after app initialized
pub async fn add_category(category: Category) {
    #[cfg(not(tvos))]
    {
        let current_notification_center =
            unsafe { UNUserNotificationCenter::currentNotificationCenter() };

        // TODO:- Replace this with oneshot after block2 supported FnOnce
        // @see https://github.com/madsmtm/objc2/issues/168
        let (sender, mut receiver) = futures::channel::mpsc::channel::<()>(1);
        let arc_sender = Arc::new(Mutex::new(sender));

        let completion_handler = ConcreteBlock::new(
            move |non_null_categories: NonNull<NSSet<UNNotificationCategory>>| {
                let mut sender_locked = arc_sender.lock().unwrap();
                let un_category: Id<UNNotificationCategory> = category.clone().into();
                unsafe {
                    let current_notification_center =
                        UNUserNotificationCenter::currentNotificationCenter();
                    let cat_set = non_null_categories.as_ref();
                    cat_set.setByAddingObject(&un_category);
                    current_notification_center.setNotificationCategories(cat_set);
                }
                sender_locked.close_channel();
            },
        );
        let completion_handler = completion_handler.copy();
        let completion_handler: &Block<_, ()> = &completion_handler;

        unsafe {
            current_notification_center
                .getNotificationCategoriesWithCompletionHandler(completion_handler);
        }

        receiver.next().await.unwrap();
        receiver.close();
    }
}

/// Adding a notification to deliver
///
/// Future will complete once the notification validated and inserted
/// in to the notification stack. This will return an error if the notification
/// not validated.
pub async fn add_notification(notification: Notification) -> Result<(), NotificationError> {
    let av = *APPLE_VERSION;

    if av >= (AppleOS::IOS, 10, 0)
        || av >= (AppleOS::MacOS, 10, 14)
        || av >= (AppleOS::MacCatalyst, 13, 1)
        || av >= (AppleOS::TvOS, 10, 0)
        || av >= (AppleOS::WatchOS, 3, 0)
    {
        let un_notification: Id<UNNotificationRequest> = notification.try_into()?;
        let (completion_handler, mut receiver) = result_completion_handler();

        unsafe {
            let current_notification_center = UNUserNotificationCenter::currentNotificationCenter();

            current_notification_center.addNotificationRequest_withCompletionHandler(
                &un_notification,
                Some(&completion_handler),
            );
        }

        let received = receiver.next().await.unwrap();
        receiver.close();

        received.map_err(NotificationError::from)
    } else {
        Ok(())
    }
}

/// Creating a completion handler which accepts only NSError as a argument. And returning the
/// Receiver which consume the status
fn result_completion_handler() -> (
    RcBlock<(*mut NSError,), ()>,
    Receiver<Result<(), Id<NSError>>>,
) {
    // TODO:- Replace this with oneshot after block2 supported FnOnce
    // @see https://github.com/madsmtm/objc2/issues/168

    let (sender, receiver) = futures::channel::mpsc::channel::<Result<(), Id<NSError>>>(1);
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
    (completion_handler, receiver)
}
