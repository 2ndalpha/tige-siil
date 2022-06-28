use std::io::prelude::*;
use std::net::TcpListener;
use std::net::TcpStream;
use std::{thread, time};

use threadpool::ThreadPool;

fn listen_webhook_events() {
    let listener = TcpListener::bind("127.0.0.1:80").unwrap();
    let pool = ThreadPool::new(20);

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
    respond_ok(stream);

    let request = String::from_utf8_lossy(&buffer[..]);
    let parts: Vec<&str> = request.split("\n").collect();

    let payload = parts[parts.len() - 1].trim_matches(char::from(0));
    println!("Request: {}", payload);

    if payload.contains("ticket_created") {
        play_drums();
    }
    if payload.contains("ticket_changed") {
        play_fast_drums();
    }
    if payload.contains("ticket_served") {
        play_super_fast_drums();
    }
    if payload.contains("ticket_called") {
        let length = payload.chars().count();
        println!("Length: {length}");

        let seed = length % 127;
        println!("Seed: {seed}");
        play(seed);
    }
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

// Channel for drums: 10
// Notes between 24
fn play_drums() {
    for n in 1..40 {
        let mut velocity = 60;
        if n % 4 == 0 {
            velocity = 127
        }
        play_note(36, 10, velocity);
        delay(400);
    }
}

fn play_fast_drums() {
    for _ in 1..40 {
        play_note(48, 10, 127);
        delay(1000);
    }
}

fn play_super_fast_drums() {
    for _ in 1..40 {
        play_note(42, 10, 127);
        delay(50);
    }
}

fn play(seed: usize) {
    play_note(seed + 40, 0, 127);
    delay(20 * seed);

    play_note(seed + 12, 0, 127);
    delay(seed);

    play_note(seed, 0, 127);

    delay(200 + 10 * seed);

    stop_note(seed + 40, 0);
    stop_note(seed + 12, 0);
    stop_note(seed, 0);
}

fn delay(milliseconds: usize) {
    thread::sleep(time::Duration::from_millis(
        milliseconds.try_into().unwrap(),
    ));
}

fn play_note(note: usize, channel: usize, velocity: usize) {
    let client = reqwest::blocking::Client::new();
    let result = client
        .post("http://ec2-13-48-30-252.eu-north-1.compute.amazonaws.com:3000/")
        .header("Content-Type", "application/json")
        .body(format!(
            "{{\"note\": {note}, \"velocity\": {velocity}, \"channel\": {channel}, \"isOn\": true }}"
        ))
        .send();
    match result {
        Err(e) => println!("Failed to start the note{:?}", e),
        _ => (),
    }
}

fn stop_note(note: usize, channel: usize) {
    let client = reqwest::blocking::Client::new();
    let result = client
        .post("http://ec2-13-48-30-252.eu-north-1.compute.amazonaws.com:3000/")
        .header("Content-Type", "application/json")
        .body(format!(
            "{{\"note\": {note}, \"velocity\": 0, \"channel\": {channel}, \"isOn\": false }}"
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
