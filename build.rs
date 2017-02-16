extern crate gcc;

fn main() {
    if cfg!(target_os = "macos") {
        gcc::Config::new().file("objc/libnotify.m").flag("-fmodules").compile("libnotify.a");
    }
}
