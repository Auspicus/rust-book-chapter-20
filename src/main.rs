use rust_book_chap_20::ThreadPool;
use std::io::prelude::*;
use std::net::{TcpListener, TcpStream};

fn main() {
    let listener = TcpListener::bind("127.0.0.1:7878").unwrap();
    let pool = ThreadPool::new(4);

    for stream in listener.incoming().take(2) {
        let mut stream = stream.unwrap();

        pool.execute(move || {
            handle_connection(&mut stream);
        });
    }
}

fn handle_connection(stream: &mut TcpStream) {
    let mut buffer = [0; 1024];
    let _ = stream.read(&mut buffer).unwrap();

    let response =
        "HTTP/1.1 200 OK\r\n\r\n<html><body><h1>Hello, world!</h1></body></html>";

    let _ = stream.write(response.as_bytes()).unwrap();
    stream.flush().unwrap();
}