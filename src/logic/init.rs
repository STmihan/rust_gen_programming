use crate::config;
use crate::logic::utils;
use crate::models::action::Action;
use crate::models::agent::Agent;
use crate::models::direction::Direction;
use crate::models::food::Food;
use crate::models::gen::Gen;

fn init_walls(gen: &mut Gen) {
    for _i in 0..config::WALLS_COUNT {
        let coord = utils::random_non_empty_coord(gen);
        gen.walls.push(coord);
    }
}

fn init_food(gen: &mut Gen) {
    for _i in 0..config::FOOD_COUNT {
        gen.food.push(Food {
            pos: utils::random_non_empty_coord(gen),
            restore: config::AGENT_ENERGY_RESTORE,
        });
    }
}

fn init_agents(gen: &mut Gen) {
    for _i in 0..config::AGENT_COUNT {
        gen.agents.push(Agent {
            pos: utils::random_non_empty_coord(gen),
            energy: config::AGENT_ENERGY,
            dir: Direction::Right,
            action: Action::None,
            is_dead: false,
            score: 0,
            view_distance: config::AGENT_VIEW_DISTANCE,
        });
    }
}


pub fn init_gen(gen: &mut Gen) {
    init_walls(gen);
    init_agents(gen);
    init_food(gen);
}