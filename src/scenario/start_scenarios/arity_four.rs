use super::Scenario;

pub fn get() -> Vec<Scenario> {
    vec![Scenario {
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
        },
    }]
}
