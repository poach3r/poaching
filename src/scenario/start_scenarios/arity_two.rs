use super::Scenario;
use crate::item;

pub fn get() -> Vec<Scenario> {
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
                players[indices[1]].inventory.push(&item::BREAD);
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
                players[indices[0]].inventory.push(&item::BREAD);
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
    ]
}
