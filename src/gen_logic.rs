use crate::logic;
use crate::models::gen::Gen;

pub fn step_gen(gen: &mut Gen) {
    logic::step::step_gen(gen);
}
pub fn init_gen(gen: &mut Gen) {
    logic::init::init_gen(gen);
}
