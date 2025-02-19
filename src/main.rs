mod gui;
mod player;
mod scenario;
mod simulator;

use std::{
    fs::File,
    io::{BufReader, Read},
};

use clap::Parser;
use log::{error, info};
use player::*;
use serde::Deserialize;
use serde_json::Value;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    #[arg(short, long, default_value_t = String::from("players.json"))]
    players: String,

    #[arg(short, long, default_value_t = false)]
    text_mode: bool,

    #[arg(allow_hyphen_values = true, trailing_var_arg = true)]
    gtk_options: Vec<String>,
}

fn main() {
    let args = Args::parse();
    env_logger::init();

    let scenarios = scenario::default_scenarios();
    let start_scenarios = scenario::game_start_scenarios();
    let mut players = load_players_from_file(args.players);

    if args.text_mode {
        simulator::simulate_game(&mut players, &scenarios, &start_scenarios);
    } else {
        gui::run(players, scenarios, start_scenarios, args.gtk_options);
    }
}

fn load_players_from_file<'a>(filename: String) -> Vec<Player<'a>> {
    let file = match File::open(&filename) {
        Ok(f) => {
            info!("Successfully opened {filename}.");
            f
        }
        Err(e) => {
            error!("Failed to open {filename}: {e}");
            panic!("Failed to open {filename}: {e}");
        }
    };

    let mut reader = BufReader::new(file);
    let mut s = String::new();

    match reader.read_to_string(&mut s) {
        Ok(_) => {
            info!("Successfully read {filename}.");
        }
        Err(e) => {
            error!("Failed to read {filename}: {e}");
            panic!("Failed to read {filename}: {e}");
        }
    }

    let v: Value = match serde_json::from_str(s.as_str()) {
        Ok(v) => {
            info!("Successfully loaded {filename} as JSON.");
            v
        }
        Err(e) => {
            error!("Failed to load {filename} as JSON: {e}");
            panic!("Failed to load {filename} as JSON: {e}");
        }
    };

    let players = match Vec::<JsonPlayer>::deserialize(v) {
        Ok(p) => {
            info!("Successfully deserialized {filename}.");
            p
        }
        Err(e) => {
            error!("Failed to deserialize {filename}: {e}");
            panic!("Failed to deserialize {filename}: {e}");
        }
    };

    players.iter().map(|player| Player::from(player)).collect()
}
