use std::rc::Rc;

use crate::player::Player;
use crate::point::Point;
use crate::screen::Screen;
use crate::PLAYER_TILE;

pub struct Game {
    pub menu: Rc<dyn Screen>,
    pub help: Rc<dyn Screen>,
    pub player: Player,
}

impl Game {
    pub fn new(menu: Rc<dyn Screen>, help: Rc<dyn Screen>) -> Game {
        Game {
            menu,
            help,
            player: Player {
                tile: PLAYER_TILE,
                pos: Point(0, 0),
            },
        }
    }
}
