use std::fmt::{Debug, Formatter};
use crate::config;
use crate::models::agent::Agent;
use crate::models::coord::Coord;
use crate::models::food::Food;

#[derive(Clone)]
pub struct Gen {
    pub agents: Vec<Agent>,
    pub walls: Vec<Coord>,
    pub food: Vec<Food>,
}

impl Gen {
    pub fn new() -> Self {
        Self {
            agents: Vec::with_capacity(config::AGENT_COUNT),
            walls: Vec::with_capacity(config::WALLS_COUNT),
            food: Vec::with_capacity(config::FOOD_COUNT),
        }
    }
}

impl Debug for Gen {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        for y in 0..config::BOARD_WIDTH {
            for x in 0..config::BOARD_HEIGHT {
                let x = x as i32;
                let y = y as i32;
                let coord = Coord { x , y };
                let mut found: char = '-';
                for agent in &self.agents {
                    if &agent.pos == &coord {
                        found = 'A';
                        break;
                    }
                }
                for wall in &self.walls {
                    if *wall == coord {
                        found = 'W';
                        break;
                    }
                }
                for food in &self.food {
                    if &food.pos == &coord {
                        found = 'F';
                        break;
                    }
                }
                write!(f, "{}", found)?;
            }
            write!(f, "\n")?;
        }

        Ok(())
    }
}
