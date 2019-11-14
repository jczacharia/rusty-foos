use clap::{App, Arg};
use serde::{Deserialize, Serialize};
use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;
use std::sync::mpsc;
use std::sync::mpsc::{Receiver, Sender};

mod events;
mod game;

use events::FoosInputPins;
use game::{FoosEvent, FoosStateMachine};

/// Foosball Game Configuration
#[derive(Serialize, Deserialize, Clone)]
pub struct Config {
    blue_goal_pin: u8,
    red_goal_pin: u8,
    ball_drop_1_pin: u8,
    ball_drop_2_pin: u8,
    reset_pin: u8,
    ip_addr: String,
    port: u32,
    max_score: u32,
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let config = init()?;

    // Init Raspberry Pi pins
    let mut inputs = FoosInputPins::new(&config)?;

    // Init thread-safe gamestate
    let (tx, rx): (Sender<FoosEvent>, Receiver<FoosEvent>) = mpsc::channel();

    // Init Raspberry Pi interrupts from config pins
    inputs.init_interrupts(&tx)?;

    let socket = ws::Builder::new().build(move |_| {
        // Dummy message handler
        move |_| {
            println!("Message handler called.");
            Ok(())
        }
    })?;

    // Used to send game data to all connected sockets
    let all_sockets_broadcaster = socket.broadcaster();
    
    // clone config
    let cc = config.clone();

    // Start listening on another thread
    std::thread::spawn(move || {
        let ip_port = format!("{}:{}", config.ip_addr.clone(), &config.port);
        socket.listen(ip_port).unwrap();
    });

    // Start main game loop
    let mut game = FoosStateMachine::new(cc.max_score);
    loop {
        std::thread::sleep(std::time::Duration::from_millis(1000));

        match rx.try_recv() {
            Ok(e) => game.next(e),
            _ => game.next(FoosEvent::NoEvent),
        };

        let gd = game.get_game_data();

        let json = serde_json::json!(game.get_game_data());

        let msg = ws::Message::text(json.to_string());

        all_sockets_broadcaster.send(msg).unwrap();

        println!("Timer: {:?}", gd);
    }
}

fn init() -> Result<Config, Box<dyn std::error::Error>> {
    let matches = App::new("Rusty Foosball")
        .version("0.1.0")
        .author("Jeremy C. Zacharia <jczacharia@gmail.com>")
        .about("Automated Foosball game server")
        .arg(
            Arg::with_name("config")
                .short("c")
                .long("config")
                .value_name("FILE")
                .help("Sets the config file")
                .takes_value(true),
        )
        .get_matches();

    // Gets a value for config if supplied by user, or defaults to "config.json"
    let config_file_name = matches.value_of("config").unwrap_or("config.json");
    let config_string = parse_config_file(config_file_name);
    let config: Config = serde_json::from_str(&config_string)?;

    Ok(config)
}

fn parse_config_file(filepath: &str) -> String {
    let file = File::open(filepath).expect("could not open file");
    let mut buffered_reader = BufReader::new(file);
    let mut contents = String::new();
    let _number_of_bytes: usize = match buffered_reader.read_to_string(&mut contents) {
        Ok(number_of_bytes) => number_of_bytes,
        Err(_err) => 0,
    };

    contents
}
