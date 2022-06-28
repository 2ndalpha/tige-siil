use std::io::prelude::*;
use std::net::TcpListener;
use std::net::TcpStream;
use std::{thread, time};

use threadpool::ThreadPool;

fn listen_webhook_events() {
    let listener = TcpListener::bind("127.0.0.1:80").unwrap();
    let pool = ThreadPool::new(8);

    for stream in listener.incoming() {
        let stream = stream.unwrap();

        pool.execute(|| {
            handle_connection(stream);
        });
    }
}

fn handle_connection(mut stream: TcpStream) {
    let mut buffer = [0; 1024];

    stream.read(&mut buffer).unwrap();

    if !buffer.starts_with(b"POST /") {
        respond_bad_method(stream);
        return;
    }

    let request = String::from_utf8_lossy(&buffer[..]);
    let parts: Vec<&str> = request.split("\n").collect();

    let payload = parts[parts.len() - 1].trim_matches(char::from(0));
    println!("Request: {}", payload);

    let length = payload.chars().count();
    println!("Length: {length}");

    let seed = length % 127;
    println!("Seed: {seed}");
    play(seed);

    respond_ok(stream)
}

fn respond_ok(mut stream: TcpStream) {
    let response = "HTTP/1.1 200 OK\r\n\r\n";

    stream.write(response.as_bytes()).unwrap();
    stream.flush().unwrap();
}

fn respond_bad_method(mut stream: TcpStream) {
    let response = "HTTP/1.1 405 Method Not Allowed\r\n\r\nMethod Not Allowed";

    stream.write(response.as_bytes()).unwrap();
    stream.flush().unwrap();
}

fn play(seed: usize) {
    play_note(seed + 40);
    delay(20 * seed);

    play_note(seed + 12);
    delay(seed);

    play_note(seed);

    delay(200 + 10 * seed);

    stop_note(seed + 40);
    stop_note(seed + 12);
    stop_note(seed);
}

fn delay(milliseconds: usize) {
    thread::sleep(time::Duration::from_millis(
        milliseconds.try_into().unwrap(),
    ));
}

fn play_note(note: usize) {
    let channel = note % 3;
    let client = reqwest::blocking::Client::new();
    let result = client
        .post("http://ec2-13-48-30-252.eu-north-1.compute.amazonaws.com:3000/")
        .header("Content-Type", "application/json")
        .body(format!(
            "{{\"note\": {note}, \"velocity\": 127, \"channel\": {channel}, \"isOn\": true }}"
        ))
        .send();
    match result {
        Err(e) => println!("Failed to start the note{:?}", e),
        _ => (),
    }
}

fn stop_note(note: usize) {
    let client = reqwest::blocking::Client::new();
    let result = client
        .post("http://ec2-13-48-30-252.eu-north-1.compute.amazonaws.com:3000/")
        .header("Content-Type", "application/json")
        .body(format!(
            "{{\"note\": {note}, \"velocity\": 0, \"channel\": 0, \"isOn\": false }}"
        ))
        .send();
    match result {
        Err(e) => println!("Failed to stop the note{:?}", e),
        _ => (),
    }
}

fn main() {
    println!("🦔 Let's make some noise!");
    listen_webhook_events();
}
