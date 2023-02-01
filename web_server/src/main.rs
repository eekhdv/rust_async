mod services;

use services::handlers;
use tokio::net::TcpListener;

#[tokio::main]
async fn main() {
    let listener = TcpListener::bind("127.0.0.1:7878").await.unwrap();
    loop {
        let (socket, _) = listener.accept().await.unwrap();
        tokio::task::spawn(async move {
            handlers::con_handler(socket).await;
        });
    }
}
