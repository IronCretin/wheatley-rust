use std::ops::Deref;

use serde_derive::Deserialize;

use crate::tile::Tile;

#[derive(Clone, Debug, Deserialize)]
pub struct MapTile {
    #[serde(flatten)]
    pub tile: Tile,

    pub transparent: bool,
    pub walkable: bool,
    pub open: Option<String>,
    pub close: Option<String>,
    pub flip: Option<String>,
}

impl Deref for MapTile {
    type Target = Tile;
    fn deref(&self) -> &Tile {
        &self.tile
    }
}
