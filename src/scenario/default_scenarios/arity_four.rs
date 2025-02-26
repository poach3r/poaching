use super::Scenario;

pub fn get() -> Vec<Scenario> {
    vec![Scenario {
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
    }]
}
