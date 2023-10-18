use rand::Rng;
use crate::nn::input::Input;
use crate::nn::output::Output;

pub fn nn(input: &Input) -> Output {
    Output { action: rand::thread_rng().gen_range(0..input.actions_count) }
}