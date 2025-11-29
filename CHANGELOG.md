# Changelog

### [v0.6.9](https://github.com/h4llow3En/mac-notification-sys//compare/v0.6.8...v0.6.9) (2025-11-29)

#### Fixes

* link AppKit
([fe5d56c](https://github.com/h4llow3En/mac-notification-sys//commit/fe5d56c1a57cceeea54a98424dda60197728007c))

### [v0.6.8](https://github.com/h4llow3En/mac-notification-sys//compare/v0.6.7...v0.6.8) (2025-10-25)

#### Fixes

* build without `-fmodules` flag
([1044a45](https://github.com/h4llow3En/mac-notification-sys//commit/1044a45edca712d7af133ea49a7623c8750c319d))

### [v0.6.7](https://github.com/h4llow3En/mac-notification-sys//compare/v0.6.6...v0.6.7) (2025-10-25)

#### Fixes

* amend licensing
([dc77812](https://github.com/h4llow3En/mac-notification-sys//commit/dc778120c081cd7e1e175dbf1cdc04a3c7389c3e))

### [v0.6.6](https://github.com/h4llow3En/mac-notification-sys//compare/v0.6.5...v0.6.6) (2025-07-09)

#### Fixes

* correct wait for click behavior
([d933ff6](https://github.com/h4llow3En/mac-notification-sys//commit/d933ff6a7ed4fb81f387b0b9ce83a1513e65c0aa))

### [v0.6.5](https://github.com/h4llow3En/mac-notification-sys//compare/v0.6.4...v0.6.5) (2025-07-03)

#### Features

* add support for waiting on notification click
([b9a2703](https://github.com/h4llow3En/mac-notification-sys//commit/b9a2703c5c8605d40ccc9c18b81e1412fbc6d347))

### [v0.6.4](https://github.com/h4llow3En/mac-notification-sys//compare/v0.6.3...v0.6.4) (2025-04-06)

#### Features

* update method signatures in header file
([09b53af](https://github.com/h4llow3En/mac-notification-sys//commit/09b53aff7f851910fbe090a404a667d47d3a4e2f))

### [v0.6.3](https://github.com/h4llow3En/mac-notification-sys//compare/v0.6.2...v0.6.3) (2025-04-06)

#### Fixes

* do not return uninitialized actiondata as notification response
([8ad8fed](https://github.com/h4llow3En/mac-notification-sys//commit/8ad8fedc0c56b7624aa01d38c9e763e970178681)),
closes [#64](https://github.com/h4llow3En/mac-notification-sys//issues/64)

### [v0.6.2](https://github.com/h4llow3En/mac-notification-sys//compare/v0.6.1...v0.6.2) (2024-09-11)

#### Fixes

* use correct fallback app name
([39d88c9](https://github.com/h4llow3En/mac-notification-sys//commit/39d88c9a03d0b84df8d49b4a8e0d9693fb897fdb))

### [v0.6.1](https://github.com/h4llow3En/mac-notification-sys//compare/v0.6.0...v0.6.1) (2023-08-12)

#### Fixes

* formatting and readme update
([4b199d1](https://github.com/h4llow3En/mac-notification-sys//commit/4b199d1042932432f53e60527b02942c9ed83ba6))

## [v0.6.0](https://github.com/h4llow3En/mac-notification-sys//compare/v0.5.9...v0.6.0) (2023-08-12)

### ⚠ BREAKING CHANGE

* notifications do not show an unused "show" action button but default


### Features

* default to no action button
([6bf2342](https://github.com/h4llow3En/mac-notification-sys//commit/6bf2342731800c26fc9cb2dc5c116975c18af2fc))

### [v0.5.9](https://github.com/h4llow3En/mac-notification-sys//compare/v0.5.8...v0.5.9) (2023-08-11)

#### Fixes

* do not crash when selecting the title of a drop down menu
([ba226cd](https://github.com/h4llow3En/mac-notification-sys//commit/ba226cd4ab0fcdfb756f3df992089bc3ecb7239d))

### [v0.5.8](https://github.com/h4llow3En/mac-notification-sys//compare/v0.5.7...v0.5.8) (2023-07-23)

#### Features

* allow playing default notification sound
([c0ffeec](https://github.com/h4llow3En/mac-notification-sys//commit/c0ffeec1957efb91847e338abf6890e4ba5daa1e))

### [v0.5.7](https://github.com/h4llow3En/mac-notification-sys//compare/v0.5.6...v0.5.7) (2023-07-23)

#### Fixes

* ensure cross-build compatibility of build.rs
([0c1dbdf](https://github.com/h4llow3En/mac-notification-sys//commit/0c1dbdff92c34eb9189bafaf8683b4da0e63d9fd))

### [v0.5.6](https://github.com/h4llow3En/mac-notification-sys//compare/v0.5.5...v0.5.6) (2022-08-06)

#### Fixes

* Always pass a macOS deployment version to cc
([41cc097](https://github.com/h4llow3En/mac-notification-sys//commit/41cc09753b6802c7061ca8c2c7f226f131158dbe))

### [v0.5.5](https://github.com/h4llow3En/mac-notification-sys//compare/v0.5.4...v0.5.5) (2022-07-13)

#### Fixes

* reverting build.rs to 0.5.2
([5a48022](https://github.com/h4llow3En/mac-notification-sys//commit/5a480222e459f2d22cf18eb8f5d2b35ed37b105f))

### [v0.5.4](https://github.com/h4llow3En/mac-notification-sys//compare/v0.5.3...v0.5.4) (2022-07-13)

#### Fixes

* reverting objc code to 0.5.2
([32ef45a](https://github.com/h4llow3En/mac-notification-sys//commit/32ef45a555a6f716073a3a74080168686e6baac0)),
closes [#43](https://github.com/h4llow3En/mac-notification-sys//issues/43)

### [v0.5.3](https://github.com/h4llow3En/mac-notification-sys//compare/v0.5.2...v0.5.3) (2022-07-10)

#### Fixes

* use NSRunningApplication instead of AppleScript
([9125aa0](https://github.com/h4llow3En/mac-notification-sys//commit/9125aa0144457074efa6fc8872ab9c6e4a592021))
* convert to ARC memory management
([afb0ad7](https://github.com/h4llow3En/mac-notification-sys//commit/afb0ad77875475bd7d26411bdafd6cf230eff930))

### [v0.5.2](https://github.com/h4llow3En/mac-notification-sys//compare/v0.5.1...v0.5.2) (2022-06-12)

#### Fixes

* copy paste mishap in readme
([95434c1](https://github.com/h4llow3En/mac-notification-sys//commit/95434c18a9bbad3d4a5b14888383332d0e77d587))

### [v0.5.1](https://github.com/h4llow3En/mac-notification-sys//compare/v0.5.0...v0.5.1) (2022-06-11)

#### Fixes

* assert on errors instead of panic in tests
([5369717](https://github.com/h4llow3En/mac-notification-sys//commit/536971757ef395dd39613f3966569d96eb4eac06))
* move installNSBundleHook to setApplication
([6bcce5f](https://github.com/h4llow3En/mac-notification-sys//commit/6bcce5f194425667997bfc255cbf54f0ecfda89c))
* Wrap setApplication in auto release pool
([ead722b](https://github.com/h4llow3En/mac-notification-sys//commit/ead722bad1e1c76846f84d08594a7764c6f48f2d))
* retain bundle id NSString
([956124e](https://github.com/h4llow3En/mac-notification-sys//commit/956124e79a3ecb2605272ab7d42f75fc5a70860d)),
closes [#8](https://github.com/h4llow3En/mac-notification-sys//issues/8)

## [v0.5.0](https://github.com/h4llow3En/mac-notification-sys//compare/v0.4.0...v0.5.0) (2022-03-20)

### Features

* provide builder pattern
([757bc25](https://github.com/h4llow3En/mac-notification-sys//commit/757bc256ce139eed8b90691296182562082e522e))

## [v0.4.0](https://github.com/h4llow3En/mac-notification-sys//compare/v0.3.0...v0.4.0) (2022-03-13)

## [v0.3.0](https://github.com/h4llow3En/mac-notification-sys//compare/v0.1.3...v0.3.0) (2019-05-04)

### [v0.1.3](https://github.com/h4llow3En/mac-notification-sys//compare/v0.1.2...v0.1.3) (2017-06-21)

### v0.1.2 (2017-04-24)
