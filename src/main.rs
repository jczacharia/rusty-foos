use clap::{App, Arg};
use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;
use std::sync::{Arc, Mutex};
use ws::{Builder, Message, Sender};

mod events;
mod game_state;

use events::{Config, FoosInputPins};
use game_state::GameState;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let config = init()?;

    // Init Raspberry Pi pins
    let mut inputs = FoosInputPins::new(&config)?;

    // Init thread-safe gamestate
    let game_state: Arc<Mutex<GameState>> = Arc::new(Mutex::new(GameState::new()));

    // Init Raspberry Pi interrupts from config pins
    inputs.init_interrupts(&game_state)?;

    let socket = Builder::new()
        .build(move |_| {
            // Dummy message handler
            move |_| {
                println!("Message handler called.");
                Ok(())
            }
        })
        .unwrap();

    let handle = socket.broadcaster();

    // Start listening on another thread
    std::thread::spawn(move || {
        socket.listen("192.168.1.152:3000").unwrap();
    });

    // Start main game loop
    game_loop(&handle, &game_state).join().unwrap();

    Ok(())
}

fn game_loop(
    handler: &Sender,
    arc_game_state: &Arc<Mutex<GameState>>,
) -> std::thread::JoinHandle<()> {
    let mut prev_game_state = GameState::new();
    let timer_game_state: Arc<Mutex<GameState>> = Arc::clone(arc_game_state);
    loop {
        std::thread::sleep(std::time::Duration::from_millis(1000));
        let mut game_state = timer_game_state.lock().unwrap();

        (*game_state).time += 1;

        // Debounce
        if game_state.red_goals > prev_game_state.red_goals {
            game_state.red_goals = prev_game_state.red_goals + 1;
            prev_game_state.red_goals = game_state.red_goals;
        }
        if game_state.blue_goals > prev_game_state.blue_goals {
            game_state.blue_goals = prev_game_state.blue_goals + 1;
            prev_game_state.blue_goals = game_state.blue_goals;
        }

        let json = serde_json::json!((*game_state));

        let msg = Message::text(json.to_string());

        handler.send(msg).unwrap();

        println!("Timer: {:?}", game_state);
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
