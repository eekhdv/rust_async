mod dbus;
use dbus::{prep_notifications, raw_handlers, service};

use std::error::Error;

use tokio;
use tokio::sync::mpsc;
// use uuid::Uuid;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let (dbus_tx, mut dbus_rx) = mpsc::channel(128);

    let notif_handler = raw_handlers::NotificationsHandler { dbus_tx: (dbus_tx) };

    service::setup_server(notif_handler).await?;

    tokio::task::spawn(async move {
        while let Some(n) = dbus_rx.recv().await {
            match n {
                prep_notifications::DbusChannel::Notify { notification } => {
                    println!("{:?}", notification)
                }
            }
        }
    });

    loop {
        std::future::pending::<()>().await;
    }
}
