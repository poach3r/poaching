use super::Scenario;
use crate::item;
use crate::status;

pub fn get() -> Vec<Scenario> {
    vec![
        Scenario {
            possible_after: 0,
            impossible_after: 1,
            condition: |_, _| true,
            message: |players, indices| {
                format!(
                    "{} fought {} for a bag but lost and ran away.",
                    players[indices[0]].name, players[indices[1]].name
                )
            },
            actions: |players, indices| {
                players[indices[0]].hurt();
                players[indices[1]].inventory.push(&item::BREAD);
            },
        },
        Scenario {
            possible_after: 0,
            impossible_after: 1,
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
            possible_after: 0,
            impossible_after: 1,
            condition: |_, _| true,
            message: |players, indices| {
                format!(
                    "{} broke {}'s nose for a basket of bread.",
                    players[indices[0]].name, players[indices[1]].name
                )
            },
            actions: |players, indices| {
                players[indices[0]].inventory.push(&item::BREAD);
                players[indices[1]].hurt();
            },
        },
        Scenario {
            possible_after: 0,
            impossible_after: 1,
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
            possible_after: 0,
            impossible_after: 1,
            condition: |_, _| true,
            message: |players, indices| {
                format!(
                    "{} curbstomped {}.",
                    players[indices[0]].name, players[indices[1]].name
                )
            },
            actions: |players, indices| {
                players[indices[1]].kill();
                players[indices[0]].kills += 1;
            },
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
                    "{} killed {} with {} {}.",
                    players[indices[0]].name,
                    players[indices[1]].name,
                    players[indices[0]].pronouns.possessive_adj,
                    players[indices[0]].inventory[item_index].name
                )
            },
            actions: |players, indices| {
                players[indices[0]].kills += 1;
                players[indices[1]].kill();
            },
        },
        Scenario {
            possible_after: 1,
            impossible_after: usize::MAX,
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
                players[indices[0]].kills += 1;
                players[indices[1]].kill();
            },
        },
        Scenario {
            possible_after: 1,
            impossible_after: usize::MAX,
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
            possible_after: 1,
            impossible_after: 6,
            condition: |players, indices| {
                if let status::Status::Alive(status::AliveStatus::Injured) =
                    players[indices[1]].status
                {
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
            possible_after: 1,
            impossible_after: 4,
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
                players[indices[0]].kills += 1;
                players[indices[1]].kills += 1;
            },
        },
        Scenario {
            possible_after: 1,
            impossible_after: usize::MAX,
            condition: |players, indices| players[indices[0]].get_item(&item::KNIFE).is_some(),
            message: |players, indices| {
                format!(
                    "{} hit {} with a long distance knife throw.",
                    players[indices[0]].name, players[indices[1]].name
                )
            },
            actions: |players, indices| {
                let i = players[indices[0]].get_item(&item::KNIFE).unwrap();
                players[indices[0]].inventory.remove(i);
                players[indices[0]].kills += 1;
                players[indices[1]].kill();
            },
        },
    ]
}
