use super::Scenario;

pub fn get() -> Vec<Scenario> {
    vec![
        Scenario {
            possible_after: 0,
            impossible_after: 1,
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
                players[indices[2]].kill();
                players[indices[3]].kill();
                players[indices[0]].kills += 1;
                players[indices[1]].kills += 1;
            },
        },
        Scenario {
            possible_after: 1,
            impossible_after: 4,
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
        },
    ]
}
