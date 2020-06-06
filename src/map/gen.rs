use rand::Rng;

use crate::map::Level;
use crate::Game;

// pub mod hallways;
// pub use hallways::Hallways;

pub trait Generator {
    fn generate(&self, game: &mut Game, level: &mut Level);
}

pub struct Empty;

impl Generator for Empty {
    fn generate(&self, game: &mut Game, level: &mut Level) {
        let floor = &game.map.tiles["floor"];
        for x in 1..level.width - 1 {
            for y in 0..level.height - 1 {
                level.set(x, y, floor.clone());
            }
        }
    }
}

pub struct Percent(pub f64);

impl Generator for Percent {
    fn generate(&self, game: &mut Game, level: &mut Level) {
        let floor = &game.map.tiles["floor"];
        for x in 0..level.width {
            for y in 0..level.height {
                if game.map_rng.gen_bool(1.0 - self.0) {
                    level.set(x, y, floor.clone());
                }
            }
        }
    }
}
