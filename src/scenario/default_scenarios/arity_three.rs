use super::Scenario;

pub fn get() -> Vec<Scenario> {
    vec![
        Scenario {
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
