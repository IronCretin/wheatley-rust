use std::ops::Deref;
use std::rc::Rc;

use serde_derive::Deserialize;

use crate::tile::Tile;

#[derive(Clone, Debug, Deserialize)]
pub struct MapTile {
    #[serde(flatten)]
    pub tile: Tile,

    pub transparent: bool,
    pub walkable: bool,
    pub open: Option<Rc<String>>,
    pub close: Option<Rc<String>>,
    pub flip: Option<Rc<String>>,
}

impl Deref for MapTile {
    type Target = Tile;
    fn deref(&self) -> &Tile {
        &self.tile
    }
}
