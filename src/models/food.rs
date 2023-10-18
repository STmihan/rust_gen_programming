use crate::models::coord::Coord;

#[derive(Debug, Clone)]
pub struct Food {
    pub pos: Coord,
    pub restore: u8,
}
