use std::collections::HashMap;
use std::rc::Rc;

use rand::distributions::{Distribution, Uniform};
use rand::SeedableRng;
use rand_pcg::Pcg32;
use serde::Deserialize;

use crate::map::{gen::Hallways, tile::MapTile, Level};
use crate::player::Player;
use crate::point::Point;
use crate::screen::Screen;
use crate::PLAYER_TILE;

pub struct Game {
    pub settings: GameSettings,
    pub map: MapInfo,
    pub menu: Rc<dyn Screen>,
    pub help: Rc<dyn Screen>,
    pub player: Player,
    pub levels: Levels,
    pub map_rng: Pcg32,
}

impl Game {
    pub fn new(
        settings: GameSettings,
        map: MapInfo,
        menu: Rc<dyn Screen>,
        help: Rc<dyn Screen>,
        seed: u64,
    ) -> Game {
        let mut game = Game {
            settings,
            map,
            menu,
            help,
            player: Player {
                tile: PLAYER_TILE,
                pos: Point(1, 1),
            },
            levels: Levels {
                level: 0,
                floors: Vec::new(),
                basement: Vec::new(),
            },
            map_rng: SeedableRng::seed_from_u64(seed),
        };
        let floor = Level::generate(
            game.settings.map.width,
            game.settings.map.height,
            &mut game,
            Hallways::new(7, 6),
        );
        let px = Uniform::from(0..floor.width);
        let py = Uniform::from(0..floor.height);
        for _ in 0..game.settings.map.place_attempts {
            let x = px.sample(&mut game.map_rng);
            let y = py.sample(&mut game.map_rng);
            if floor.get(x, y).walkable {
                game.player.pos = Point(x, y);
                break;
            }
        }
        game.levels.floors.push(floor);
        game
    }
}

pub struct Levels {
    level: i32,
    floors: Vec<Level>,
    basement: Vec<Level>,
}
impl Levels {
    pub fn cur(&self) -> &Level {
        if self.level < 0 {
            &self.basement[(-self.level + 1) as usize]
        } else {
            &self.floors[self.level as usize]
        }
    }
    pub fn cur_mut(&mut self) -> &mut Level {
        if self.level < 0 {
            &mut self.basement[(-self.level + 1) as usize]
        } else {
            &mut self.floors[self.level as usize]
        }
    }
}

#[derive(Debug, Deserialize, Clone)]
pub struct GameSettings {
    pub interface: InterfaceSettings,
    pub player: PlayerSettings,
    pub map: MapSettings,
}
#[derive(Debug, Deserialize, Clone)]
pub struct InterfaceSettings {
    pub width: u32,
    pub height: u32,
    pub font: FontSettings,
    pub key_delay: u32,
}
#[derive(Debug, Deserialize, Clone)]
pub struct FontSettings {
    pub font: String,
    pub width: u32,
    pub height: u32,
}
#[derive(Debug, Deserialize, Clone)]
pub struct PlayerSettings {
    pub fov: i32,
}
#[derive(Debug, Deserialize, Clone)]
pub struct MapSettings {
    pub place_attempts: i32,
    // pub num_monsters: u32,
    pub width: i32,
    pub height: i32,
}

#[derive(Debug, Deserialize)]
pub struct MapInfo {
    pub tiles: HashMap<String, MapTile>,
}