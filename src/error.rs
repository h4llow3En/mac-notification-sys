//! Errors returning from the library
use core::fmt::Debug;

use icrate::Foundation::NSError;
use objc2::rc::Id;

/// All the errors that returning from this library
#[derive(Clone)]
pub enum NotificationError {
    /// Error from the Objective C User Notifications framework
    NSError(Id<NSError>),
    /// Not supported for the current OS version
    NotSupported,
}

impl From<Id<NSError>> for NotificationError {
    fn from(value: Id<NSError>) -> Self {
        Self::NSError(value)
    }
}

impl Debug for NotificationError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            NotificationError::NSError(ns_error) => {
                f.debug_struct("NSError")
                    .field("code", &ns_error.code())
                    .field("domain", &ns_error.domain().to_string())
                    .field("message", &ns_error.localizedDescription().to_string())
                    .finish()
            },
            NotificationError::NotSupported => {
                f.write_str("NotSupported")
            }
        }
    }
}
