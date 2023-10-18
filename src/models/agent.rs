use crate::models::action::Action;
use crate::models::coord::Coord;
use crate::models::direction::Direction;

#[derive(Debug, Clone)]
pub struct Agent {
    pub pos: Coord,
    pub energy: i16,
    pub dir: Direction,
    pub action: Action,

    pub is_dead: bool,

    pub score: u32,
    pub view_distance: u8,
}

impl Agent {
    pub fn get_front(&self) -> Coord {
        return self.pos.get_dir(self.dir);
    }
}
