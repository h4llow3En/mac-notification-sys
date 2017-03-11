//! Custum errors for mac_notification_sys
#![allow(missing_docs)]
use std::error::Error;
use std::fmt;
use std::convert::From;

pub type NotificationResult<T> = Result<T, ErrorKind>;

#[derive(Clone, Copy, Debug)]
pub enum ErrorKind {
    ApplicationError(ApplicationError),
    NotificationError(NotificationError),
}

#[derive(Clone, Copy, Debug)]
pub enum ApplicationError {
    AlreadySet,
    CouldNotSet,
}

impl fmt::Display for ApplicationError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}

impl Error for ApplicationError {
    fn description(&self) -> &str {
        match *self {
            ApplicationError::AlreadySet => "Application can only be set once.",
            ApplicationError::CouldNotSet => {
                "Could not set application, using default \"com.apple.Termial\""
            }
        }
    }
}

impl From<ApplicationError> for ErrorKind {
    fn from(e: ApplicationError) -> Self {
        ErrorKind::ApplicationError(e)
    }
}

#[derive(Clone, Copy, Debug)]
pub enum NotificationError {
    ScheduleInThePast,
    UnableToSchedule,
    UnableToDeliver,
}

impl fmt::Display for NotificationError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}

impl Error for NotificationError {
    fn description(&self) -> &str {
        match *self {
            NotificationError::ScheduleInThePast => "Can not schedule notification in the past",
            NotificationError::UnableToSchedule => "Could not schedule notification",
            NotificationError::UnableToDeliver => "Could not deliver notification",
        }
    }
}

impl From<NotificationError> for ErrorKind {
    fn from(e: NotificationError) -> Self {
        ErrorKind::NotificationError(e)
    }
}
