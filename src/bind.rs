//! Rust bindings to Objective-C frameworks that not included in icrate
#[allow(non_snake_case)]
pub mod CoreGraphics {

    use icrate::Foundation::{CGRect, NSDictionary};

    #[cfg_attr(
        any(
            target_os = "macos",
            target_os = "ios",
            target_os = "tvos",
            target_os = "watchos"
        ),
        link(name = "CoreGraphics", kind = "framework")
    )]
    extern "C" {
        pub fn CGRectCreateDictionaryRepresentation(rect: *const CGRect) -> *const NSDictionary;
    }
}

#[allow(non_snake_case)]
pub mod CoreFoundation {

    use core::ffi::c_void;
    use objc2::Encode;

    #[repr(C)]
    pub struct __CFAllocator(c_void);

    unsafe impl objc2::RefEncode for __CFAllocator {
        const ENCODING_REF: objc2::Encoding = objc2::Encoding::Pointer(&<Self as Encode>::ENCODING);
    }

    unsafe impl Encode for __CFAllocator {
        const ENCODING: objc2::Encoding = objc2::Encoding::Struct(stringify!(__CFAllocator), &[]);
    }

    pub type CFAllocatorRef = __CFAllocator;

    #[cfg_attr(
        any(
            target_os = "macos",
            target_os = "ios",
            target_os = "tvos",
            target_os = "watchos"
        ),
        link(name = "CoreFoundation", kind = "framework")
    )]
    extern "C" {
        pub static kCFAllocatorDefault: *const CFAllocatorRef;

    }
}

#[allow(non_upper_case_globals, non_snake_case)]
pub mod CoreMedia {

    use icrate::Foundation::NSDictionary;
    use objc2::{Encode, Encoding};

    use super::CoreFoundation::CFAllocatorRef;

    #[cfg_attr(
        any(
            target_os = "macos",
            target_os = "ios",
            target_os = "tvos",
            target_os = "watchos"
        ),
        link(name = "CoreMedia", kind = "framework")
    )]
    extern "C" {
        pub static kCMTimePositiveInfinity: CMTime;
        pub static kCMTimeZero: CMTime;

        pub fn CMTimeCopyAsDictionary(
            time: *const CMTime,
            allocator: *const CFAllocatorRef,
        ) -> *const NSDictionary;

        pub fn CMTimeMakeWithSeconds(seconds: f64, timescale: *const CMTimeScale) -> CMTime;
    }

    pub const NSEC_PER_SEC: i64 = 1000000000;

    pub type CMTimeValue = i64;

    pub type CMTimeScale = i32;

    pub type CMTimeFlags = u32;

    pub type CMTimeEpoch = i64;

    #[repr(C)]
    #[derive(Copy, Clone, Debug)]
    pub struct CMTime {
        value: CMTimeValue,
        timescale: CMTimeScale,
        flags: CMTimeFlags,
        epoch: CMTimeEpoch,
    }

    unsafe impl objc2::RefEncode for CMTime {
        const ENCODING_REF: Encoding = Encoding::Pointer(&<Self as Encode>::ENCODING);
    }

    unsafe impl Encode for CMTime {
        const ENCODING: Encoding = Encoding::Struct(
            stringify!(CMTime),
            &[Encoding::Int, Encoding::Int, Encoding::UInt, Encoding::Int],
        );
    }
}
