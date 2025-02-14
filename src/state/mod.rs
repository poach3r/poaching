use rand::rngs::ThreadRng;

pub struct State<'a> {
    pub players: Vec<crate::player::Player<'a>>,
    pub scenarios: Vec<crate::scenario::Scenario>,
    pub rng: ThreadRng,
    pub round: u16,
}
