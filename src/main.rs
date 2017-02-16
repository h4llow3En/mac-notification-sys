#[macro_use]
extern crate objc_foundation;
use std::ops::Deref;
use objc_foundation::{NSString,INSString};

#[allow(improper_ctypes)]
#[link(name = "notify")]
extern {
    fn sendNotification(title: *const NSString, message: *const NSString, sound: *const NSString);
    fn setApplication(newbundleIdentifier: *const NSString);
}

fn main() {
    unsafe {
        setApplication(NSString::from_str("com.apple.safari").deref());
        sendNotification(NSString::from_str("Hello, world!").deref(), NSString::from_str("Message").deref(), NSString::from_str("Ping").deref());
    }
}
