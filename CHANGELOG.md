# Changelog

### [v0.5.7](https://github.com/hoodie/mac-notification-sys/compare/v0.5.6...v0.5.7) (2023-07-23)

#### Fixes

* ensure cross-build compatibility of build.rs
([0c1dbdf](https://github.com/hoodie/mac-notification-sys/commit/0c1dbdff92c34eb9189bafaf8683b4da0e63d9fd))

### [v0.5.6](https://github.com/hoodie/mac-notification-sys/compare/v0.5.5...v0.5.6) (2022-08-06)

#### Fixes

* Always pass a macOS deployment version to cc
([41cc097](https://github.com/hoodie/mac-notification-sys/commit/41cc09753b6802c7061ca8c2c7f226f131158dbe))

### [v0.5.5](https://github.com/hoodie/mac-notification-sys/compare/v0.5.4...v0.5.5) (2022-07-12)

#### Fixes

* reverting build.rs to 0.5.2
([5a48022](https://github.com/hoodie/mac-notification-sys/commit/5a480222e459f2d22cf18eb8f5d2b35ed37b105f))

### [v0.5.4](https://github.com/hoodie/mac-notification-sys/compare/v0.5.3...v0.5.4) (2022-07-12)

#### Fixes

* reverting objc code to 0.5.2
([32ef45a](https://github.com/hoodie/mac-notification-sys/commit/32ef45a555a6f716073a3a74080168686e6baac0)),
closes [#43](https://github.com/hoodie/mac-notification-sys/issues/43)

### [v0.5.3](https://github.com/hoodie/mac-notification-sys/compare/v0.5.2...v0.5.3) (2022-07-10)

#### Fixes

* use NSRunningApplication instead of AppleScript
([9125aa0](https://github.com/hoodie/mac-notification-sys/commit/9125aa0144457074efa6fc8872ab9c6e4a592021))
* convert to ARC memory management
([afb0ad7](https://github.com/hoodie/mac-notification-sys/commit/afb0ad77875475bd7d26411bdafd6cf230eff930))

### [v0.5.2](https://github.com/hoodie/mac-notification-sys/compare/v0.5.1...v0.5.2) (2022-06-12)

#### Fixes

* copy paste mishap in readme
([95434c1](https://github.com/hoodie/mac-notification-sys/commit/95434c18a9bbad3d4a5b14888383332d0e77d587))

### [v0.5.1](https://github.com/hoodie/mac-notification-sys/compare/v0.5.0...v0.5.1) (2022-06-11)

#### Fixes

* assert on errors instead of panic in tests
([5369717](https://github.com/hoodie/mac-notification-sys/commit/536971757ef395dd39613f3966569d96eb4eac06))
* move installNSBundleHook to setApplication
([6bcce5f](https://github.com/hoodie/mac-notification-sys/commit/6bcce5f194425667997bfc255cbf54f0ecfda89c))
* Wrap setApplication in auto release pool
([ead722b](https://github.com/hoodie/mac-notification-sys/commit/ead722bad1e1c76846f84d08594a7764c6f48f2d))
* retain bundle id NSString
([956124e](https://github.com/hoodie/mac-notification-sys/commit/956124e79a3ecb2605272ab7d42f75fc5a70860d)),
closes [#8](https://github.com/hoodie/mac-notification-sys/issues/8)

## [v0.5.0](https://github.com/hoodie/mac-notification-sys/compare/v0.4.0...v0.5.0) (2022-03-20)

### Features

* provide builder pattern
([757bc25](https://github.com/hoodie/mac-notification-sys/commit/757bc256ce139eed8b90691296182562082e522e))

## [v0.4.0](https://github.com/hoodie/mac-notification-sys/compare/v0.3.0...v0.4.0) (2022-03-13)

## [v0.3.0](https://github.com/hoodie/mac-notification-sys/compare/v0.1.3...v0.3.0) (2019-05-04)

### [v0.1.3](https://github.com/hoodie/mac-notification-sys/compare/v0.1.2...v0.1.3) (2017-06-21)

### v0.1.2 (2017-04-24)
