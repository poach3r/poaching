use crate::player::status::*;
use crate::player::*;

/// A scenario that occures with `arity` players.
#[derive(Debug)]
pub struct Scenario {
    pub condition: fn(players: &Vec<Player>, indices: &Vec<usize>) -> bool,
    pub get_message: fn(players: &Vec<Player>, indices: &Vec<usize>) -> String,
    pub actions: Vec<fn(player: &mut Player)>,
}

impl Scenario {
    /// Prints the result of `get_message` and
    /// performs an action on every player involved.
    pub fn run(&self, players: &mut Vec<Player>, indices: &Vec<usize>) {
        println!("{}", (self.get_message)(players, indices));
        for (index, action) in self.actions.iter().enumerate() {
            players[indices[index]].moved = true;
            action(&mut players[indices[index]]);
        }
    }

    fn nothing_burger(
        get_message: fn(players: &Vec<Player>, indices: &Vec<usize>) -> String,
    ) -> Scenario {
        Scenario {
            condition: |_, _| true,
            get_message,
            actions: vec![|_| {}],
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
                format!("{} ran away.", players[indices[0]].name)
            }),
        ],
        vec![
            Scenario {
                condition: |_, _| true,
                get_message: |players, indices| {
                    format!(
                        "{} got into a fistfight with {}.",
                        players[indices[0]].name, players[indices[1]].name
                    )
                },
                actions: vec![
                    |player| {
                        player.hurt();
                    },
                    |player| {
                        player.hurt();
                    },
                ],
            },
            Scenario {
                condition: |_, _| true,
                get_message: |players, indices| {
                    format!(
                        "{} curbstomped {}.",
                        players[indices[0]].name, players[indices[1]].name
                    )
                },
                actions: vec![|_| {}, |player| {
                    player.kill();
                }],
            },
            Scenario {
                condition: |_, _| true,
                get_message: |players, indices| {
                    format!(
                        "{} stabbed {} and couldn't stop {} from laughing.",
                        players[indices[0]].name,
                        players[indices[1]].name,
                        players[indices[0]].pronouns.reflexive
                    )
                },
                actions: vec![|_| {}, |player| {
                    player.kill();
                }],
            },
        ],
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
                get_message: |players, indices| {
                    format!(
                        "{} tried and failed to kill a deer.",
                        players[indices[0]].name
                    )
                },
                actions: vec![|_| {}],
            },
            Scenario {
                condition: |_, _| true,
                get_message: |players, indices| {
                    format!("{} was bitten by a snake.", players[indices[0]].name)
                },
                actions: vec![|player| {
                    player.hurt();
                }],
            },
        ],
        vec![
            Scenario {
                condition: |_, _| true,
                get_message: |players, indices| {
                    format!(
                        "{} caught {} off guard and killed {}.",
                        players[indices[0]].name,
                        players[indices[1]].name,
                        players[indices[1]].pronouns.object
                    )
                },
                actions: vec![|_| {}, |player| {
                    player.kill();
                }],
            },
            Scenario {
                condition: |_, _| true,
                get_message: |players, indices| {
                    format!(
                        "{} got into a fistfight with {}.",
                        players[indices[0]].name, players[indices[1]].name
                    )
                },
                actions: vec![
                    |player| {
                        player.hurt();
                    },
                    |player| {
                        player.hurt();
                    },
                ],
            },
            Scenario {
                condition: |players, indices| {
                    if let Status::Alive(AliveStatus::Injured) = players[indices[1]].status {
                        true
                    } else {
                        false
                    }
                },
                get_message: |players, indices| {
                    format!(
                        "{} tended to {}'s wounds.",
                        players[indices[0]].name, players[indices[1]].name
                    )
                },
                actions: vec![|_| {}, |player| {
                    player.heal();
                }],
            },
            Scenario {
                condition: |_, _| true,
                get_message: |players, indices| {
                    format!(
                        "{} and {} poisoned eachother.",
                        players[indices[0]].name, players[indices[1]].name
                    )
                },
                actions: vec![
                    |player| {
                        player.kill();
                    },
                    |player| {
                        player.kill();
                    },
                ],
            },
            Scenario {
                condition: |_, _| true,
                get_message: |players, indices| {
                    format!(
                        "{} hit {} with a throwing knife.",
                        players[indices[0]].name, players[indices[1]].name
                    )
                },
                actions: vec![|_| {}, |player| {
                    player.kill();
                }],
            },
        ],
        vec![
            Scenario {
                condition: |_, _| true,
                get_message: |players, indices| {
                    format!(
                        "{}, {} and {} discussed their plans for tomorrow.",
                        players[indices[0]].name,
                        players[indices[1]].name,
                        players[indices[2]].name
                    )
                },
                actions: vec![|_| {}, |_| {}, |_| {}],
            },
            Scenario {
                condition: |_, _| true,
                get_message: |players, indices| {
                    format!(
                        "{} and {} were hunting {} together until {} betrayed {}.",
                        players[indices[0]].name,
                        players[indices[1]].name,
                        players[indices[2]].name,
                        players[indices[1]].name,
                        players[indices[0]].pronouns.object
                    )
                },
                actions: vec![|player| player.kill(), |_| {}, |_| {}],
            },
            Scenario {
                condition: |_, _| true,
                get_message: |players, indices| {
                    format!(
                        "{}, {} and {} were crushed by a falling tree at night.",
                        players[indices[0]].name,
                        players[indices[1]].name,
                        players[indices[2]].name
                    )
                },
                actions: vec![
                    |player| {
                        player.kill();
                    },
                    |player| {
                        player.kill();
                    },
                    |player| {
                        player.kill();
                    },
                ],
            },
        ],
        vec![Scenario {
            condition: |_, _| true,
            get_message: |players, indices| {
                format!(
                    "{}, {}, {} and {} sang songs around a campfire.",
                    players[indices[0]].name,
                    players[indices[1]].name,
                    players[indices[2]].name,
                    players[indices[3]].name
                )
            },
            actions: vec![|_| {}, |_| {}, |_| {}, |_| {}],
        }],
    ]
}
