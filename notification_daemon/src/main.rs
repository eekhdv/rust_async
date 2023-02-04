mod dbus;
use dbus::prep_notifications::DbusChannel;

mod terminal;
use terminal::drawing::NotificationBox;
use terminal::screen::ScreenDimensions;

use std::error::Error;
use std::process::exit;
use std::sync::Arc;

use tokio;
use tokio::sync::mpsc;

use console_engine::rect_style::BorderStyle;
use console_engine::{ConsoleEngine, KeyCode};

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let (dbus_tx, mut dbus_rx) = mpsc::channel(128);
    let notif_handler = dbus::raw_handlers::NotificationsHandler {
        dbus_tx: (dbus_tx),
        n_counter: 0,
    };

    let notif_box = NotificationBox::new();
    let n_catcher = Arc::clone(&notif_box.notifications);
    let n_drawer = Arc::clone(&notif_box.notifications);

    dbus::connection::setup_server(notif_handler).await?;

    tokio::task::spawn(async move {
        while let Some(n) = dbus_rx.recv().await {
            match n {
                DbusChannel::Notify { notification } => {
                    let mut lock = n_catcher.lock().await;
                    let expire_timeout = notification.expire_timeout;
                    let unique_id = notification.unique_id;
                    let index = lock
                        .iter()
                        .position(|x| x.unique_id == notification.unique_id);
                    if let Some(i) = index {
                        if let Some(v) = lock.get_mut(i) {
                            v.app_name = notification.app_name;
                            v.title = notification.title;
                            v.body = notification.body;
                        }
                    } else {
                        lock.push(notification);
                    }
                    if expire_timeout == i32::MAX {
                        continue;
                    }
                    tokio::task::spawn(async move {
                        tokio::time::sleep(std::time::Duration::from_millis(expire_timeout as u64))
                            .await;
                        let connection = zbus::Connection::session().await.unwrap();
                        let _ = connection
                            .call_method(
                                Some("org.freedesktop.Notifications"),
                                "/org/freedesktop/Notifications",
                                Some("org.freedesktop.Notifications"),
                                "CloseNotification",
                                &(unique_id),
                            )
                            .await
                            .unwrap();
                    });
                }
                DbusChannel::CloseNotification { unique_id } => {
                    let mut lock = n_catcher.lock().await;
                    let index = lock.iter().position(|x| x.unique_id == unique_id).unwrap();
                    lock.remove(index);
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

            if engine.is_key_pressed(KeyCode::Char('q')) {
                ConsoleEngine::init_fill(10).unwrap();
                exit(0);
            }

            let lock = n_drawer.lock().await;
            let mut cur_x = 1;
            let mut cur_y = 1;

            engine.print(
                0,
                0,
                format!("width: {}, height: {}", cur_screen.width, cur_screen.height).as_str(),
            );
            for notif_box in lock.iter() {
                let app_name_len: i32 = notif_box.app_name.len().try_into().unwrap();
                let body_len: i32 = notif_box.body.len().try_into().unwrap();
                engine.rect_border(
                    cur_x,
                    cur_y,
                    cur_x + 4 + {
                        if app_name_len > body_len {
                            app_name_len
                        } else {
                            body_len
                        }
                    },
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
            }
            engine.draw(); // draw the screen
        }
    });

    loop {
        std::future::pending::<()>().await;
    }
}
