extern crate gcc;

fn main() {
    if cfg!(target_os = "macos") {
        gcc::Config::new().file("objc/notify.m").flag("-fmodules").compile("libnotify.a");
    }
}
