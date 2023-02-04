mod dbus;
// use dbus::prep_notifications::Rect;
use dbus::{prep_notifications, raw_handlers, service};

use std::error::Error;
use std::sync::Arc;

use tokio;
use tokio::sync::{mpsc, Mutex};

use console_engine::rect_style::BorderStyle;
use console_engine::ConsoleEngine;

#[derive(Debug, Clone)]
pub struct NotificationsDrawer {
    pub notification_boxes: Arc<Mutex<Vec<prep_notifications::Notification>>>,
}

pub struct ScreenDimensions {
    pub width: u32,
    pub height: u32,
}

impl NotificationsDrawer {
    pub fn new() -> Self {
        NotificationsDrawer {
            notification_boxes: Arc::new(Mutex::new(vec![])),
        }
    }
}

impl ScreenDimensions {
    pub fn new(w: u32, h: u32) -> Self {
        Self {
            width: w,
            height: h,
        }
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let (dbus_tx, mut dbus_rx) = mpsc::channel(128);
    let notif_handler = raw_handlers::NotificationsHandler { dbus_tx: (dbus_tx) };

    let notif_drawer = NotificationsDrawer::new();
    let notif_clone = Arc::clone(&notif_drawer.notification_boxes);
    let notif_drawer_clone = Arc::clone(&notif_drawer.notification_boxes);

    service::setup_server(notif_handler).await?;

    tokio::task::spawn(async move {
        while let Some(n) = dbus_rx.recv().await {
            match n {
                prep_notifications::DbusChannel::Notify { notification } => {
                    let mut lock1 = notif_clone.lock().await;
                    lock1.push(notification);
                }
            }
        }
    });

    tokio::task::spawn(async move {
        let mut engine = ConsoleEngine::init_fill(1).unwrap();
        let cur_screen = ScreenDimensions::new(engine.get_width(), engine.get_height());
        loop {
            engine.wait_frame();
            engine.check_resize();
            engine.clear_screen();

            let lock = notif_drawer_clone.lock().await;
            let mut cur_x = 1;
            let mut cur_y = 1;

            engine.print(
                0,
                0,
                format!("width: {}, height: {}", cur_screen.width, cur_screen.height).as_str(),
            );
            for notif_box in lock.iter() {
                // if engine.is_key_pressed(KeyCode::Char('q')) {
                //     break;
                // }

                let app_name_len: i32 = notif_box.app_name.len().try_into().unwrap();
                let body_len: i32 = notif_box.body.len().try_into().unwrap();
                engine.rect_border(
                    cur_x,
                    cur_y,
                    cur_x
                        + 2
                        + {
                            if app_name_len > body_len {
                                app_name_len
                            } else {
                                body_len
                            }
                        }
                        + 2,
                    cur_y + 6,
                    BorderStyle::new_light(),
                );

                engine.print(cur_x + 2, cur_y + 1, notif_box.app_name.as_str());
                engine.print(cur_x + 2, cur_y + 3, notif_box.title.as_str());
                engine.print(cur_x + 2, cur_y + 5, notif_box.body.as_str());

                cur_y += 8;

                if cur_y >= cur_screen.height.try_into().unwrap() {
                    cur_x += 30;
                    cur_y = 1;
                }
                engine.draw(); // draw the screen
            }
            tokio::time::sleep(tokio::time::Duration::from_secs(1)).await;
        }
    });

    loop {
        std::future::pending::<()>().await;
    }
}
