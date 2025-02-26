use crate::{item, status};

use super::Scenario;

pub fn get() -> Vec<Scenario> {
    vec![
        Scenario {
            possible_after: 0,
            impossible_after: 1,
            condition: |_, _| true,
            message: |players, indices| {
                format!(
                    "I don't feel like writing what {}, {} and {} did because im lazy.",
                    players[indices[0]].name, players[indices[1]].name, players[indices[2]].name
                )
            },
            actions: |_, _| {},
        },
        Scenario {
            possible_after: 1,
            impossible_after: usize::MAX,
            condition: |players, indices| {
                if let status::Status::Alive(status::AliveStatus::Healthy) =
                    players[indices[2]].status
                {
                    players[indices[0]]
                        .get_item_kind(item::Kind::Defense)
                        .is_some()
                } else {
                    false
                }
            },
            message: |players, indices| {
                format!(
                    "{} was attacked by {}, but {} protected {}.",
                    players[indices[0]].name,
                    players[indices[1]].name,
                    players[indices[2]].name,
                    players[indices[0]].pronouns.object
                )
            },
            actions: |players, indices| {
                players[indices[1]].hurt();
                players[indices[2]].hurt();
            },
        },
        Scenario {
            possible_after: 1,
            impossible_after: usize::MAX,
            condition: |players, indices| players[indices[0]].get_item(&item::EXPLOSIVES).is_some(),
            message: |players, indices| {
                format!(
                    "{} set off a massive explosive that killed {} and {}.",
                    players[indices[0]].name, players[indices[1]].name, players[indices[2]].name
                )
            },
            actions: |players, indices| {
                let item_index = players[indices[0]].get_item(&item::EXPLOSIVES).unwrap();
                players[indices[0]].inventory.remove(item_index);
                players[indices[1]].kill();
                players[indices[2]].kill();
            },
        },
        Scenario {
            possible_after: 1,
            impossible_after: 6,
            condition: |_, _| true,
            message: |players, indices| {
                format!(
                    "{}, {} and {} discussed their plans for tomorrow.",
                    players[indices[0]].name, players[indices[1]].name, players[indices[2]].name
                )
            },
            actions: |_, _| {},
        },
        Scenario {
            possible_after: 1,
            impossible_after: usize::MAX,
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
                players[indices[1]].kills += 1;
            },
        },
        //Scenario {
        //    condition: |_, _| true,
        //    message: |players, indices| {
        //        format!(
        //            "{}, {} and {} were crushed by a falling tree at night.",
        //            players[indices[0]].name,
        //            players[indices[1]].name,
        //            players[indices[2]].name
        //        )
        //    },
        //    actions: |players, indices| {
        //        players[indices[0]].kill();
        //        players[indices[1]].kill();
        //        players[indices[2]].kill();
        //    },
        //},
    ]
}
