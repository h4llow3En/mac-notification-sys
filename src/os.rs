//! Helper to detect os and check API availability
use core::cmp::Ordering;

use lazy_static::lazy_static;
use icrate::Foundation::NSProcessInfo;

lazy_static! {
    pub static ref APPLE_VERSION: AppleVersion = {
        #[cfg(any(watchos, ios, macos, catalyst))]
        {
            #[cfg(watchos)]
            let os: AppleOS = AppleOS::WatchOS;

            #[cfg(macos)]
            let os: AppleOS = AppleOS::MacOS;

            #[cfg(catalyst)]
            let os: AppleOS = AppleOS::MacCatalyst;

            #[cfg(ios)]
            let os: AppleOS = AppleOS::IOS;

            let p_info = NSProcessInfo::processInfo();
            let os_version = p_info.operatingSystemVersion();
            AppleVersion(os, os_version.majorVersion as u16, os_version.minorVersion as u16)
        }
        #[cfg(not(any(watchos, ios, macos, catalyst)))]
        {
            AppleVersion(AppleOS::None, 0, 0)
        }
    };
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub enum AppleOS {
    MacOS,
    MacCatalyst,
    TvOS,
    WatchOS,
    VisionOS,
    IOS,
    #[cfg(otheros)]
    None,
}

#[derive(Copy, Clone, Debug)]
pub struct AppleVersion(AppleOS, u16, u16);

impl PartialEq<AppleOS> for AppleVersion {
    fn eq(&self, other: &AppleOS) -> bool {
        &self.0 == other
    }
}

impl PartialEq<(AppleOS, u16, u16)> for AppleVersion {
    fn eq(&self, other: &(AppleOS, u16, u16)) -> bool {
        self.0 == other.0 && self.1 == other.1 && self.2 == other.2
    }
}

impl PartialOrd<(AppleOS, u16, u16)> for AppleVersion {
    fn partial_cmp(&self, other: &(AppleOS, u16, u16)) -> Option<Ordering> {
        if self.0 != other.0 {
            return None;
        }

        if self.1 == other.1 && self.2 == other.2 {
            Some(Ordering::Equal)
        } else if self.1 > other.1 || (self.1 == other.1 && self.2 > other.2) {
            Some(Ordering::Greater)
        } else if self.1 < other.1 || (self.1 == other.1 && self.2 < other.2) {
            Some(Ordering::Less)
        } else {
            None
        }
    }
}

#[cfg(test)]
mod test {
    use super::{AppleOS, AppleVersion};

    #[test]
    pub fn test_os_version_and_os_partial_eq() {
        let mac_version = AppleVersion(AppleOS::MacOS, 13, 2);
        assert!(mac_version == AppleOS::MacOS);
        assert!(mac_version != AppleOS::IOS);
    }

    #[test]
    pub fn test_same_os_partial_eq() {
        let mac_version = AppleVersion(AppleOS::MacOS, 13, 2);
        assert!(mac_version == (AppleOS::MacOS, 13, 2));
        assert!(mac_version != (AppleOS::MacOS, 13, 1));
        assert!(mac_version != (AppleOS::MacOS, 13, 3));
        assert!(mac_version != (AppleOS::MacOS, 12, 2));
        assert!(mac_version != (AppleOS::MacOS, 14, 2));
        assert!(mac_version != (AppleOS::MacOS, 14, 1));
        assert!(mac_version != (AppleOS::MacOS, 14, 3));
    }

    #[test]
    pub fn test_same_os_partial_ord() {
        let mac_version = AppleVersion(AppleOS::MacOS, 13, 2);
        assert!(mac_version >= (AppleOS::MacOS, 13, 2));
        assert!(mac_version <= (AppleOS::MacOS, 13, 2));
        assert!(mac_version > (AppleOS::MacOS, 13, 1));
        assert!(mac_version < (AppleOS::MacOS, 13, 3));
        assert!(mac_version > (AppleOS::MacOS, 12, 2));
        assert!(mac_version < (AppleOS::MacOS, 14, 2));
        assert!(mac_version <= (AppleOS::MacOS, 14, 2));
        assert!(mac_version >= (AppleOS::MacOS, 12, 2));
        assert!(mac_version >= (AppleOS::MacOS, 13, 1));
        assert!(mac_version <= (AppleOS::MacOS, 13, 4));
    }

    #[test]
    pub fn test_different_os_partial_eq() {
        let mac_version = AppleVersion(AppleOS::IOS, 13, 2);
        assert!(mac_version != (AppleOS::MacOS, 13, 2));
        assert!(mac_version != (AppleOS::MacOS, 13, 1));
        assert!(mac_version != (AppleOS::MacOS, 13, 3));
        assert!(mac_version != (AppleOS::MacOS, 12, 2));
        assert!(mac_version != (AppleOS::MacOS, 14, 2));
        assert!(mac_version != (AppleOS::MacOS, 14, 1));
        assert!(mac_version != (AppleOS::MacOS, 14, 3));
    }

    #[test]
    pub fn test_different_os_partial_ord() {
        let mac_version = AppleVersion(AppleOS::IOS, 13, 2);
        assert!(!(mac_version >= (AppleOS::MacOS, 13, 2)));
        assert!(!(mac_version <= (AppleOS::MacOS, 13, 2)));
        assert!(!(mac_version > (AppleOS::MacOS, 13, 1)));
        assert!(!(mac_version < (AppleOS::MacOS, 13, 3)));
        assert!(!(mac_version > (AppleOS::MacOS, 12, 2)));
        assert!(!(mac_version < (AppleOS::MacOS, 14, 2)));
        assert!(!(mac_version <= (AppleOS::MacOS, 14, 2)));
        assert!(!(mac_version >= (AppleOS::MacOS, 12, 2)));
        assert!(!(mac_version >= (AppleOS::MacOS, 13, 1)));
        assert!(!(mac_version <= (AppleOS::MacOS, 13, 4)));
    }
}
