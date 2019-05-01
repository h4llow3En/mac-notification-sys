//! Custom errors for mac-notification-sys.
use failure;
/// Custom Result type for mac-notification-sys.
pub type NotificationResult<T> = Result<T, failure::Error>;

/// Errors that can occur setting the Bundle Identifier.
#[derive(Debug, Fail)]
pub enum ApplicationError {

    /// The application name is already set.
    #[fail(display = "Application '{}' can only be set once.", _0)]
    AlreadySet(String),

    /// The application name could not be set.
    #[fail(display = "Could not set application '{}', using default \"com.apple.Termial\"", _0)]
    CouldNotSet(String),
}

/// Errors that can occur while interacting with the NSUserNotificationCenter.
#[derive(Debug, Fail)]
pub enum NotificationError {
    /// Notifications can not be scheduled in the past.
    #[fail(display = "Can not schedule notification in the past")]
    ScheduleInThePast,

    /// Schedule a notification caused an error.
    #[fail(display = "Could not schedule notification")]
    UnableToSchedule,

    /// Deliver a notification caused an error.
    #[fail(display = "Could not deliver notification")]
    UnableToDeliver,
}
