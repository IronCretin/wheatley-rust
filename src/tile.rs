use tcod::colors::Color;
use tcod::console::{Console, Root};

use crate::point::Point;

#[derive(Copy, Clone, Debug)]
pub struct Tile {
    pub ch: char,
    pub fg: Color,
    pub bg: Color,
}
impl Tile {
    pub fn draw(&self, p: Point, display: &mut Root) {
        display.put_char_ex(p.0, p.1, self.ch, self.fg, self.bg)
    }
}
