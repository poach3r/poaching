use super::Scenario;
use crate::item;

pub fn get() -> Vec<Scenario> {
    vec![
        Scenario {
            condition: |_, _| true,
            message: |players, indices| format!("{} found some bread.", players[indices[0]].name),
            actions: |players, indices| {
                players[indices[0]].inventory.push(&item::BREAD);
            },
        },
        Scenario {
            condition: |_, _| true,
            message: |players, indices| {
                format!("{} found bottled water.", players[indices[0]].name)
            },
            actions: |players, indices| {
                players[indices[0]].inventory.push(&item::WATER);
            },
        },
        Scenario::nothing_burger(|players, indices| {
            format!(
                "{} tried to grab supplies but was scared off.",
                players[indices[0]].name
            )
        }),
        Scenario {
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
        Scenario::nothing_burger(|players, indices| {
            format!(
                "{} runs away with a lighter and some rope.",
                players[indices[0]].name
            )
        }),
        Scenario::nothing_burger(|players, indices| {
            format!("{} sprinted into the woods.", players[indices[0]].name)
        }),
        Scenario::nothing_burger(|players, indices| {
            format!("{} sprinted into the woods.", players[indices[0]].name)
        }),
        Scenario::nothing_burger(|players, indices| {
            format!("{} sprinted into the woods.", players[indices[0]].name)
        }),
    ]
}
