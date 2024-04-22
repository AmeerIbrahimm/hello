use std::io::{Write, Read};
use std::time::Duration;
use std::fs;
use async_std::task;
use async_std::net::{TcpListener,TcpStream};
use async_std::prelude::*;
use futures::stream::StreamExt;

#[async_std::main]
async fn main() {
    let listener = TcpListener::bind("127.0.0.1:7878").await.unwrap();

    // for stream in listener.incoming() {
    //     let stream = stream.unwrap();
    //     handle_connection(stream).await;
    // }
    listener.incoming().for_each_concurrent(None,|tcpstream| async move {
        let stream = tcpstream.unwrap();
        handle_connection(stream).await;
}).await;
}

async fn handle_connection(mut stream: impl Read + Write + Unpin) {
    let mut buffer = [0; 1024];
    stream.read(&mut buffer).await.unwrap();
    let sleep = b"GET /sleep HTTP/1.1\r\n";
    let get = b"GET / HTTP/1.1\r\n";
    let (status_line, filename) = if buffer.starts_with(get) {
        ("HTTP/1.1 200 OK", "hello.html")
    } else if buffer.starts_with(sleep) {
        task::sleep(Duration::from_secs(5)).await;
        ("HTTP/1.1 200 OK", "hello.html")
    } else {
        ("HTTP/1.1 404 NOT FOUND", "404.html")
    };
    let contents = fs::read_to_string(filename).unwrap();
    let length = contents.len();
    let response = format!("{status_line}\r\nContent-length: {length}\r\n\r\n{contents}");
    stream.write_all(response.as_bytes()).await.unwrap();
}
