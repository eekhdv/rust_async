// mod web_server;
// use web_server::handlers;

// use tokio::net::TcpListener;
mod dbus;
use dbus::service;

use std::error::Error;

use tokio;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    service::setup_server().await?;

    loop {
        std::future::pending::<()>().await;
    }
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
