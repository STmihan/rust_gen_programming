use crate::{config, render};
use crate::models::state::State;

pub fn render(state: &mut State) {
    render::walls::render_walls(&state.gen.walls, &mut state.canvas);
    render::agents::render_agents(&state.gen.agents, &mut state.canvas);
    render::agents::render_selected_agent(&state.selected_agent, &state.gen, &mut state.canvas);

    render::food::render_food(&state.gen.food, &mut state.canvas);

    if config::RENDER_GRID {
        render::board::render_board(&mut state.canvas,
                                    config::BOARD_WIDTH as usize,
                                    config::BOARD_HEIGHT as usize,
                                    config::CELL_WIDTH,
                                    config::CELL_HEIGHT,
                                    config::BOARD_COLOR,
        );
    }
}
