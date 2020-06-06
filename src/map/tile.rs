use std::ops::Deref;

use crate::colors::*;
use crate::tile::Tile;

#[derive(Clone, Debug)]
pub struct MapTile {
    pub tile: Tile,
    pub transparent: bool,
    pub walkable: bool,
    pub action: Action,
}

#[derive(Clone, Debug)]
pub enum Action {
    Open(Tile, bool, bool),
    Close(Tile, bool, bool),
    CloseFlip,
    None,
}

impl Deref for MapTile {
    type Target = Tile;
    fn deref(&self) -> &Tile {
        &self.tile
    }
}

pub const WALL: MapTile = MapTile {
    tile: Tile {
        ch: '#' as u16,
        fg: WHITE,
        bg: BLACK,
    },
    transparent: false,
    walkable: false,
    action: Action::None,
};
pub const FLOOR: MapTile = MapTile {
    tile: Tile {
        ch: '.' as u16,
        fg: LIGHT_GREY,
        bg: BLACK,
    },
    transparent: true,
    walkable: true,
    action: Action::None,
};
pub const DOOR: MapTile = MapTile {
    tile: Tile {
        ch: '+' as u16,
        fg: BRASS,
        bg: BLACK,
    },
    transparent: false,
    walkable: false,
    action: Action::Open(
        Tile {
            ch: '\'' as u16,
            fg: BRASS,
            bg: BLACK,
        },
        true,
        true,
    ),
};

pub const DEFAULT_TILE: MapTile = WALL;
