use clap::{Arg, Command};
use notify::RecursiveMode;
use notify_debouncer_mini::new_debouncer;
use reqwest::Client;
use std::collections::HashMap;
use std::io::BufRead;
use std::path::Path;
use std::sync::mpsc::channel;
use std::time::Duration;

use better_reporter_agent::{chat, connection, general};

#[tokio::main]
async fn main() {
    let matches = Command::new("Better Reporter Agent")
        .version("0.3.0")
        .author("Oqch")
        .about("Send notifications to Discord")
        .arg(
            Arg::new("webhook_url")
                .required(true)
                .index(1)
                .help("A webhook url of Discord"),
        )
        .get_matches();

    let url: &String = matches.get_one("webhook_url").expect("bad webhook url");
    let client = reqwest::Client::new();

    let (tx, rx) = channel();

    let files = [
        Path::new("./factorio/script-output/better-reporter/general.txt"),
        Path::new("./factorio/script-output/better-reporter/connection.txt"),
        Path::new("./factorio/script-output/better-reporter/chat.txt"),
        // Path::new("./general.txt"),
        // Path::new("./connection.txt"),
    ];

    let mut debouncer = new_debouncer(Duration::from_secs(2), tx).unwrap();

    debouncer
        .watcher()
        .watch(files[0], RecursiveMode::Recursive)
        .unwrap();
    debouncer
        .watcher()
        .watch(files[1], RecursiveMode::Recursive)
        .unwrap();
    debouncer
        .watcher()
        .watch(files[2], RecursiveMode::Recursive)
        .unwrap();

    loop {
        match rx.recv() {
            Err(e) => eprintln!("watch error: {}", e),
            Ok(event) => match event {
                Ok(events) => {
                    let pathbuf = &events[0].path;
                    let reader = std::io::BufReader::new(std::fs::File::open(pathbuf).unwrap());
                    if let Some(text) = reader.lines().last() {
                        match text {
                            Ok(text) => {
                                if pathbuf.file_name().unwrap() == "chat.txt" {
                                    let parsed = serde_json::from_str::<chat::Message>(&text);
                                    match parsed {
                                        Ok(message) => {
                                            let t = match message {
                                                chat::Message::Chat(e) => {
                                                    if let Some(name) = e.player_name {
                                                        format!("{}: {}", name, e.text)
                                                    } else {
                                                        format!(": {}", e.text)
                                                    }
                                                }
                                            };
                                            let res = send(&client, url, &t).await;
                                            match res.await {
                                                Ok(res) => println!("{:?}", res),
                                                Err(e) => eprintln!("{}", e),
                                            }
                                        }
                                        Err(_e) => {
                                            eprintln!("coudn't parse a console event: {}", &text)
                                        }
                                    }
                                } else if pathbuf.file_name().unwrap() == "general.txt" {
                                    let parsed = serde_json::from_str::<general::Message>(&text);
                                    match parsed {
                                        Ok(message) => {
                                            let t = match message {
                                                general::Message::PlayerDied(e) => {
                                                    format!("{}が登遐されました。", e.player_name)
                                                }
                                            };
                                            let res = send(&client, url, &t).await;
                                            match res.await {
                                                Ok(res) => println!("{:?}", res),
                                                Err(e) => eprintln!("{}", e),
                                            }
                                        }
                                        Err(_e) => {
                                            eprintln!("coudn't parse a general event: {}", &text)
                                        }
                                    }
                                } else {
                                    let parsed = serde_json::from_str::<connection::Message>(&text);
                                    match parsed {
                                        Ok(message) => {
                                            let t = match message {
                                                connection::Message::Left(e) => e.to_string(),
                                                connection::Message::Join(e) => e.to_string(),
                                            };
                                            let res = send(&client, url, &t).await;
                                            match res.await {
                                                Ok(res) => println!("{:?}", res),
                                                Err(e) => eprintln!("{}", e),
                                            }
                                        }
                                        Err(_e) => {
                                            eprintln!("coudn't parse a connection event: {}", &text)
                                        }
                                    }
                                }
                            }
                            Err(e) => eprintln!("{}", e),
                        }
                    }
                }
                _ => println!("ignore event: {:?}", event),
            },
        }
    }
}

async fn send(
    client: &Client,
    url: &str,
    text: &str,
) -> impl std::future::Future<Output = Result<reqwest::Response, reqwest::Error>> {
    let mut json = HashMap::new();
    let req_builder = client.post(url);
    json.insert("content", text);
    req_builder.json(&json).send()
}
