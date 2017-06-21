//! Custom errors for mac-notification-sys.

/// Custom Result type for mac-notification-sys.
pub type NotificationResult<T> = Result<T>;

/// Errors that can occur setting the Bundle Identifier.
pub mod applications_error {
    error_chain!{
        errors {
            /// The application name is already set.
            AlreadySet{
                description("Application can only be set once.")
            }
            /// The application name could not be set.
            CouldNotSet{
                description("Could not set application, using default \"com.apple.Termial\"")
            }
        }
    }
}

/// Errors that can occur while interacting with the NSUserNotificationCenter.
pub mod notification_error {
    error_chain!{
        errors {
            /// Notifications can not be scheduled in the past.
            ScheduleInThePast {
                description("Can not schedule notification in the past")
            }

            /// Schedule a notification caused an error.
            UnableToSchedule {
                description("Could not schedule notification")
            }

            /// Deliver a notification caused an error.
            UnableToDeliver {
                description("Could not deliver notification")
            }
        }
    }
}

error_chain! {
    links {
        ApplicationError(applications_error::Error, applications_error::ErrorKind) #[doc = "An error setting the application occured."];
        NotificationError(notification_error::Error, notification_error::ErrorKind) #[doc = "An error scheduling the notification occured."];
    }
}
