use std::io::prelude::*;
use std::net::TcpListener;
use std::net::TcpStream;
use std::{thread, time};

use tokio::runtime::Builder;
use tokio::spawn;

const MIDI_PLAYER_URL: &str = "http://ec2-13-48-30-252.eu-north-1.compute.amazonaws.com:3000/";

fn listen_webhook_events() {
    let listener = TcpListener::bind("127.0.0.1:80").unwrap();

    for stream in listener.incoming() {
        let stream = stream.unwrap();

        spawn(async move {
            handle_connection(stream).await;
        });
    }
}

async fn handle_connection(mut stream: TcpStream) {
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

    if payload.to_lowercase().contains("mario") {
        play_mario().await;
    } else if payload.to_lowercase().contains("fast drums") && payload.contains("\"On\"") {
        turn_on_fast_drums();
    } else if payload.to_lowercase().contains("fast drums") && payload.contains("\"Off\"") {
        turn_off_fast_drums();
    } else if payload.to_lowercase().contains("alternative drums") && payload.contains("\"On\"") {
        turn_on_alternative_drums();
    } else if payload.to_lowercase().contains("alternative drums") && payload.contains("\"Off\"") {
        turn_off_alternative_drums();
    } else if payload.to_lowercase().contains("drums") && payload.contains("\"On\"") {
        turn_on_drums();
    } else if payload.to_lowercase().contains("drums") && payload.contains("\"Off\"") {
        turn_off_drums();
    } else if payload.contains("ticket_createdX") {
        play_drums();
    } else if payload.contains("ticket_changedX") {
        play_fast_drums();
    } else if payload.contains("ticket_servedX") {
        play_super_fast_drums();
    } else if payload.contains("ticket_calledX") {
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

async fn play_mario() {
    let part1 = [
        76, 12, 76, 12, 20, 12, 76, 12, 20, 12, 72, 12, 76, 12, 20, 12, 79, 12, 20, 36, 67, 12, 20,
        36,
    ];
    let part2 = [
        72, 12, 20, 24, 67, 12, 20, 24, 64, 12, 20, 24, 69, 12, 20, 12, 71, 12, 20, 12, 70, 12, 69,
        12, 20, 12, 67, 16, 76, 16, 79, 16, 81, 12, 20, 12, 77, 12, 79, 12, 20, 12, 76, 12, 20, 12,
        72, 12, 74, 12, 71, 12, 20, 24,
    ];
    let part3 = [
        48, 12, 20, 12, 79, 12, 78, 12, 77, 12, 75, 12, 60, 12, 76, 12, 53, 12, 68, 12, 69, 12, 72,
        12, 60, 12, 69, 12, 72, 12, 74, 12, 48, 12, 20, 12, 79, 12, 78, 12, 77, 12, 75, 12, 55, 12,
        76, 12, 20, 12, 84, 12, 20, 12, 84, 12, 84, 12,
    ];
    let part4 = [
        55, 12, 20, 12, 48, 12, 20, 12, 79, 12, 78, 12, 77, 12, 75, 12, 60, 12, 76, 12, 53, 12, 68,
        12, 69, 12, 72, 12, 60, 12, 69, 12, 72, 12, 74, 12, 48, 12, 20, 12, 75, 24, 20, 12, 74, 24,
        20, 12, 72, 24, 20, 12, 55, 12, 55, 12, 20, 12, 48, 12,
    ];
    let part5 = [
        72, 12, 72, 12, 20, 12, 72, 12, 20, 12, 72, 12, 74, 12, 20, 12, 76, 12, 72, 12, 20, 12, 69,
        12, 67, 12, 20, 12, 43, 12, 20, 12, 72, 12, 72, 12, 20, 12, 72, 12, 20, 12, 72, 12, 74, 12,
        76, 12, 55, 12, 20, 24, 48, 12, 20, 24, 43, 12, 20, 12, 72, 12, 72, 12, 20, 12, 72, 12, 20,
        12, 72, 12, 74, 12, 20, 12, 76, 12, 72, 12, 20, 12, 69, 12, 67, 12, 20, 12, 43, 12, 20, 12,
        76, 12, 76, 12, 20, 12, 76, 12, 20, 12, 72, 12, 76, 12, 20, 12, 79, 12, 20, 36, 67, 12, 20,
        36,
    ];
    let part6 = [
        76, 12, 72, 12, 20, 12, 67, 12, 55, 12, 20, 12, 68, 12, 20, 12, 69, 12, 77, 12, 53, 12, 77,
        12, 69, 12, 60, 12, 53, 12, 20, 12, 71, 16, 81, 16, 81, 16, 81, 16, 79, 16, 77, 16, 76, 12,
        72, 12, 55, 12, 69, 12, 67, 12, 60, 12, 55, 12, 20, 12, 76, 12, 72, 12, 20, 12, 67, 12, 55,
        12, 20, 12, 68, 12, 20, 12, 69, 12, 77, 12, 53, 12, 77, 12, 69, 12, 60, 12, 53, 12, 20, 12,
        71, 12, 77, 12, 20, 12, 77, 12, 77, 16, 76, 16, 74, 16, 72, 12, 64, 12, 55, 12, 64, 12, 60,
        12, 20, 36,
    ];
    let part7 = [
        72, 12, 20, 24, 67, 12, 20, 24, 64, 24, 69, 16, 71, 16, 69, 16, 68, 24, 70, 24, 68, 24, 67,
        12, 65, 12, 67, 48,
    ];

    play_mario_notes(&part1);

    play_mario_notes(&part2);
    play_mario_notes(&part2);

    let part3_join_handle = spawn(async move {
        play_mario_notes(&part3);
    });
    let part4_join_handle = spawn(async move {
        play_mario_notes(&part4);
    });

    match part3_join_handle.await {
        Ok(_) => (),
        Err(_) => println!("Failed to play part 3"),
    }
    match part4_join_handle.await {
        Ok(_) => (),
        Err(_) => println!("Failed to play part 4"),
    }

    play_mario_notes(&part5);

    play_mario_notes(&part2);
    play_mario_notes(&part2);

    play_mario_notes(&part6);
    play_mario_notes(&part6);

    play_mario_notes(&part5);
    play_mario_notes(&part6);
    play_mario_notes(&part7);
}

fn play_mario_notes(notes: &[usize]) {
    let mut last_note: &usize = &0;
    for (i, note_or_delay) in notes.iter().enumerate() {
        if i % 2 == 1 {
            delay(*note_or_delay * 10);
            stop_note(*last_note, 1);
        } else {
            play_note(*note_or_delay, 1, 127);
            last_note = note_or_delay;
        }
    }
}

fn turn_on_drums() {
    println!("Turning ON the drums");
    play_note(36, 11, 127);
}

fn turn_off_drums() {
    println!("Turning OFF the drums");
    stop_note(36, 11);
}

fn turn_on_alternative_drums() {
    println!("Turning ON the alternative drums");
    play_note(44, 11, 127);
}

fn turn_off_alternative_drums() {
    println!("Turning OFF the alternative drums");
    stop_note(44, 11);
}

fn turn_on_fast_drums() {
    println!("Turning ON the fast drums");
    play_note(38, 13, 90);
}

fn turn_off_fast_drums() {
    println!("Turning OFF the fast drums");
    stop_note(38, 13);
}

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
    for i in 1..40 {
        play_note(10 + i, 10, 127);
        delay(100);
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
    let client = reqwest::Client::new();
    spawn(async move {
        println!("Playing note {note}");
        let result = client
            .post(MIDI_PLAYER_URL)
            .header("Content-Type", "application/json")
            .body(format!(
                "{{\"note\": {note}, \"velocity\": {velocity}, \"channel\": {channel}, \"isOn\": true }}"
            ))
            .send().await;
        match result {
            Err(e) => println!("Failed to start the note{:?}", e),
            _ => (),
        }
    });
}

fn stop_note(note: usize, channel: usize) {
    let client = reqwest::Client::new();
    spawn(async move {
        let result = client
            .post(MIDI_PLAYER_URL)
            .header("Content-Type", "application/json")
            .body(format!(
                "{{\"note\": {note}, \"velocity\": 127, \"channel\": {channel}, \"isOn\": false }}"
            ))
            .send()
            .await;
        match result {
            Err(e) => println!("Failed to stop the note{:?}", e),
            _ => (),
        }
    });
}

fn main() {
    println!("🦔 Let's make some noise!");

    let runtime = Builder::new_multi_thread()
        .worker_threads(20)
        .thread_stack_size(3 * 1024 * 1024)
        .enable_all()
        .build()
        .unwrap();

    runtime.block_on(async move {
        listen_webhook_events();
    });
}
