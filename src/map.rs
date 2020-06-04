use crate::tile::{Tile, FLOOR};
use ndarray::{Array, Array2};
use tcod::map::Map;

pub struct Level {
    pub width: i32,
    pub height: i32,
    pub map: Map,
    pub tiles: Array2<Tile>,
}

impl Level {
    fn new(width: i32, height: i32) -> Level {
        let mut map = Map::new(width, height);
        map.clear(true, true);
        Level {
            width,
            height,
            map,
            tiles: Array::from_elem((width as usize, height as usize), FLOOR),
        }
    }
}
