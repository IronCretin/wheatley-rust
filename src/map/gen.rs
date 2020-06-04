use tcod::random::Rng;

use super::tile::WALL;
use crate::map::Level;

pub mod hallways;

pub trait Generator {
    fn generate(&self, rng: &mut Rng, level: &mut Level);
}

pub struct Empty;

impl Generator for Empty {
    fn generate(&self, _rng: &mut Rng, level: &mut Level) {
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
    fn generate(&self, rng: &mut Rng, level: &mut Level) {
        Empty.generate(rng, level);
        for x in 0..level.width {
            for y in 0..level.height {
                if rng.get_double(0.0, 1.0) <= self.0 {
                    level.set(x, y, WALL);
                }
            }
        }
    }
}
