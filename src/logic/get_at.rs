use crate::config;
use crate::models::agent::Agent;
use crate::models::coord::Coord;
use crate::models::direction::Direction;
use crate::models::food::Food;
use crate::models::gen::Gen;

pub fn get_agent_at_canvas(x: i32, y: i32, gen: &Gen) -> Option<usize> {
    let canvas_x = x / config::CELL_WIDTH as i32;
    let canvas_y = y / config::CELL_HEIGHT as i32;
    let coord = Coord { x: canvas_x, y: canvas_y };
    get_agent_at(&coord, gen)
}

pub fn get_agent_at(at: &Coord, gen: &Gen) -> Option<usize> {
    for i in 0..gen.agents.len() {
        if &gen.agents[i].pos.x == &at.x && &gen.agents[i].pos.y == &at.y {
            return Some(i);
        }
    }

    return None;
}

pub fn get_food_at(at: &Coord, gen: &Gen) -> Option<usize> {
    for i in 0..gen.food.len() {
        if &gen.food[i].pos.x == &at.x && &gen.food[i].pos.y == &at.y {
            return Some(i);
        }
    }

    return None;
}

pub fn get_wall_at(at: &Coord, gen: &Gen) -> Option<usize> {
    for i in 0..gen.walls.len() {
        if gen.walls[i].x == at.x && gen.walls[i].y == at.y {
            return Some(i);
        }
    }

    return None;
}

pub fn get_all_walls_in_view(point: Coord, view_direction: Direction, view_distance: u8, gen: &Gen) -> Vec<Coord> {
    let mut walls = vec![];

    let mut coord = point;
    for _i in 0..view_distance {
        coord = coord.get_dir(view_direction);
        if let Some(wall) = get_wall_at(&coord, gen) {
            walls.push(gen.walls[wall]);
        }
    }

    return walls;
}

pub fn get_all_food_in_view(point: Coord, view_direction: Direction, view_distance: u8, gen: &Gen) -> Vec<&Food> {
    let mut foods = vec![];

    let mut coord = point;
    for _i in 0..view_distance {
        coord = coord.get_dir(view_direction);
        if let Some(food) = get_food_at(&coord, gen) {
            foods.push(&gen.food[food]);
        }
    }

    return foods;
}

pub fn get_all_agents_in_view(point: Coord, view_direction: Direction, view_distance: u8, gen: &Gen) -> Vec<&Agent> {
    let mut agents = vec![];

    let mut coord = point;
    for _i in 0..view_distance {
        coord = coord.get_dir(view_direction);
        if let Some(agent) = get_agent_at(&coord, gen) {
            agents.push(&gen.agents[agent]);
        }
    }

    return agents;
}
