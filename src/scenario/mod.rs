use crate::player::status::*;
use crate::player::*;

#[derive(Debug)]
pub struct Scenario {
    pub condition: fn(players: &Vec<Player>, indices: &Vec<usize>) -> bool,
    pub message: fn(players: &Vec<Player>, indices: &Vec<usize>) -> String,
    pub actions: fn(players: &mut Vec<Player>, indices: &Vec<usize>),
}

impl Scenario {
    /// Prints the result of `get_message` and
    /// performs an action on every player involved.
    pub fn run(&self, players: &mut Vec<Player>, indices: &Vec<usize>, arity: usize) -> String {
        for index in 0..arity {
            players[indices[index]].moved = true
        }

        (self.actions)(players, indices);
        (self.message)(players, indices)
    }

    fn nothing_burger(
        get_message: fn(players: &Vec<Player>, indices: &Vec<usize>) -> String,
    ) -> Scenario {
        Scenario {
            condition: |_, _| true,
            message: get_message,
            actions: |_, _| {},
        }
    }
}

/// Returns a vector of vectors of scenarios
/// that are to be used at the start of the game.
/// The element of each vector is equivalent
/// to the arity of it's scenarios.
pub fn game_start_scenarios() -> Vec<Vec<Scenario>> {
    vec![
        vec![
            Scenario::nothing_burger(|players, indices| {
                format!("{} found some food.", players[indices[0]].name)
            }),
            Scenario::nothing_burger(|players, indices| {
                format!("{} found bottled water.", players[indices[0]].name)
            }),
            Scenario::nothing_burger(|players, indices| {
                format!("{} sprinted into the woods.", players[indices[0]].name)
            }),
            Scenario::nothing_burger(|players, indices| {
                format!(
                    "{} tried to grab supplies but was scared off.",
                    players[indices[0]].name
                )
            }),
            Scenario::nothing_burger(|players, indices| {
                format!(
                    "{} clutched a first-aid kit and ran away.",
                    players[indices[0]].name
                )
            }),
            Scenario::nothing_burger(|players, indices| {
                format!(
                    "{} takes a sickle from inside the Cornucopia.",
                    players[indices[0]].name
                )
            }),
            Scenario::nothing_burger(|players, indices| {
                format!(
                    "{} finds a bag full of explosives",
                    players[indices[0]].name
                )
            }),
            Scenario::nothing_burger(|players, indices| {
                format!(
                    "{} grabs a shield leaning on the Cornucopia.",
                    players[indices[0]].name
                )
            }),
            Scenario::nothing_burger(|players, indices| {
                format!(
                    "{} runs away with a lighter and some rope.",
                    players[indices[0]].name
                )
            }),
        ],
        vec![
            Scenario {
                condition: |_, _| true,
                message: |players, indices| {
                    format!(
                        "{} fought {} for a bag but lost and ran away.",
                        players[indices[0]].name, players[indices[1]].name
                    )
                },
                actions: |players, indices| {
                    players[indices[0]].hurt();
                },
            },
            Scenario {
                condition: |_, _| true,
                message: |players, indices| {
                    format!(
                        "{} scared {} away from the Cornucopia.",
                        players[indices[0]].name, players[indices[1]].name
                    )
                },
                actions: |_, _| {},
            },
            Scenario {
                condition: |_, _| true,
                message: |players, indices| {
                    format!(
                        "{} broke {}'s nose for a basket of bread.",
                        players[indices[0]].name, players[indices[1]].name
                    )
                },
                actions: |players, indices| {
                    players[indices[1]].hurt();
                },
            },
            Scenario {
                condition: |_, _| true,
                message: |players, indices| {
                    format!(
                        "{} got into a fistfight with {}.",
                        players[indices[0]].name, players[indices[1]].name
                    )
                },
                actions: |players, indices| {
                    players[indices[0]].hurt();
                    players[indices[1]].hurt();
                },
            },
            Scenario {
                condition: |_, _| true,
                message: |players, indices| {
                    format!(
                        "{} curbstomped {}.",
                        players[indices[0]].name, players[indices[1]].name
                    )
                },
                actions: |players, indices| {
                    players[indices[1]].kill();
                },
            },
        ],
        vec![Scenario {
            condition: |_, _| true,
            message: |players, indices| {
                format!(
                    "I don't feel like writing what {}, {} and {} did because im lazy.",
                    players[indices[0]].name, players[indices[1]].name, players[indices[2]].name
                )
            },
            actions: |_, _| {},
        }],
        vec![Scenario {
            condition: |_, _| true,
            message: |players, indices| {
                format!(
                    "{} and {} fought {} and {}. {} and {} survived.",
                    players[indices[0]].name,
                    players[indices[1]].name,
                    players[indices[2]].name,
                    players[indices[3]].name,
                    players[indices[0]].name,
                    players[indices[1]].name,
                )
            },
            actions: |players, indices| {
                players[indices[0]].hurt();
                players[indices[1]].hurt();
                players[indices[3]].kill();
                players[indices[4]].kill();
            },
        }],
    ]
}

/// Returns a vector of vectors of scenarios.
/// The element of each vector is equivalent
/// to the arity of it's scenarios.
pub fn default_scenarios() -> Vec<Vec<Scenario>> {
    vec![
        vec![
            Scenario::nothing_burger(|players, indices| {
                format!(
                    "{} questioned {} sanity.",
                    players[indices[0]].name, players[indices[0]].pronouns.possessive_adj
                )
            }),
            Scenario::nothing_burger(|players, indices| {
                format!(
                    "{} lost one of {} shoes.",
                    players[indices[0]].name, players[indices[0]].pronouns.possessive_adj
                )
            }),
            Scenario::nothing_burger(|players, indices| {
                format!("{} almost fell off a bridge.", players[indices[0]].name)
            }),
            Scenario::nothing_burger(|players, indices| {
                format!("{} made a shelter near a cave.", players[indices[0]].name)
            }),
            Scenario::nothing_burger(|players, indices| {
                format!("{} found a dead body.", players[indices[0]].name)
            }),
            Scenario::nothing_burger(|players, indices| {
                format!(
                    "{} climbed a tree to get a better view of the arena.",
                    players[indices[0]].name
                )
            }),
            Scenario {
                condition: |players, indices| {
                    if let Status::Alive(AliveStatus::Healthy) = players[indices[0]].status {
                        true
                    } else {
                        false
                    }
                },
                message: |players, indices| {
                    format!(
                        "{} tried and failed to kill a deer.",
                        players[indices[0]].name
                    )
                },
                actions: |_, _| {},
            },
            Scenario {
                condition: |_, _| true,
                message: |players, indices| {
                    format!("{} was bitten by a snake.", players[indices[0]].name)
                },
                actions: |players, indices| {
                    players[indices[0]].hurt();
                },
            },
        ],
        vec![
            Scenario {
                condition: |_, _| true,
                message: |players, indices| {
                    format!(
                        "{} caught {} off guard and killed {}.",
                        players[indices[0]].name,
                        players[indices[1]].name,
                        players[indices[1]].pronouns.object
                    )
                },
                actions: |players, indices| {
                    players[indices[1]].kill();
                },
            },
            Scenario {
                condition: |_, _| true,
                message: |players, indices| {
                    format!(
                        "{} got into a fistfight with {}.",
                        players[indices[0]].name, players[indices[1]].name
                    )
                },
                actions: |players, indices| {
                    players[indices[0]].hurt();
                    players[indices[1]].hurt();
                },
            },
            Scenario {
                condition: |players, indices| {
                    if let Status::Alive(AliveStatus::Injured) = players[indices[1]].status {
                        true
                    } else {
                        false
                    }
                },
                message: |players, indices| {
                    format!(
                        "{} tended to {}'s wounds.",
                        players[indices[0]].name, players[indices[1]].name
                    )
                },
                actions: |players, indices| {
                    players[indices[1]].heal();
                },
            },
            Scenario {
                condition: |_, _| true,
                message: |players, indices| {
                    format!(
                        "{} and {} poisoned eachother.",
                        players[indices[0]].name, players[indices[1]].name
                    )
                },
                actions: |players, indices| {
                    players[indices[0]].kill();
                    players[indices[1]].kill();
                },
            },
            Scenario {
                condition: |_, _| true,
                message: |players, indices| {
                    format!(
                        "{} hit {} with a throwing knife.",
                        players[indices[0]].name, players[indices[1]].name
                    )
                },
                actions: |players, indices| {
                    players[indices[1]].kill();
                },
            },
        ],
        vec![
            Scenario {
                condition: |_, _| true,
                message: |players, indices| {
                    format!(
                        "{}, {} and {} discussed their plans for tomorrow.",
                        players[indices[0]].name,
                        players[indices[1]].name,
                        players[indices[2]].name
                    )
                },
                actions: |_, _| {},
            },
            Scenario {
                condition: |_, _| true,
                message: |players, indices| {
                    format!(
                        "{} and {} were hunting {} together until {} betrayed {}.",
                        players[indices[0]].name,
                        players[indices[1]].name,
                        players[indices[2]].name,
                        players[indices[1]].name,
                        players[indices[0]].pronouns.object
                    )
                },
                actions: |players, indices| {
                    players[indices[0]].kill();
                },
            },
            Scenario {
                condition: |_, _| true,
                message: |players, indices| {
                    format!(
                        "{}, {} and {} were crushed by a falling tree at night.",
                        players[indices[0]].name,
                        players[indices[1]].name,
                        players[indices[2]].name
                    )
                },
                actions: |players, indices| {
                    players[indices[0]].kill();
                    players[indices[1]].kill();
                    players[indices[2]].kill();
                },
            },
        ],
        vec![Scenario {
            condition: |_, _| true,
            message: |players, indices| {
                format!(
                    "{}, {}, {} and {} sang songs around a campfire.",
                    players[indices[0]].name,
                    players[indices[1]].name,
                    players[indices[2]].name,
                    players[indices[3]].name
                )
            },
            actions: |_, _| {},
        }],
    ]
}
