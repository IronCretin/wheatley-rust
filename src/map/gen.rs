use super::tile::WALL;
use crate::map::Level;

pub trait Generator {
    fn generate(&self, level: &mut Level);
}

pub struct Empty;

impl Generator for Empty {
    fn generate(&self, level: &mut Level) {
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
