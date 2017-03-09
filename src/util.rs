use super::sys;
use error::*;
use std::ops::Deref;
use objc_foundation::{NSString, INSString};
use std::path::PathBuf;
use std::env;

static mut APPLICATION_SET: bool = false;

pub fn get_bundle_identifier(app_name: &str) -> String {
    get_bundle_identifier_or(app_name).unwrap_or("com.apple.Terminal".to_string())
}

pub fn get_bundle_identifier_or(app_name: &str) -> Option<String> {
    unsafe {
        sys::getBundleIdentifier(NSString::from_str(app_name).deref()) // *const NSString
            .as_ref() // Option<NSString>
            .map(|nstr| nstr.as_str().to_owned())
    }
}

/// ACHTUNG
pub fn set_application(bundle_ident: &str) -> NotificationResult<()> {
    unsafe {
        if APPLICATION_SET {
            Err(ApplicationError::AlreadySet.into())
        } else {
            if sys::setApplication(NSString::from_str(bundle_ident).deref()) {
                Ok(())
            } else {
                Err(ApplicationError::CouldNotSet.into())
            }
        }
    }
}

pub fn check_sound(sound_name: &str) -> bool {
    let mut file_exists: bool = false;
    let new_path: String;
    let mut sound_paths: Vec<&str> =
        vec!["/Library/Sounds/", "/Network/Library/Sounds/", "/System/Library/Sounds/"];
    match env::home_dir() {
        Some(path) => {
            new_path = path.to_str().unwrap().to_string() + "/Library/Sounds/";
            sound_paths.insert(0, &new_path);
        }
        None => print!("No home path found.", ),
    }
    for check_path in sound_paths {
        let mut test_path = PathBuf::from(check_path);
        test_path.push(sound_name);
        test_path.push(".aiff");
        if test_path.as_path().exists() {
            file_exists = true;
        }
    }
    file_exists
}
