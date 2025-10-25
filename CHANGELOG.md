# Changelog

### [v0.6.7](///compare/v0.6.6...v0.6.7) (2025-10-25)

#### Fixes

* amend licensing (dc77812)

### [v0.6.6](///compare/v0.6.5...v0.6.6) (2025-07-09)

#### Fixes

* correct wait for click behavior (d933ff6)

### [v0.6.5](///compare/v0.6.4...v0.6.5) (2025-07-03)

#### Features

* add support for waiting on notification click (b9a2703)

### [v0.6.4](///compare/v0.6.3...v0.6.4) (2025-04-06)

#### Features

* update method signatures in header file (09b53af)

### [v0.6.3](///compare/v0.6.2...v0.6.3) (2025-04-06)

#### Fixes

* do not return uninitialized actiondata as notification response (8ad8fed),
closes #64

### [v0.6.2](///compare/v0.6.1...v0.6.2) (2024-09-11)

#### Fixes

* use correct fallback app name (39d88c9)

### [v0.6.1](///compare/v0.6.0...v0.6.1) (2023-08-12)

#### Fixes

* formatting and readme update (4b199d1)

## [v0.6.0](///compare/v0.5.9...v0.6.0) (2023-08-12)

### ⚠ BREAKING CHANGE

* notifications do not show an unused "show" action button but default


### Features

* default to no action button (6bf2342)

### [v0.5.9](///compare/v0.5.8...v0.5.9) (2023-08-11)

#### Fixes

* do not crash when selecting the title of a drop down menu (ba226cd)

### [v0.5.8](///compare/v0.5.7...v0.5.8) (2023-07-23)

#### Features

* allow playing default notification sound (c0ffeec)

### [v0.5.7](///compare/v0.5.6...v0.5.7) (2023-07-23)

#### Fixes

* ensure cross-build compatibility of build.rs (0c1dbdf)

### [v0.5.6](///compare/v0.5.5...v0.5.6) (2022-08-06)

#### Fixes

* Always pass a macOS deployment version to cc (41cc097)

### [v0.5.5](///compare/v0.5.4...v0.5.5) (2022-07-13)

#### Fixes

* reverting build.rs to 0.5.2 (5a48022)

### [v0.5.4](///compare/v0.5.3...v0.5.4) (2022-07-13)

#### Fixes

* reverting objc code to 0.5.2 (32ef45a), closes #43

### [v0.5.3](///compare/v0.5.2...v0.5.3) (2022-07-10)

#### Fixes

* use NSRunningApplication instead of AppleScript (9125aa0)
* convert to ARC memory management (afb0ad7)

### [v0.5.2](///compare/v0.5.1...v0.5.2) (2022-06-12)

#### Fixes

* copy paste mishap in readme (95434c1)

### [v0.5.1](///compare/v0.5.0...v0.5.1) (2022-06-11)

#### Fixes

* assert on errors instead of panic in tests (5369717)
* move installNSBundleHook to setApplication (6bcce5f)
* Wrap setApplication in auto release pool (ead722b)
* retain bundle id NSString (956124e), closes #8

## [v0.5.0](///compare/v0.4.0...v0.5.0) (2022-03-20)

### Features

* provide builder pattern (757bc25)

## [v0.4.0](///compare/v0.3.0...v0.4.0) (2022-03-13)

## [v0.3.0](///compare/v0.1.3...v0.3.0) (2019-05-04)

### [v0.1.3](///compare/v0.1.2...v0.1.3) (2017-06-21)

### v0.1.2 (2017-04-24)
