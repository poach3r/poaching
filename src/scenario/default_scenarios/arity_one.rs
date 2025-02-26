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

                for item in players[indices[0]].inventory.iter() {
                    if let item::Kind::Healing = item.kind {
                        return true;
                    }
                }

                false
            },
            message: |players, indices| {
                let mut item_name = "";
                for item in players[indices[0]].inventory.iter() {
                    if let item::Kind::Healing = item.kind {
                        item_name = item.name;
                        break;
                    }
                }

                format!(
                    "{} healed {} with {} {}.",
                    players[indices[0]].name,
                    players[indices[0]].pronouns.reflexive,
                    players[indices[0]].pronouns.possessive_adj,
                    item_name
                )
            },
            actions: |players, indices| {
                for (i, item) in players[indices[0]].inventory.iter().enumerate() {
                    if let item::Kind::Healing = item.kind {
                        players[indices[0]].inventory.remove(i);
                        break;
                    }
                }
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
                for item in players[indices[0]].inventory.iter() {
                    if let item::Kind::Weapon = item.kind {
                        return true;
                    }
                }

                false
            },
            message: |players, indices| {
                let weapon = players[indices[0]]
                    .inventory
                    .iter()
                    .filter(|x| {
                        if let item::Kind::Weapon = x.kind {
                            true
                        } else {
                            false
                        }
                    })
                    .collect::<Vec<&&item::Item>>()[0];
                format!(
                    "{} killed a deer with {} {}.",
                    players[indices[0]].name,
                    players[indices[0]].pronouns.possessive_adj,
                    weapon.name
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
