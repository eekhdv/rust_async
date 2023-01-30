// mod web_server;
// use web_server::handlers;

// use tokio::net::TcpListener;

use std::collections::HashMap;
use std::error::Error;

use tokio;

use zbus::{zvariant::Value, Connection};

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let conn = Connection::session().await?;

    let m = conn.call_method(
        Some("org.freedesktop.Notifications"),
        "/org/freedesktop/Notifications",
        Some("org.freedesktop.Notifications"),
        "Notify",
        &(
            "test-app",
            0u32,
            "dialog-information",
            "Test title",
            "Test body, teeeest body",
            vec![""; 0],
            HashMap::<&str, &Value>::new(),
            5000,
        ),
    ).await?;
    let reply: u32 = m.body().unwrap();
    dbg!(reply);
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
