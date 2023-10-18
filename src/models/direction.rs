use std::fmt::{Debug, Formatter, Result};
use std::mem::transmute;
use crate::models::direction::Direction::DirectionCount;

#[derive(Copy, Clone)]
pub enum Direction {
    Up = 0,
    Right,
    Down,
    Left,

    DirectionCount,
}

impl Direction {
    pub fn turn_right(&self) -> Direction {
        let dir_count = DirectionCount as u8;
        let dir = *self;
        let next = (dir as u8 + 1) % dir_count;
        Direction::from_u8(next)
    }

    pub fn turn_left(&self) -> Direction {
        let dir_count = DirectionCount as u8;
        let dir = *self;
        let prev = (dir as u8 + dir_count - 1) % dir_count;
        Direction::from_u8(prev)
    }

    pub fn from_u8(dir: u8) -> Direction {
        let dir_count = DirectionCount as u8;

        if dir >= dir_count {
            panic!("Invalid direction");
        }

        unsafe { transmute(dir) }
    }
}

impl Debug for Direction {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        let dir = match self {
            Direction::Up => "Up",
            Direction::Right => "Right",
            Direction::Down => "Down",
            Direction::Left => "Left",
            _ => {
                panic!("Invalid direction");
            }
        };
        write!(f, "{}", dir)
    }
}
