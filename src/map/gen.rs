use rand::Rng;

use super::tile::FLOOR;
use crate::map::Level;

pub mod hallways;
pub use hallways::Hallways;

pub trait Generator {
    fn generate<R: Rng>(&self, rng: &mut R, level: &mut Level);
}

pub struct Empty;

impl Generator for Empty {
    fn generate<R: Rng>(&self, _rng: &mut R, level: &mut Level) {
        for x in 1..level.width - 1 {
            for y in 0..level.height - 1 {
                level.set(x, y, FLOOR);
            }
        }
    }
}

pub struct Percent(pub f64);

impl Generator for Percent {
    fn generate<R: Rng>(&self, rng: &mut R, level: &mut Level) {
        for x in 0..level.width {
            for y in 0..level.height {
                if rng.gen_bool(1.0 - self.0) {
                    level.set(x, y, FLOOR);
                }
            }
        }
    }
}
