use crate::{config, gen_logic, gen_render};
use crate::models::state::State;

pub fn init(state: &mut State) { gen_logic::init_gen(&mut state.gen); }

pub fn run(state: &mut State) {
    gen_render::render(state);
    if config::TRAINING_MODE {
        gen_logic::step_gen(&mut state.gen);
        if state.gen.agents.iter().all(|a| a.is_dead) {
            gen_logic::restart(state);
        }
    }
}
