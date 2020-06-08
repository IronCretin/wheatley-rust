use std::ops::Deref;

use crate::point::Point;
use crate::tile::Tile;
use crate::monster::Creature;

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
impl Creature for Player {
    fn get_pos(&self) -> Point {
        self.pos
    }
    fn set_pos(&mut self, pos: Point) {
        self.pos = pos
    }
}