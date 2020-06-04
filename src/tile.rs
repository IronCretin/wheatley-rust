use tcod::colors::*;
use tcod::console::{Console, Root};

#[derive(Clone)]
pub struct Tile {
    pub ch: char,
    pub fg: Color,
    pub bg: Color,
}
pub trait HasTile {
    fn tile(&self) -> &Tile;
}

fn draw<T: HasTile>(t: &T, x: i32, y: i32, display: &mut Root) {
    let tile = t.tile();
    display.put_char_ex(x, y, tile.ch, tile.fg, tile.bg)
}

impl Tile {
    fn tile(&self) -> &Tile {
        self
    }
}

pub const WALL: Tile = Tile {
    ch: '#',
    fg: WHITE,
    bg: BLACK,
};
pub const FLOOR: Tile = Tile {
    ch: '.',
    fg: GREY,
    bg: BLACK,
};
pub const DOOR: Tile = Tile {
    ch: '.',
    fg: GREY,
    bg: BLACK,
};
