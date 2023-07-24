use std::time::Duration;

use async_io::block_on;
use mac_notification_sys::{*, un::notification::AuthorizationOptions};

fn main() {
    std::thread::spawn(||{
        block_on(async {
            println!("Asking for authorization");
            let authorized = un::request_authorization(AuthorizationOptions::Sound|AuthorizationOptions::Badge).await;
            println!("Finished authorization");
            match authorized {
                Ok(()) => {
                    println!("User authorized for one or many options");
                    let badge_updated = un::set_badge_count(30).await;
                    match badge_updated {
                        Ok(()) => {
                            println!("Badge updated");
                        },
                        Err(e) => {
                            println!("Error occured while updating the badge. {:?}", e);
                        }
                    }
                },
                Err(e) => {
                    println!("Error occured while authorization step. {:?}", e);
                }
            }
        });
    });

    loop {
        run_ns_run_loop_once();
        std::thread::sleep(Duration::from_millis(100));
    }
}
