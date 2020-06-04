use crate::tile::{HasTile, Tile};

pub struct Player {
    pub tile: Tile,
}
impl HasTile for Player {
    fn tile(&self) -> &Tile {
        &self.tile
    }
}
