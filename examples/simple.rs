use std::time::Duration;

use async_io::block_on;
use mac_notification_sys::*;

fn main() {
    println!("Starting Ask For Authorization Outer");
    std::thread::spawn(||{
        println!("Starting Ask For Authorization Inner");
        let authorized = block_on(request_authorization(AuthorizationOptions::Sound|AuthorizationOptions::Badge));
        dbg!(authorized);
        println!("Finished Ask For Authorization Inner");
    });

    loop {
        run_ns_run_loop_once();
        std::thread::sleep(Duration::from_millis(100));
    }
}
