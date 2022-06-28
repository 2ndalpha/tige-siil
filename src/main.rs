use std::io::prelude::*;
use std::net::TcpListener;
use std::net::TcpStream;
use std::error::Error;
use std::{thread, time};

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
    play_new_ticket();
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

fn play_new_ticket() {
    play_note(80);
    delay(500);

    play_note(52);
    delay(500);

    play_note(40);

    delay(2000);

    stop_note(80);
    stop_note(52);
    stop_note(40);
}

fn delay(mut milliseconds: u64) {
    thread::sleep(time::Duration::from_millis(milliseconds));
}

fn play_note(mut note: u8) -> Result<(), Box<dyn Error>> {
    let client = reqwest::blocking::Client::new();
    let resp = client.post("https://fb72-195-250-172-87.eu.ngrok.io/")
    .header("Content-Type", "application/json")
    .body(format!("{{\"note\": {note}, \"velocity\": 20, \"channel\": 0, \"isOn\": true }}"))
    .send()?;

    println!("{:#?}", resp);
    Ok(())
}

fn stop_note(mut note: u8) -> Result<(), Box<dyn Error>> {
    let client = reqwest::blocking::Client::new();
    let resp = client.post("https://fb72-195-250-172-87.eu.ngrok.io/")
    .header("Content-Type", "application/json")
    .body(format!("{{\"note\": {note}, \"velocity\": 20, \"channel\": 0, \"isOn\": false }}"))
    .send()?;

    println!("{:#?}", resp);
    Ok(())
}

fn main() {
    println!("🦔 Let's make some noise!");
    listen_webhook_events();
}