use rand::Rng;

use super::tile::WALL;
use crate::map::Level;

pub mod hallways;
pub use hallways::Hallways;

pub trait Generator {
    fn generate<R: Rng>(&self, rng: &mut R, level: &mut Level);
}

pub struct Empty;

impl Generator for Empty {
    fn generate<R: Rng>(&self, _rng: &mut R, level: &mut Level) {
        let h = level.height - 1;
        for x in 0..level.width {
            level.set(x, 0, WALL);
            level.set(x, h, WALL);
        }
        let w = level.height - 1;
        for y in 0..level.height {
            level.set(0, y, WALL);
            level.set(w, y, WALL);
        }
    }
}

pub struct Percent(pub f64);

impl Generator for Percent {
    fn generate<R: Rng>(&self, rng: &mut R, level: &mut Level) {
        Empty.generate(rng, level);
        for x in 0..level.width {
            for y in 0..level.height {
                if rng.gen_bool(self.0) {
                    level.set(x, y, WALL);
                }
            }
        }
    }
}
