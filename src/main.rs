use anyhow::Result;
use obws::{responses::{recording::RecordStatus, streaming::StreamStatus}, Client};
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use std::{env, io};
use once_cell::sync::Lazy;

mod structs;

pub static ARGS: Lazy<Vec<String>> = Lazy::new(|| {
    let args: Vec<String> = env::args().collect();
    args
});

#[tokio::main]
async fn main() {
    let mut hostname: String = "localhost".to_string();
    let mut port: u16 = 4455;

    let mut password = String::new();
    println!("Enter OBS password:");

    match io::stdin().read_line(&mut password) {
        Ok(_) => {
            password = password.trim_end().to_string();
        }
        Err(error) => println!("Error: {}", error),
    }

    let mut output: structs::output = structs::output {
        stream: None,
        recording: None
    };

    if let Some(index) = env::args().position(|arg| arg == "--address") {
        if let Some(address) = env::args().nth(index + 1) {
            if let Some(hostname_str) = address.split(':').nth(0) {
                println!("hostname_str {}", hostname_str);
                hostname = hostname_str.to_string();
            }

            // Split the address string by ":" and parse the port as u16
            if let Some(port_str) = address.split(':').nth(1) {
                if let Ok(port_v) = port_str.parse::<u16>() {
                    port = port_v;
                }
            }
        }
    }

    println!("HOSTNAME {}", hostname);
    println!("PORT {}", port);

    let is_streaming = env::args().any(|arg| arg == "--streaming");
    if is_streaming {
        let stream = get_streaming(hostname.clone(), port, password.clone()).await.expect("Missing stream");
        output.stream = Some(stream);
    }

    let is_recording = env::args().any(|arg| arg == "--recording");
    if is_recording {
        let recording = get_recording(hostname.clone(), port, password.clone()).await.expect("Missing recording");
        output.recording = Some(recording);
    }

    let serialized = serde_json::to_string(&output).unwrap();

    println!("OUTPUT: {}", serialized);
}

async fn get_client(hostname: String, port: u16, password: String) -> Result<obws::Client, anyhow::Error> {
    Ok(Client::connect(hostname, port, Some(password)).await?)
}

async fn get_streaming(hostname: String, port: u16, password: String) -> Result<StreamStatus> {
    let client = get_client(hostname, port, password).await?;

    // Get a list of available scenes and print them out.
    let stream_status = client.streaming().status().await?;

    Ok(stream_status)
}

async fn get_recording(hostname: String, port: u16, password: String) -> Result<RecordStatus> {
    let client = get_client(hostname, port, password).await?;

    // Get a list of available scenes and print them out.
    let recording_status = client.recording().status().await?;

    Ok(recording_status)
}