use sdl2::event::Event;
use sdl2::EventPump;
use sdl2::keyboard::Keycode;

use crate::{config, gen_logic, logic};
use crate::models::state::State;

pub fn process_event(event_pump: &mut EventPump, state: &mut State) {
    for event in event_pump.poll_iter() {
        match event {
            Event::Quit { .. } => on_escape_down(state),

            Event::KeyDown { keycode: Some(Keycode::Escape), .. } => on_escape_down(state),
            Event::KeyDown { keycode: Some(Keycode::Space), .. } => on_space_down(state),
            Event::KeyDown { keycode: Some(Keycode::P), .. } => on_p_down(state),
            Event::KeyDown { keycode: Some(Keycode::R), .. } => on_r_down(state),

            Event::MouseButtonDown { x, y, .. } => on_mouse_button_down(state, x, y),
            _ => {}
        }
    }
}

fn on_mouse_button_down(state: &mut State, x: i32, y: i32) {
    let x = x - (config::WINDOW_WIDTH - config::CANVAS_WIDTH) as i32 / 2;
    let y = y - (config::WINDOW_HEIGHT - config::CANVAS_HEIGHT) as i32 / 2;
    let agent = logic::get_at::get_agent_at_canvas(x, y, &state.gen);
    if agent.is_some() {
        let agent = &state.gen.agents[agent.unwrap()];
        println!("Selected agent: {:?}", agent);
        let foods = logic::get_at::get_all_food_in_view(agent.pos, agent.dir, agent.view_distance, &state.gen);
        println!("Foods in view: {:?}", foods);
        let walls = logic::get_at::get_all_walls_in_view(agent.pos, agent.dir, agent.view_distance, &state.gen);
        println!("Walls in view: {:?}", walls);
        let agents = logic::get_at::get_all_agents_in_view(agent.pos, agent.dir, agent.view_distance, &state.gen);
        println!("Agents in view: {:?}", agents);
    }
    state.selected_agent = agent;
}

fn on_space_down(state: &mut State) {
    gen_logic::step_gen(&mut state.gen)
}

fn on_p_down(state: &mut State) {
    println!("Gen: \n{:?}", state.gen);
}

fn on_escape_down(state: &mut State) {
    state.exit = true;
}

fn on_r_down(state: &mut State) {
    gen_logic::restart(state);
}
