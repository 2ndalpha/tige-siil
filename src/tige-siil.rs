use std::io::prelude::*;
use std::net::TcpListener;
use std::net::TcpStream;

fn listen_webhook_events() {
    let listener = TcpListener::bind("127.0.0.1:80").unwrap();

    for stream in listener.incoming() {
        let stream = stream.unwrap();
        handle_connection(stream);
    }
}

fn handle_connection(mut stream: TcpStream) {
    let mut buffer = [0; 1024];

    stream.read(&mut buffer).unwrap();

    if !buffer.starts_with(b"POST /") {
        respond_bad_method(stream);
        return
    }

    let request = String::from_utf8_lossy(&buffer[..]);
    let parts: Vec<&str> = request.split("\n").collect();

    println!("Request: {}", parts[parts.len() - 1]);
    respond_ok(stream);
}

fn respond_ok(mut stream: TcpStream) {
    let response = "HTTP/1.1 200 OK\r\n\r\n";

    stream.write(response.as_bytes()).unwrap();
    stream.flush().unwrap();
}

fn respond_bad_method(mut stream: TcpStream) {
    let response = "HTTP/1.1 405 Method Not Allowed\r\n\r\n";

    stream.write(response.as_bytes()).unwrap();
    stream.flush().unwrap();
}

fn main() {
    println!("Hello World!");
    listen_webhook_events();
}