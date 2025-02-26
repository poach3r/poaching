mod gui;
mod player;
mod scenario;
mod simulator;

use std::{
    fs::File,
    io::{BufReader, Read},
    path::PathBuf,
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

    let scenarios = scenario::default_scenarios::get();
    let mut players = if let Some(players) = load_players_filename(args.players) {
        players
    } else {
        Vec::new()
    };

    if args.text_mode {
        simulator::simulate_game(&mut players, &scenarios);
    } else {
        gui::run(players, scenarios, args.gtk_options);
    }
}

fn load_players_file<'a>(file: File) -> Option<Vec<Player<'a>>> {
    let mut reader = BufReader::new(&file);
    let mut s = String::new();

    match reader.read_to_string(&mut s) {
        Ok(_) => {
            info!("Successfully read {file:#?}.");
        }
        Err(e) => {
            error!("Failed to read {file:#?}: {e}");
            return None;
        }
    }

    let v: Value = match serde_json::from_str(s.as_str()) {
        Ok(v) => {
            info!("Successfully loaded {file:#?} as JSON.");
            v
        }
        Err(e) => {
            error!("Failed to load {file:#?} as JSON: {e}");
            return None;
        }
    };

    let players = match Vec::<JsonPlayer>::deserialize(v) {
        Ok(p) => {
            info!("Successfully deserialized {file:#?}.");
            p
        }
        Err(e) => {
            error!("Failed to deserialize {file:#?}: {e}");
            return None;
        }
    };

    Some(players.iter().map(|player| Player::from(player)).collect())
}

fn load_players_path<'a>(path: PathBuf) -> Option<Vec<Player<'a>>> {
    let file = match File::open(&path) {
        Ok(f) => {
            info!("Successfully opened {path:#?}.");
            f
        }
        Err(e) => {
            error!("Failed to open {path:#?}: {e}");
            return None;
        }
    };

    load_players_file(file)
}

fn load_players_filename<'a>(filename: String) -> Option<Vec<Player<'a>>> {
    let file = match File::open(&filename) {
        Ok(f) => {
            info!("Successfully opened {filename}.");
            f
        }
        Err(e) => {
            error!("Failed to open {filename}: {e}");
            return None;
        }
    };

    load_players_file(file)
}
