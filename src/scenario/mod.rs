pub mod default_scenarios;

use crate::player::*;

#[derive(Debug)]
pub struct Scenario {
    pub possible_after: usize,
    pub impossible_after: usize,
    pub condition: fn(players: &Vec<Player>, indices: &Vec<usize>) -> bool,
    pub message: fn(players: &Vec<Player>, indices: &Vec<usize>) -> String,
    pub actions: fn(players: &mut Vec<Player>, indices: &Vec<usize>),
}

impl Scenario {
    /// Prints the result of `get_message` and
    /// performs an action on every player involved.
    pub fn run(&self, players: &mut Vec<Player>, indices: &Vec<usize>, arity: usize) -> String {
        for index in 0..arity + 1 {
            players[indices[index]].moved = true
        }

        (self.actions)(players, indices);
        (self.message)(players, indices)
    }

    fn nothing_burger(
        possible_after: usize,
        impossible_after: usize,
        get_message: fn(players: &Vec<Player>, indices: &Vec<usize>) -> String,
    ) -> Scenario {
        Scenario {
            possible_after,
            impossible_after,
            condition: |_, _| true,
            message: get_message,
            actions: |_, _| {},
        }
    }
}
