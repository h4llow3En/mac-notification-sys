extern crate macos_notifications;
extern crate chrono;
use chrono::prelude::*;
use macos_notifications::*;

fn main() {
    let stamp = UTC::now().timestamp() as f64 + 5.;
    println!("{:?}", stamp);
    let bundle = util::get_bundle_identifier("firefox");
    util::set_application(&bundle);
    let _sent = schedule_notification("Danger", "Will Robinson", "Run away as fast as you can", "Blow", stamp+5.);

}
