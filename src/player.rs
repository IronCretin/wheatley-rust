use std::ops::Deref;

use crate::point::Point;
use crate::tile::Tile;

pub struct Player {
    pub tile: Tile,
    pub pos: Point,
}
impl Deref for Player {
    type Target = Tile;
    fn deref(&self) -> &Tile {
        &self.tile
    }
}
