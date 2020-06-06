use doryen_fov::{FovAlgorithm, FovRestrictive, MapData};
use ndarray::{Array, Array2};

pub mod gen;
pub mod tile;

use crate::Game;
use gen::Generator;
use tile::MapTile;

pub struct Level {
    pub width: i32,
    pub height: i32,
    pub fov_data: MapData,
    tiles: Array2<MapTile>,
    pub seen: Array2<bool>,
}

impl Level {
    pub fn generate<T: Generator>(width: i32, height: i32, game: &mut Game, gen: T) -> Level {
        let mut l = Level::new(width, height, &game.map.tiles["wall"]);
        gen.generate(game, &mut l);
        l
    }
    fn new(width: i32, height: i32, tile: &MapTile) -> Level {
        let mut fov_data = MapData::new(width as usize, height as usize);
        for x in 0..width as usize {
            for y in 0..width as usize {
                fov_data.set_transparent(x, y, tile.transparent);
            }
        }
        Level {
            width,
            height,
            fov_data,
            tiles: Array::from_elem((width as usize, height as usize), tile.clone()),
            seen: Array::from_elem((width as usize, height as usize), false),
        }
    }
    pub fn get(&self, x: i32, y: i32) -> &MapTile {
        return &self.tiles[[x as usize, y as usize]];
    }
    pub fn get_mut(&mut self, x: i32, y: i32) -> &mut MapTile {
        return &mut self.tiles[[x as usize, y as usize]];
    }
    pub fn set(&mut self, x: i32, y: i32, t: MapTile) {
        self.fov_data
            .set_transparent(x as usize, y as usize, t.transparent);
        self.tiles[[x as usize, y as usize]] = t;
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
    pub fn is_in_fov(&self, x: i32, y: i32) -> bool {
        self.fov_data.is_in_fov(x as usize, y as usize)
    }
}
