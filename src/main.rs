mod gui;
mod player;
mod scenario;
mod simulator;

use simulator::*;

const DEBUGGING_ENABLED: bool = false;

fn main() {
    //gui::start();
    simulate_game();
}
