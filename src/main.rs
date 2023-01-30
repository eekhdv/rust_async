// mod web_server;
// use web_server::handlers;

// use tokio::net::TcpListener;
mod dbus;
use dbus::notifications;

use std::error::Error;

use tokio;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let _send = notifications::send_notif().await?;
    Ok(())
}

// #[tokio::main] // ----------- web_server
// async fn main() {
// let listener = TcpListener::bind("127.0.0.1:7878").await.unwrap();
// loop {
//     let (socket, _) = listener.accept().await.unwrap();
//     tokio::task::spawn(async move {
//         handlers::con_handler(socket).await;
//     });
// }
// }
