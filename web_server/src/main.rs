mod services;

use tokio::net::TcpListener;
use services::handlers;

#[tokio::main]
async fn main() {
    // Listen for incoming TCP connections on localhost port 7878
    let listener = TcpListener::bind("127.0.0.1:7878").await.unwrap();

    loop {
        let (socket, _) = listener.accept().await.unwrap();
        tokio::task::spawn(async move {
            handlers::con_handler(socket).await;
        });
    }
}

