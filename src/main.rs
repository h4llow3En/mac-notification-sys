#[macro_use]
extern crate objc_foundation;
use std::ops::Deref;
use objc_foundation::{NSString,INSString};

#[allow(improper_ctypes)]
#[link(name = "notify")]
extern {
    fn sendNotification(title: *const NSString, message: *const NSString, sound: *const NSString);
    fn setApplication(newbundleIdentifier: *const NSString) -> bool;
    fn getBundleIdentifier(appName: *const NSString) -> *const NSString;
}

fn main() {
    unsafe {
        let application = getBundleIdentifier(NSString::from_str("Safari").deref());
        let _ = setApplication(application);
        sendNotification(NSString::from_str("Hello, world!").deref(), NSString::from_str("Message").deref(), NSString::from_str("Ping").deref());
    }
}
