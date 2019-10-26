use clap::{App, Arg};
use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;
use std::sync::{Arc, Mutex};

mod events;
mod game_state;
use events::{Config, FoosInputPins};
use game_state::GameState;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let config = init()?;

    // Init Raspberry Pi pins
    let mut inputs = FoosInputPins::new(config)?;

    // Init thread-safe gamestate;
    let game_state: Arc<Mutex<GameState>> = Arc::new(Mutex::new(GameState::new()));

    // Previous game state used to debounce inputs
    let mut prev_game_state = GameState::new();

    // Init Raspberry Pi interrupts from config pins
    inputs.init_interrupts(&game_state)?;

    // Main game loop
    let timer_game_state: Arc<Mutex<GameState>> = Arc::clone(&game_state);
    let game_timer = std::thread::spawn(move || loop {
        std::thread::sleep(std::time::Duration::from_millis(1000));

        let mut ref_game_state = timer_game_state.lock().unwrap();
        if (*ref_game_state).ball_dropped == true {
            (*ref_game_state).reset();
        }

        (*ref_game_state).time += 1;

        debounce_inputs(&mut ref_game_state, &mut prev_game_state);

        println!("Timer: {:?}", ref_game_state);
    });

    // Basically a forever loop since game_timer thread won't end
    game_timer.join().unwrap();

    Ok(())
}

fn debounce_inputs(game_state: &mut GameState, prev_game_state: &mut GameState) {
    if game_state.red_goals > prev_game_state.red_goals {
        game_state.red_goals = prev_game_state.red_goals + 1;
        prev_game_state.red_goals = game_state.red_goals;
    }

    if game_state.blue_goals > prev_game_state.blue_goals {
        game_state.blue_goals = prev_game_state.blue_goals + 1;
        prev_game_state.blue_goals = game_state.blue_goals;
    }
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

fn init() -> Result<Config, Box<dyn std::error::Error>> {
    let matches = App::new("Hackathon Foosball | IQ Inc. ")
        .version("0.1.0")
        .author("Jeremy C. Zacharia <jzachariaiq-inc.com>")
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
