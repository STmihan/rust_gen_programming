use sdl2::render::Canvas;
use sdl2::video::Window;
use crate::models::gen::Gen;

pub struct State {
    pub exit: bool,
    pub canvas: Canvas<Window>,
    pub gen: Gen,
    pub selected_agent: Option<usize>,
}

impl State {
    pub fn new(canvas: Canvas<Window>) -> Self {
        State { exit: false, canvas, gen: Gen::new(), selected_agent: None }
    }
}