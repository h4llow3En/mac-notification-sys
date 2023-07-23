use cfg_aliases::cfg_aliases;

fn main() {
    cfg_aliases! {
        watchos: { target_os = "watchos" },
        macos: { target_os = "macos" },
        ios: { all(not(any(target="aarch64-apple-ios-macabi", target="x86_64-apple-ios-macabi")), target_os = "ios") },
        catalyst: { any(target="aarch64-apple-ios-macabi", target="x86_64-apple-ios-macabi") },
        otheros: {not(any(target_os ="watchos", target_os ="macos", target_os = "ios"))}
    }
}
