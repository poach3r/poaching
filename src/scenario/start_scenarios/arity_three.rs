use super::Scenario;

pub fn get() -> Vec<Scenario> {
    vec![Scenario {
        condition: |_, _| true,
        message: |players, indices| {
            format!(
                "I don't feel like writing what {}, {} and {} did because im lazy.",
                players[indices[0]].name, players[indices[1]].name, players[indices[2]].name
            )
        },
        actions: |_, _| {},
    }]
}
