use crate::{gen_logic, gen_render};
use crate::models::state::State;

pub fn init(state: &mut State) { gen_logic::init_gen(&mut state.gen); }

pub fn run(state: &mut State) { gen_render::render(state); }
