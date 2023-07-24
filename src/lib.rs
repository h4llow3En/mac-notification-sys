//! A very thin wrapper around UNNotification
//!
//! Only supported for `macOS 10.14+`
#![warn(
    missing_docs,
    trivial_casts,
    trivial_numeric_casts,
    unused_import_braces,
    unused_qualifications
)]
#![cfg(target_os = "macos")]
#![allow(improper_ctypes)]

use icrate::Foundation::{NSDate, NSDefaultRunLoopMode, NSRunLoop};

#[cfg(feature="un")]
pub mod un;
pub mod error;
mod os;

/// Run the RunLoop once
pub fn run_ns_run_loop_once() {
    unsafe {
        let main_loop = NSRunLoop::mainRunLoop();
        let limit_date = NSDate::dateWithTimeIntervalSinceNow(0.1);
        main_loop.acceptInputForMode_beforeDate(NSDefaultRunLoopMode, &limit_date);
    }
}
