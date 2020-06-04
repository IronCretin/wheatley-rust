use std::ops::Deref;

use tcod::colors::*;

use crate::tile::Tile;

#[derive(Clone)]
pub struct MapTile {
    pub tile: Tile,
    pub transparent: bool,
    pub walkable: bool,
}

impl Deref for MapTile {
    type Target = Tile;
    fn deref(&self) -> &Tile {
        &self.tile
    }
}

pub const WALL: MapTile = MapTile {
    tile: Tile {
        ch: '#',
        fg: WHITE,
        bg: BLACK,
    },
    transparent: false,
    walkable: false,
};
pub const FLOOR: MapTile = MapTile {
    tile: Tile {
        ch: '.',
        fg: GREY,
        bg: BLACK,
    },
    transparent: true,
    walkable: true,
};
pub const DOOR: MapTile = MapTile {
    tile: Tile {
        ch: '+',
        fg: BRASS,
        bg: BLACK,
    },
    transparent: false,
    walkable: false,
};

pub const DEFAULT_TILE: MapTile = FLOOR;
