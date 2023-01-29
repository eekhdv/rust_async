use std::fs;
use std::pin::Pin;
use std::io::Result;
use std::task::{Context, Poll};
use tokio::io::{AsyncRead, AsyncWrite, AsyncWriteExt, AsyncReadExt};
use tokio::time;
use tokio::net::{TcpListener,TcpStream};
use tokio::task;

struct MockTcpStream {
    read_data: Vec<u8>,
    write_data: Vec<u8>,
}

impl AsyncRead for MockTcpStream {
    fn poll_read(
            self: Pin<&mut Self>,
            _: &mut Context<'_>,
            buf: &mut tokio::io::ReadBuf<'_>,
        ) -> Poll<Result<()>> {
        let size: usize = std::cmp::min(self.read_data.len(), buf.capacity());
        buf.clear();
        buf.put_slice(&self.read_data);
        Poll::Ready(Ok(size))
    }
}

impl AsyncWrite for MockTcpStream {
    fn poll_write(
            self: Pin<&mut Self>,
            _: &mut Context<'_>,
            buf: &[u8],
        ) -> Poll<std::result::Result<usize, std::io::Error>> {
        self.write_data = Vec::from(buf);

        Poll::Ready(Ok(buf.len()))
    }

    fn poll_flush(self: Pin<&mut Self>, _: &mut Context<'_>) -> Poll<std::result::Result<(), std::io::Error>> {
        Poll::Ready(Ok(()))
    }

    fn poll_shutdown(self: Pin<&mut Self>, _: &mut Context<'_>) -> Poll<std::result::Result<(), std::io::Error>> {
        Poll::Ready(Ok(()))
    }
}


#[tokio::main]
async fn main() {
    // Listen for incoming TCP connections on localhost port 7878
    let listener = TcpListener::bind("127.0.0.1:7878").await.unwrap();
    loop {
        let (socket, _) = listener.accept().await.unwrap();
        handle_connection(socket).await;
    }


    // Block forever, handling each request that arrives at this IP address
    // for stream in listener.incoming() {
    //     
    //     let stream = stream.unwrap();

    //     handle_connection(stream).await?;
    // }
}

async fn handle_connection(mut stream: impl AsyncWrite + AsyncRead + Unpin) {
    // Read the first 1024 bytes of data from the stream
    let mut buffer = [0; 1024];
    stream.read_buf(&mut buffer);

    let get = b"GET / HTTP/1.1\r\n";
    let sleep = b"GET /sleep HTTP/1.1\r\n";

    // Respond with greetings or a 404,
    // depending on the data in the request
    let (status_line, filename) = if buffer.starts_with(get) {
        ("HTTP/1.1 200 OK\r\n\r\n", "hello.html")
    } else if buffer.starts_with(sleep) {
        time::sleep(time::Duration::from_secs(5)).await;
        ("HTTP/1.1 200 OK\r\n\r\n", "hello.html")
    } else {
        ("HTTP/1.1 404 NOT FOUND\r\n\r\n", "404.html")
    };
    let contents = fs::read_to_string(filename).unwrap();

    // Write response back to the stream,
    // and flush the stream to ensure the response is sent back to the client
    let response = format!("{status_line}{contents}");
    stream.write_all(response.as_bytes());
}
