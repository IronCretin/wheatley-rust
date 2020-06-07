use std::collections::HashMap;
use std::ops::Deref;
use std::rc::Rc;

use doryen_fov::{FovAlgorithm, FovRestrictive, MapData};
// use ndarray::{Array, Array2};
use serde_derive::Deserialize;

pub mod gen;

use crate::tile::Tile;
use crate::Game;
use gen::Generator;

#[derive(Debug, Deserialize)]
pub struct MapInfo {
    pub tiles: HashMap<String, MapTile>,
}

pub struct Level {
    pub width: usize,
    pub height: usize,
    pub fov_data: MapData,
    tiles: Vec<MapTile>,
    seen: Vec<bool>,
}

impl Level {
    pub fn generate<T: Generator>(width: usize, height: usize, game: &mut Game, gen: T) -> Level {
        let mut l = Level::new(width, height, &game.map_info.tiles["wall"]);
        gen.generate(game, &mut l);
        l
    }
    fn new(width: usize, height: usize, tile: &MapTile) -> Level {
        let mut fov_data = MapData::new(width, height);
        for x in 0..width {
            for y in 0..width {
                fov_data.set_transparent(x, y, tile.transparent);
            }
        }
        Level {
            width,
            height,
            fov_data,
            tiles: vec![tile.clone(); width * height],
            seen: vec![false; width * height],
        }
    }
    pub fn get(&self, x: usize, y: usize) -> &MapTile {
        return &self.tiles[x * self.width + y];
    }
    pub fn set(&mut self, x: usize, y: usize, t: MapTile) {
        self.fov_data.set_transparent(x, y, t.transparent);
        self.tiles[x * self.width + y] = t;
    }
    pub fn is_seen(&self, x: usize, y: usize) -> bool {
        self.seen[x * self.width + y]
    }
    pub fn set_seen(&mut self, x: usize, y: usize, seen: bool) {
        self.seen[x * self.width + y] = seen;
    }
    pub fn compute_fov(&mut self, x: i32, y: i32, radius: i32) {
        self.fov_data.clear_fov();
        FovRestrictive::default().compute_fov(
            &mut self.fov_data,
            x as usize,
            y as usize,
            radius as usize,
            true,
        );
    }
    pub fn is_in_fov(&self, x: usize, y: usize) -> bool {
        self.fov_data.is_in_fov(x, y)
    }
}

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
