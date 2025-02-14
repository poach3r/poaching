use rand::rngs::ThreadRng;
use rand::seq::IndexedRandom;

use crate::player::pronouns::*;
use crate::player::status::*;
use crate::player::*;
use crate::scenario::*;
use rand::seq::SliceRandom;

/// Simulates an entire game with predefined scenarios
/// and players. This will be made much more modular
/// in the future but this is more useful for
/// rapid prototyping.
pub fn simulate_game<'a>() -> Vec<Player<'a>> {
    let mut rng = rand::rng();
    let scenarios = default_scenarios();
    let mut players: Vec<Player> = vec![
        Player::new("Bob", MALE),
        Player::new("Susan", FEMALE),
        Player::new("Mike", MALE),
        Player::new("Jill", FEMALE),
        Player::new("Jack", MALE),
        Player::new("Chloe", FEMALE),
        Player::new("James", MALE),
        Player::new("Mary", FEMALE),
        Player::new("Robert", MALE),
        Player::new("Jennifer", FEMALE),
        Player::new("Sam", ENBY),
        Player::new("Elliot", ENBY),
        Player::new("Anthony", MALE),
        Player::new("Emily", FEMALE),
    ];

    let mut current_day = 1u16;

    // game start
    println!("Day 1:");
    let start_scenarios = game_start_scenarios();
    simulate_round(&mut players, &start_scenarios, &mut rng);

    // progress rounds
    while get_available_players(&players, 0).len() > 1 {
        current_day += 1;
        println!("Day {}:", current_day);
        players.shuffle(&mut rng);
        simulate_round(&mut players, &scenarios, &mut rng);
    }

    match get_available_players(&players, 0).first() {
        Some(winner) => println!("Winner: {}", players[*winner].name),
        _ => println!("Nobody won."),
    };

    players
}

/// Simulates a single round.
fn simulate_round<'a, 'b>(
    players: &mut Vec<Player<'a>>,
    scenarios: &Vec<Vec<Scenario>>,
    rng: &'b mut ThreadRng,
) {
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

        scenario.run(players, &indices);
        index += 1;
    }

    for player in players {
        player.moved = false;

        if crate::DEBUGGING_ENABLED {
            println!("{}", player.to_string());
        }
    }
    println!();
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
