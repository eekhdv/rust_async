use console_engine::{pixel, Color, KeyCode, ConsoleEngine};
mod dbus;
use dbus::{prep_notifications, raw_handlers, service};

use std::error::Error;
use std::sync::Arc;

use tokio;
use tokio::sync::{mpsc, Mutex};

#[derive(Debug, Clone)]
struct NotificationsDrawer {
    // notification_boxes: Rc<RefCell<Vec<prep_notifications::Notification>>>,
    notification_boxes: Arc<Mutex<Vec<prep_notifications::Notification>>>,
}

struct ScreenDimensions {
    width: u32,
    height: u32,
}


// impl Iterator for NotificationsDrawer {
//     type Item = prep_notifications::Notification;
// 
//     fn next(&mut self) -> Option<Self::Item> {
//         self.notification_boxes.pop()
//     }
// }

impl NotificationsDrawer {
    fn new() -> Self {
        NotificationsDrawer { notification_boxes: Arc::new(Mutex::new(vec![])) } 
    }
}

impl ScreenDimensions {
    fn new(w: u32, h: u32) -> Self {
        Self { width: w, height: h }
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    // initializes a screen filling the terminal of at least 50x20 of size with a target of 3 frame per second
    // let mut engine = console_engine::ConsoleEngine::init_fill(10).unwrap();

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
                    // let _engine = console_engine::ConsoleEngine::init_fill_require(50, 20, 10).unwrap();
                    let mut lock1 = notif_clone.lock().await;
                    lock1.push(notification);
                    // notif_clone.try_lock().unwrap().push(notification);
                    // println!("test-> {:#?}", lock1.pop());
                }
            }
        }
    });

    tokio::task::spawn(async move {
        let mut engine = console_engine::ConsoleEngine::init_fill(10).unwrap();
        let cur_screen = ScreenDimensions::new(engine.get_width(), engine.get_height());
        loop {
            let lock = notif_drawer_clone.lock().await;
            for _notif_box in lock.iter() {
                engine.wait_frame(); // wait for next frame + capture inputs
                engine.check_resize(); // resize the terminal if its size has changed
                                       // exit check
                // if engine.is_key_pressed(KeyCode::Char('q')) {
                //     break;
                // }
                engine.clear_screen();
                engine.print(0, 0, format!("width: {}, height: {}", cur_screen.width, cur_screen.height).as_str());
                engine.rect(
                    4,
                    4,
                    18,
                    12,
                    pixel::pxl('#'),
                );

                engine.rect(
                    20,
                    20,
                    50,
                    39,
                    pixel::pxl('#'),
                );

                engine.print(4, 4 + (12 - 4) / 2, "hello");
                engine.draw(); // draw the screen
            }
        }
    });

    loop {
        std::future::pending::<()>().await;
    }

    // main loop, be aware that you'll have to break it because ctrl+C is captured
    // loop {
    //     engine.wait_frame(); // wait for next frame + capture inputs
    //     engine.check_resize(); // resize the terminal if its size has changed
    //                            // exit check
    //     if engine.is_key_pressed(KeyCode::Char('q')) {
    //         break;
    //     }
    //     engine.clear_screen();
    //     engine.print(0, 0, format!("{}", engine.get_screen().get_height()).as_str());
    //     engine.rect(
    //         4,
    //         4,
    //         18,
    //         12,
    //         pixel::pxl('#'),
    //     );

    //     engine.rect(
    //         20,
    //         20,
    //         50,
    //         39,
    //         pixel::pxl('#'),
    //     );

    //     engine.print(4, 4 + (12 - 4) / 2, "hello");
    //     engine.draw(); // draw the screen
    // }
}
