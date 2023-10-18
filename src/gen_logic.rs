use crate::logic;
use crate::models::gen::Gen;
use crate::models::state::State;

pub fn step_gen(gen: &mut Gen) {
    logic::step::step_gen(gen);
}
pub fn init_gen(gen: &mut Gen) {
    logic::init::init_gen(gen);
}

pub fn restart(state: &mut State) {
    state.gen = Gen::new();
    state.selected_agent = None;
    init_gen(&mut state.gen);
}