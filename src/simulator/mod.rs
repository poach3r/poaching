use rand::rngs::ThreadRng;
use rand::seq::IndexedRandom;

use crate::player::status::*;
use crate::player::*;
use crate::scenario::*;
use log::info;
use rand::seq::SliceRandom;

/// Simulates an entire game with predefined scenarios
/// and players. This will be made much more modular
/// in the future but this is more useful for
/// rapid prototyping.
pub fn simulate_game<'a>(
    players: &mut Vec<Player>,
    scenarios: &Vec<Vec<Scenario>>,
    start_scenarios: &Vec<Vec<Scenario>>,
) {
    let mut rng = rand::rng();
    let mut current_day = 1u16;

    // game start
    println!("Day 1:");
    simulate_round(players, &start_scenarios, &mut rng);
    println!();

    // progress rounds
    while get_available_players(&players, 0).len() > 1 {
        current_day += 1;
        println!("Day {}:", current_day);
        players.shuffle(&mut rng);
        simulate_round(players, scenarios, &mut rng);
        println!();
    }

    match get_available_players(players, 0).first() {
        Some(winner) => println!("Winner: {}", players[*winner].name),
        _ => println!("Nobody won."),
    };

    info!("Simulation ended with following result:\n{players:#?}");
}

/// Simulates a single round.
pub fn simulate_round<'a, 'b>(
    players: &mut Vec<Player<'a>>,
    scenarios: &Vec<Vec<Scenario>>,
    rng: &'b mut ThreadRng,
) -> String {
    let mut string = String::new();
    let mut index: usize = 0;
    while index < players.len() {
        if let Status::Dead = players[index].status {
            index += 1;
            continue;
        }
        if players[index].moved {
            index += 1;
            continue;
        }

        let indices = get_available_players(players, index);
        let scenario = get_scenario(scenarios, rng, players, &indices);

        string.push_str(scenario.run(players, &indices).as_str());
        string.push('\n');
        index += 1;
    }

    for player in players {
        player.moved = false;
    }

    string.pop(); // remove extra newline
    string
}

/// Returns the indices of all players capable of moving.
fn get_available_players(players: &Vec<Player>, index: usize) -> Vec<usize> {
    let mut indices = Vec::<usize>::new();
    let mut n = index;
    while n < players.len() {
        if let Status::Dead = players[n].status {
            n += 1;
            continue;
        }
        if players[n].moved {
            n += 1;
            continue;
        }

        indices.push(n);
        n += 1;
    }

    indices
}

/// Gets a random scenario from `scenarios` based on the provided `players` and `indices`.
fn get_scenario<'a, 'b>(
    scenarios: &'a Vec<Vec<Scenario>>,
    rng: &'b mut ThreadRng,
    players: &Vec<Player>,
    indices: &Vec<usize>,
) -> &'a Scenario {
    loop {
        let mut random_scenes: Vec<&Scenario> = Vec::new();

        for i in 0..scenarios.len() {
            if i >= indices.len() {
                continue;
            }

            let possible_scene = match scenarios[i].choose(rng) {
                Some(s) => s,
                None => {
                    panic!(
                        "{}",
                        "Failed to choose a scenario. Please send this full message to poacher.
                         rng: {rng:?}
                         scenarios: {scenarios:?}"
                            .trim()
                    );
                }
            };

            if (possible_scene.condition)(players, indices) {
                random_scenes.push(possible_scene);
            }
        }

        match random_scenes.choose(rng) {
            Some(s) => return *s,
            None => continue,
        }
    }
}
