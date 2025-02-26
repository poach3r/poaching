use super::Scenario;
use crate::item;
use crate::status;

pub fn get() -> Vec<Scenario> {
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
            condition: |_, _| true,
            message: |players, indices| {
                format!("{} was bitten by a snake.", players[indices[0]].name)
            },
            actions: |players, indices| {
                players[indices[0]].hurt();
            },
        },
        Scenario {
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
