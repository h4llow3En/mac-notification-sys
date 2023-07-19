//! Errors returning from the library
use std::fmt::Debug;

use icrate::Foundation::NSError;
use objc2::rc::Id;

/// All the errors that returning from this library
#[derive(Clone)]
pub enum NotificationError {
    /// Error from the Objective C User Notifications framework
    NSError(Id<NSError>),
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
            }
        }
    }
}
