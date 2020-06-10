use std::collections::{HashMap, VecDeque};
use std::ops::{Index, IndexMut};
use std::rc::Rc;

use doryen_extra::random::{Dice, MersenneTwister, Random};
use rand::distributions::{Distribution, Uniform};
use rand::SeedableRng;
use serde::Deserialize;

use crate::combat::DamageInfo;
use crate::map::{gen::Hallways, Level, MapInfo};
use crate::monster::{Attack, Monster, MonsterInfo};
use crate::point::Point;
use crate::screen::Screen;
use crate::tile::Tile;
use crate::util::insert_at_zero;

pub struct Game {
    pub info: GameInfo,
    pub menu: Rc<dyn Screen>,
    pub help: Rc<dyn Screen>,
    pub messages: VecDeque<String>,
    pub levels: Levels,
    pub map_rng: Random<MersenneTwister>,
    pub play_rng: Random<MersenneTwister>,
}

impl Game {
    pub fn new(info: GameInfo, menu: Rc<dyn Screen>, help: Rc<dyn Screen>, seed: u64) -> Game {
        let mut game = Game {
            info,
            menu,
            help,
            messages: VecDeque::new(),
            levels: Levels {
                level: 0,
                floors: Vec::new(),
                basement: Vec::new(),
            },
            map_rng: SeedableRng::seed_from_u64(seed),
            play_rng: SeedableRng::seed_from_u64(seed),
        };
        let mut level = Level::generate(
            game.info.settings.map.width,
            game.info.settings.map.height,
            &mut game,
            Hallways::new(7, 6),
        );
        let px = Uniform::from(0..level.width);
        let py = Uniform::from(0..level.height);
        for _ in 0..game.info.settings.map.place_attempts {
            let x = px.sample(&mut game.map_rng);
            let y = py.sample(&mut game.map_rng);
            if level.tiles.get(x, y).walkable {
                insert_at_zero(
                    &mut level.monsters,
                    Monster {
                        info: Rc::new(MonsterInfo {
                            weight: 0.0,
                            name: "player".to_owned(),
                            tile: game.info.settings.player.tile,
                            attacks: vec![Attack {
                                dam: Dice::new("1d6"),
                                class: "cringe".to_owned(),
                                text: None,
                            }],
                            health: 20,
                            friendly: true,
                        }),
                        hp: 20,
                        pos: Point(x as i32, y as i32),
                    },
                );
                break;
            }
        }
        let pos = level.monsters[0].pos;
        game.levels.add_top(level);
        game.levels.cur_mut().tiles.compute_fov(
            pos.0 as usize,
            pos.1 as usize,
            game.info.settings.player.fov,
        );
        game
    }
}

pub struct Levels {
    level: i32,
    floors: Vec<Level>,
    basement: Vec<Level>,
}
impl Levels {
    pub fn add_top(&mut self, level: Level) {
        self.floors.push(level);
    }
    pub fn cur_idx(&self) -> i32 {
        self.level
    }
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
impl Index<i32> for Levels {
    type Output = Level;
    fn index(&self, leveln: i32) -> &Level {
        if leveln >= 0 {
            &self.floors[leveln as usize]
        } else {
            &self.basement[(-leveln - 1) as usize]
        }
    }
}
impl IndexMut<i32> for Levels {
    fn index_mut(&mut self, leveln: i32) -> &mut Level {
        if leveln >= 0 {
            &mut self.floors[leveln as usize]
        } else {
            &mut self.basement[(-leveln - 1) as usize]
        }
    }
}

pub struct GameInfo {
    pub settings: GameSettings,
    pub map: MapInfo,
    pub monster: HashMap<String, Rc<MonsterInfo>>,
    pub damage: HashMap<String, DamageInfo>,
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
    pub fov: usize,
    pub tile: Tile,
}
#[derive(Debug, Deserialize, Clone)]
pub struct MapSettings {
    pub place_attempts: i32,
    pub num_monsters: u32,
    pub width: usize,
    pub height: usize,
}
