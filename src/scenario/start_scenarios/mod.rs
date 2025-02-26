mod arity_four;
mod arity_one;
mod arity_three;
mod arity_two;

use super::Scenario;

/// Returns a vector of vectors of scenarios
/// that are to be used at the start of the game.
/// The element of each vector is equivalent
/// to the arity of it's scenarios.
pub fn get() -> Vec<Vec<Scenario>> {
    vec![
        arity_one::get(),
        arity_two::get(),
        arity_three::get(),
        arity_four::get(),
    ]
}
