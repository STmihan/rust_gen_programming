use sdl2::gfx::primitives::DrawRenderer;
use sdl2::pixels::Color;
use sdl2::rect::Rect;
use sdl2::render::Canvas;
use sdl2::video::Window;
use crate::{config, logic};
use crate::models::agent::Agent;
use crate::models::direction::Direction;
use crate::models::gen::Gen;

fn draw_agent(x: f32, y: f32, dir: Direction, canvas: &mut Canvas<Window>, color: Color) {
    let dir: usize = dir as usize;

    const AGENT_DIRECTION_POINTS: [[f32; 6]; 4] = [
        [0., 1., 0.5, 0., 1., 1.], // Up
        [0., 0., 1., 0.5, 0., 1.], // Right
        [0., 0., 0.5, 1., 1., 0.], // Down
        [1., 0., 0., 0.5, 1., 1.], // Left
    ];

    let x1 = x + AGENT_DIRECTION_POINTS[dir][0] * config::CELL_WIDTH as f32;
    let y1 = y + AGENT_DIRECTION_POINTS[dir][1] * config::CELL_HEIGHT as f32;

    let x2 = x + AGENT_DIRECTION_POINTS[dir][2] * config::CELL_WIDTH as f32;
    let y2 = y + AGENT_DIRECTION_POINTS[dir][3] * config::CELL_HEIGHT as f32;

    let x3 = x + AGENT_DIRECTION_POINTS[dir][4] * config::CELL_WIDTH as f32;
    let y3 = y + AGENT_DIRECTION_POINTS[dir][5] * config::CELL_HEIGHT as f32;

    let scale = config::AGENT_SCALE;
    let x1 = x + (x1 - x) * scale + config::CELL_WIDTH as f32 * (1. - scale) / 2.;
    let y1 = y + (y1 - y) * scale + config::CELL_HEIGHT as f32 * (1. - scale) / 2.;
    let x2 = x + (x2 - x) * scale + config::CELL_WIDTH as f32 * (1. - scale) / 2.;
    let y2 = y + (y2 - y) * scale + config::CELL_HEIGHT as f32 * (1. - scale) / 2.;
    let x3 = x + (x3 - x) * scale + config::CELL_WIDTH as f32 * (1. - scale) / 2.;
    let y3 = y + (y3 - y) * scale + config::CELL_HEIGHT as f32 * (1. - scale) / 2.;

    let x1 = x1 as i16;
    let y1 = y1 as i16;
    let x2 = x2 as i16;
    let y2 = y2 as i16;
    let x3 = x3 as i16;
    let y3 = y3 as i16;


    canvas.filled_trigon(
        x1, y1,
        x2, y2,
        x3, y3,
        color,
    ).unwrap();
}

pub fn render_selected_agent(agent_i: &Option<usize>, gen: &Gen, canvas: &mut Canvas<Window>) {
    if agent_i.is_some() {
        let selected_agent = agent_i.unwrap();
        render_agent_details(&gen.agents[selected_agent], canvas);
        // render_agent_view(&gen.agents[selected_agent], &gen, canvas);
    }
}

fn render_agent(agent: &Agent, canvas: &mut Canvas<Window>) {
    let x = (agent.pos.x * config::CELL_WIDTH as i32) as f32;
    let y = (agent.pos.y * config::CELL_HEIGHT as i32) as f32;

    let color = if agent.is_dead {
        config::DEAD_AGENT_COLOR
    } else {
        config::AGENT_COLOR
    };

    draw_agent(x, y, agent.dir, canvas, color)
}

pub fn render_agents(agents: &Vec<Agent>, canvas: &mut Canvas<Window>) {
    for agent in agents {
        render_agent(agent, canvas);
    }
}

fn render_agent_details(agent: &Agent, canvas: &mut Canvas<Window>) {
    let x = agent.pos.x * config::CELL_WIDTH as i32;
    let y = agent.pos.y * config::CELL_HEIGHT as i32;

    draw_agent(x as f32, y as f32, agent.dir, canvas, config::SELECTED_AGENT_COLOR);

    let energy_width = config::CELL_WIDTH;
    let energy_fill = (agent.energy as f32 / config::AGENT_ENERGY as f32 * energy_width as f32) as u32;
    let energy_rect = Rect::new(
        x,
        y,
        energy_fill,
        config::CELL_HEIGHT / 10,
    );
    canvas.set_draw_color(config::AGENT_ENERGY_COLOR);
    canvas.fill_rect(energy_rect).unwrap();
}

fn render_agent_view(agent: &Agent, gen: &Gen, canvas: &mut Canvas<Window>) {
    let agents_pos_in_view = logic::get_at::get_all_agents_in_view(agent.pos, agent.dir, agent.view_distance, gen);
    // let agents_dir_in_view = agents_pos_in_view.iter().map(|a| a.dir as i32).collect();
    let agents_pos_in_view: Vec<(i32, i32)> = agents_pos_in_view.iter().map(|a| (a.pos.x, a.pos.y)).collect();

    let foods_pos_in_view = logic::get_at::get_all_food_in_view(agent.pos, agent.dir, agent.view_distance, gen);
    let foods_pos_in_view: Vec<(i32, i32)> = foods_pos_in_view.iter().map(|f| (f.pos.x, f.pos.y)).collect();

    let walls_pos_in_view = logic::get_at::get_all_walls_in_view(agent.pos, agent.dir, agent.view_distance, gen);
    let walls_pos_in_view: Vec<(i32, i32)> = walls_pos_in_view.iter().map(|w| (w.x, w.y)).collect();


    for (x, y) in agents_pos_in_view {
        let x = x * config::CELL_WIDTH as i32 + config::CELL_WIDTH as i32 / 2;
        let y = y * config::CELL_HEIGHT as i32 + config::CELL_HEIGHT as i32 / 2;
        let rad = config::CELL_HEIGHT as i16 / 4;
        canvas.circle(x as i16, y as i16, rad, config::AGENT_COLOR).unwrap();
    }

    for (x, y) in foods_pos_in_view {
        let x = x * config::CELL_WIDTH as i32 + config::CELL_WIDTH as i32 / 2;
        let y = y * config::CELL_HEIGHT as i32 + config::CELL_HEIGHT as i32 / 2;
        let rad = config::CELL_HEIGHT as i16 / 4;
        canvas.circle(x as i16, y as i16, rad, config::AGENT_COLOR).unwrap();
    }

    for (x, y) in walls_pos_in_view {
        let x = x * config::CELL_WIDTH as i32 + config::CELL_WIDTH as i32 / 2;
        let y = y * config::CELL_HEIGHT as i32 + config::CELL_HEIGHT as i32 / 2;
        let rad = config::CELL_HEIGHT as i16 / 4;
        canvas.circle(x as i16, y as i16, rad, config::AGENT_COLOR).unwrap();
    }
}
