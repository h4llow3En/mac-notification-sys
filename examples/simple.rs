use std::time::Duration;

use async_io::block_on;
use mac_notification_sys::*;

fn main() {
    std::thread::spawn(|| {
        block_on(async {
            println!("Asking for authorization");
            let authorized = un::request_authorization(
                un::notification::AuthorizationOptions::Sound | un::notification::AuthorizationOptions::Badge,
            )
            .await;
            println!("Finished authorization");
            match authorized {
                Ok(()) => {
                    println!("User authorized for one or many options");
                    let badge_updated = un::set_badge_count(30).await;
                    match badge_updated {
                        Ok(()) => {
                            let category_id = "test";
                            let category = un::builder::CategoryBuilder::new()
                                .identifier(category_id)
                                .action(un::builder::ActionBuilder::new_with_title("Test").build())
                                .build();
                            un::set_categories(vec![category]);
                            println!("Categories set");
                            let notification =
                                un::builder::NotificationBuilder::new_with_body("Test")
                                    .title("test")
                                    .category_id(category_id)
                                    .build();
                            let res = un::add_notification(notification).await;
                            println!("Notification sent status {:?}", res);
                        }
                        Err(e) => {
                            println!("Error occured while updating the badge. {:?}", e);
                        }
                    }
                }
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
