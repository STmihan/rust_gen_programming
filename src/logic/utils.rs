use rand::Rng;
use crate::config;
use crate::logic::get_at::{get_agent_at, get_food_at, get_wall_at};
use crate::models::coord::Coord;
use crate::models::gen::Gen;

pub fn random_non_empty_coord(gen: &Gen) -> Coord {
    let high = config::BOARD_WIDTH * config::BOARD_HEIGHT;
    for _i in 0..high {
        let coord = random_coord();
        if get_agent_at(&coord, gen).is_some() {
            continue;
        }
        if get_food_at(&coord, gen).is_some() {
            continue;
        }
        if get_wall_at(&coord, gen).is_some() {
            continue;
        }

        return coord;
    }

    panic!("random_non_empty_coord failed to find a non-empty coord");
}

pub fn random_coord() -> Coord {
    Coord {
        x: rand::thread_rng().gen_range(0..config::BOARD_WIDTH as i32),
        y: rand::thread_rng().gen_range(0..config::BOARD_HEIGHT as i32),
    }
}

pub fn is_out_of_bounds(at: &Coord) -> bool {
    if at.x < 0 || at.x >= config::BOARD_WIDTH as i32 {
        return true;
    }

    if at.y < 0 || at.y >= config::BOARD_HEIGHT as i32 {
        return true;
    }

    return false;
}
