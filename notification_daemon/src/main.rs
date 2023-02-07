mod dbus;
use dbus::prep_notifications::{set_notif_lifetime, DbusChannel};
use zbus::Connection;

mod terminal;
use terminal::drawing::methods::*;
use terminal::drawing::NotificationBox;
use terminal::screen::ScreenDimensions;

use std::error::Error;
use std::process::exit;
use std::sync::Arc;

use tokio;
use tokio::sync::mpsc;

use console_engine::{ConsoleEngine, KeyCode};

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let (dbus_tx, mut dbus_rx) = mpsc::channel(128);
    let notif_handler = dbus::raw_handlers::NotificationsHandler {
        dbus_tx: (dbus_tx),
        n_counter: 0,
    };

    let srvc_name = "org.freedesktop.Notifications";
    let srvc_obj = "/org/freedesktop/Notifications";

    let connection = Connection::session()
        .await?;
    connection
        .object_server()
        .at(srvc_obj, notif_handler)
        .await?;
    connection
        .request_name(srvc_name)
        .await?;

    let notif_box = NotificationBox::new();
    let n_catcher = Arc::clone(&notif_box.notifications);
    let n_drawer = Arc::clone(&notif_box.notifications);

    tokio::task::spawn(async move {
        while let Some(n) = dbus_rx.recv().await {
            match n {
                DbusChannel::Notify { notification } => {
                    let mut lock = n_catcher.lock().await;

                    let unique_id: u32 = notification.unique_id;
                    let expire_timeout: i32 = notification.expire_timeout;

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
                    if expire_timeout != i32::MAX {
                        set_notif_lifetime(unique_id, expire_timeout as u64);
                    }
                }
                DbusChannel::CloseNotification { unique_id } => {
                    let mut lock = n_catcher.lock().await;
                    if let Some(index) = lock.iter().position(|x| x.unique_id == unique_id) {
                        lock.remove(index);
                    }
                }
            }
        }
    });

    tokio::task::spawn(async move {
        let mut engine = ConsoleEngine::init_fill(10).unwrap();
        let mut longest_field = 0;
        let mut cur_screen =
            ScreenDimensions::new(engine.get_width() as i32, engine.get_height() as i32);

        loop {
            engine.wait_frame();
            engine.check_resize();
            engine.clear_screen();
            cur_screen.width = engine.get_width() as i32;
            cur_screen.height = engine.get_height() as i32;

            if engine.is_key_pressed(KeyCode::Char('q')) {
                ConsoleEngine::init_fill(10).unwrap();
                exit(0);
            }

            let lock = n_drawer.lock().await;
            let mut left_up: (i32, i32) = (1, 1);

            draw_frame(&mut engine, cur_screen.width - 1, cur_screen.height - 1);

            for notif_box in lock.iter() {
                let app_name = &notif_box.app_name;
                let title = &notif_box.title;
                let body = &notif_box.body;

                let cur_max_field = get_longest_field(app_name.len(), title.len(), body.len());
                if cur_max_field > longest_field {
                    longest_field = cur_max_field
                };

                draw_box_for_notification(&mut engine, left_up, cur_max_field as i32);

                print_app_name(&mut engine, left_up, app_name);
                print_title(&mut engine, left_up, title);
                print_body(&mut engine, left_up, body);

                left_up.1 += 8;

                if left_up.1 >= cur_screen.height - 3 {
                    move_next_line(&mut left_up, &mut longest_field);
                }
            }
            engine.draw(); // draw the screen
        }
    });

    loop {
        std::future::pending::<()>().await;
    }
}
