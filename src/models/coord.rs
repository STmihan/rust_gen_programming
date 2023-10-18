use crate::models::direction::Direction;

#[derive(Debug, Clone, Copy)]
pub struct Coord {
    pub x: i32,
    pub y: i32,
}

impl PartialEq for Coord {
    fn eq(&self, other: &Self) -> bool {
        self.x == other.x && self.y == other.y
    }
}

impl Coord {
    pub fn get_dir(&self, dir: Direction) -> Coord {
        let result = match dir {
            Direction::Up => {
                Coord { x: self.x, y: self.y - 1 }
            }
            Direction::Right => {
                Coord { x: self.x + 1, y: self.y }
            }
            Direction::Down => {
                Coord { x: self.x, y: self.y + 1 }
            }
            Direction::Left => {
                Coord { x: self.x - 1, y: self.y }
            }
            _ => { panic!("Invalid direction") }
        };

        return result;
    }
}
