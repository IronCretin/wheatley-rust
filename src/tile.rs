use doryen_rs::{Color, Console};

use crate::point::Point;

#[derive(Copy, Clone, Debug)]
pub struct Tile {
    pub ch: u16,
    pub fg: Color,
    pub bg: Color,
}
impl Tile {
    pub fn draw(&self, p: Point, con: &mut Console) {
        con.cell(p.0, p.1, Some(self.ch), Some(self.fg), Some(self.bg))
    }
}
