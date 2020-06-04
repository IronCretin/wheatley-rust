use ndarray::{Array, Array2};
use tcod::map::Map;
use tcod::random::Rng;

pub mod gen;
pub mod tile;

use gen::Generator;
use tile::{MapTile, DEFAULT_TILE};

pub struct Level {
    pub width: i32,
    pub height: i32,
    map: Map,
    tiles: Array2<MapTile>,
    pub seen: Array2<bool>,
}

impl Level {
    pub fn generate<T: Generator>(width: i32, height: i32, rng: &mut Rng, gen: T) -> Level {
        let mut l = Level::new(width, height);
        gen.generate(rng, &mut l);
        l
    }
    fn new(width: i32, height: i32) -> Level {
        let tile = DEFAULT_TILE;
        let mut map = Map::new(width, height);
        map.clear(tile.transparent, tile.walkable);
        Level {
            width,
            height,
            map,
            tiles: Array::from_elem((width as usize, height as usize), tile),
            seen: Array::from_elem((width as usize, height as usize), false),
        }
    }
    pub fn get(&self, x: i32, y: i32) -> &MapTile {
        return &self.tiles[[x as usize, y as usize]];
    }
    pub fn set(&mut self, x: i32, y: i32, t: MapTile) {
        self.map.set(x, y, t.transparent, t.walkable);
        self.tiles[[x as usize, y as usize]] = t;
    }
}
