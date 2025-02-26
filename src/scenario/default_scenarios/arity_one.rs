use std::usize;

use super::Scenario;
use crate::item;
use crate::status;

pub fn get() -> Vec<Scenario> {
    vec![
        Scenario {
            possible_after: 0,
            impossible_after: 1,
            condition: |_, _| true,
            message: |players, indices| format!("{} found some bread.", players[indices[0]].name),
            actions: |players, indices| {
                players[indices[0]].inventory.push(&item::BREAD);
            },
        },
        Scenario {
            possible_after: 0,
            impossible_after: 1,
            condition: |_, _| true,
            message: |players, indices| {
                format!("{} found bottled water.", players[indices[0]].name)
            },
            actions: |players, indices| {
                players[indices[0]].inventory.push(&item::WATER);
            },
        },
        Scenario::nothing_burger(0, 1, |players, indices| {
            format!(
                "{} tried to grab supplies but was scared off.",
                players[indices[0]].name
            )
        }),
        Scenario {
            possible_after: 0,
            impossible_after: 1,
            condition: |_, _| true,
            message: |players, indices| {
                format!(
                    "{} clutched a first-aid kit and ran away.",
                    players[indices[0]].name
                )
            },
            actions: |players, indices| {
                players[indices[0]].inventory.push(&item::MEDKIT);
            },
        },
        Scenario {
            possible_after: 0,
            impossible_after: 1,
            condition: |_, _| true,
            message: |players, indices| {
                format!(
                    "{} took a sickle from inside the Cornucopia.",
                    players[indices[0]].name
                )
            },
            actions: |players, indices| {
                players[indices[0]].inventory.push(&item::SICKLE);
            },
        },
        Scenario {
            possible_after: 0,
            impossible_after: 1,
            condition: |_, _| true,
            message: |players, indices| {
                format!(
                    "{} found a bag full of explosives.",
                    players[indices[0]].name
                )
            },
            actions: |players, indices| {
                players[indices[0]].inventory.push(&item::EXPLOSIVES);
            },
        },
        Scenario {
            possible_after: 0,
            impossible_after: 1,
            condition: |_, _| true,
            message: |players, indices| {
                format!(
                    "{} grabbed a shield leaning on the Cornucopia.",
                    players[indices[0]].name
                )
            },
            actions: |players, indices| {
                players[indices[0]].inventory.push(&item::SHIELD);
            },
        },
        Scenario::nothing_burger(0, 1, |players, indices| {
            format!(
                "{} runs away with a lighter and some rope.",
                players[indices[0]].name
            )
        }),
        Scenario::nothing_burger(0, 1, |players, indices| {
            format!("{} sprinted into the woods.", players[indices[0]].name)
        }),
        Scenario::nothing_burger(0, 1, |players, indices| {
            format!("{} sprinted into the woods.", players[indices[0]].name)
        }),
        Scenario::nothing_burger(0, 1, |players, indices| {
            format!("{} sprinted into the woods.", players[indices[0]].name)
        }),
        Scenario::nothing_burger(1, 6, |players, indices| {
            format!(
                "{} questioned {} sanity.",
                players[indices[0]].name, players[indices[0]].pronouns.possessive_adj
            )
        }),
        Scenario::nothing_burger(1, 6, |players, indices| {
            format!(
                "{} lost one of {} shoes.",
                players[indices[0]].name, players[indices[0]].pronouns.possessive_adj
            )
        }),
        Scenario::nothing_burger(1, usize::MAX, |players, indices| {
            format!("{} almost fell off a bridge.", players[indices[0]].name)
        }),
        Scenario::nothing_burger(1, usize::MAX, |players, indices| {
            format!(
                "{} climbed a tree to get a better view of the arena.",
                players[indices[0]].name
            )
        }),
        Scenario {
            possible_after: 1,
            impossible_after: usize::MAX,
            condition: |players, indices| {
                if let status::Status::Alive(status::AliveStatus::Healthy) =
                    players[indices[0]].status
                {
                    return false;
                }

                players[indices[0]]
                    .get_item_kind(item::Kind::Healing)
                    .is_some()
            },
            message: |players, indices| {
                let item_index = players[indices[0]]
                    .get_item_kind(item::Kind::Healing)
                    .unwrap();

                format!(
                    "{} healed {} with {} {}.",
                    players[indices[0]].name,
                    players[indices[0]].pronouns.reflexive,
                    players[indices[0]].pronouns.possessive_adj,
                    players[indices[0]].inventory[item_index].name
                )
            },
            actions: |players, indices| {
                let item_index = players[indices[0]]
                    .get_item_kind(item::Kind::Healing)
                    .unwrap();
                players[indices[0]].inventory.remove(item_index);
                players[indices[0]].heal()
            },
        },
        Scenario {
            possible_after: 1,
            impossible_after: 6,
            condition: |players, indices| {
                if let status::Status::Alive(status::AliveStatus::Healthy) =
                    players[indices[0]].status
                {
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
            possible_after: 1,
            impossible_after: usize::MAX,
            condition: |players, indices| {
                players[indices[0]]
                    .get_item_kind(item::Kind::Weapon)
                    .is_some()
            },
            message: |players, indices| {
                let item_index = players[indices[0]]
                    .get_item_kind(item::Kind::Weapon)
                    .unwrap();
                format!(
                    "{} killed a deer with {} {}.",
                    players[indices[0]].name,
                    players[indices[0]].pronouns.possessive_adj,
                    players[indices[0]].inventory[item_index].name
                )
            },
            actions: |_, _| {},
        },
        Scenario {
            possible_after: 1,
            impossible_after: usize::MAX,
            condition: |_, _| true,
            message: |players, indices| {
                format!("{} was bitten by a snake.", players[indices[0]].name)
            },
            actions: |players, indices| {
                players[indices[0]].hurt();
            },
        },
        Scenario {
            possible_after: 1,
            impossible_after: usize::MAX,
            condition: |_, _| true,
            message: |players, indices| {
                format!(
                    "{} made a knife out of flint and a stick.",
                    players[indices[0]].name
                )
            },
            actions: |players, indices| {
                players[indices[0]].inventory.push(&item::KNIFE);
            },
        },
    ]
}
