use std::fs;
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::net::TcpStream;
use tokio::time;

pub async fn con_handler(mut stream: TcpStream) {
    // Read the first 1024 bytes of data from the stream
    let mut buffer = Vec::with_capacity(1024);
    stream.read_buf(&mut buffer).await.unwrap();

    let get = b"GET / HTTP/1.1\r\n";
    let sleep = b"GET /sleep HTTP/1.1\r\n";

    // Respond with greetings or a 404,
    // depending on the data in the request
    let (status_line, filename) = if buffer.starts_with(get) {
        ("HTTP/1.1 200 OK\r\n\r\n", "web_server/src/hello.html")
    } else if buffer.starts_with(sleep) {
        time::sleep(time::Duration::from_secs(5)).await;
        ("HTTP/1.1 200 OK\r\n\r\n", "web_server/src/hello.html")
    } else {
        ("HTTP/1.1 404 NOT FOUND\r\n\r\n", "web_server/src/404.html")
    };
    let contents = fs::read_to_string(filename).unwrap();

    // Write response back to the stream,
    // and flush the stream to ensure the response is sent back to the client
    let response = format!("{status_line}{contents}");
    stream.write_all(response.as_bytes()).await.unwrap();
}
