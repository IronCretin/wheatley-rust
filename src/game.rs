use std::rc::Rc;

use crate::map::{gen::Empty, Level};
use crate::player::Player;
use crate::point::Point;
use crate::screen::Screen;
use crate::PLAYER_TILE;

pub struct Game {
    pub menu: Rc<dyn Screen>,
    pub help: Rc<dyn Screen>,
    pub player: Player,
    pub level: i32,
    floors: Vec<Level>,
    basement: Vec<Level>,
}

impl Game {
    pub fn new(menu: Rc<dyn Screen>, help: Rc<dyn Screen>) -> Game {
        Game {
            menu,
            help,
            player: Player {
                tile: PLAYER_TILE,
                pos: Point(1, 1),
            },
            level: 0,
            floors: vec![Level::generate(100, 100, Empty)],
            basement: vec![],
        }
    }
    pub fn cur_level(&self) -> &Level {
        if self.level < 0 {
            &self.basement[(-self.level + 1) as usize]
        } else {
            &self.floors[self.level as usize]
        }
    }
    pub fn cur_level_mut(&mut self) -> &mut Level {
        if self.level < 0 {
            &mut self.basement[(-self.level + 1) as usize]
        } else {
            &mut self.floors[self.level as usize]
        }
    }
}
