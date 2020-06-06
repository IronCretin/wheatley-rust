use std::rc::Rc;

use rand::distributions::{Distribution, Uniform};
use rand::SeedableRng;
use rand_pcg::Pcg32;

use crate::map::{gen::Hallways, Level};
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
    pub map_rng: Pcg32,
}

const MAX_ATTEMPTS: i32 = 100;

impl Game {
    pub fn new(menu: Rc<dyn Screen>, help: Rc<dyn Screen>, seed: u64) -> Game {
        let mut game = Game {
            menu,
            help,
            player: Player {
                tile: PLAYER_TILE,
                pos: Point(1, 1),
            },
            level: 0,
            floors: Vec::new(),
            basement: Vec::new(),
            map_rng: SeedableRng::seed_from_u64(seed),
        };
        let floor = Level::generate(200, 200, &mut game.map_rng, Hallways::new(7, 6));
        let px = Uniform::from(0..floor.width);
        let py = Uniform::from(0..floor.height);
        for _ in 0..MAX_ATTEMPTS {
            let x = px.sample(&mut game.map_rng);
            let y = py.sample(&mut game.map_rng);
            if floor.get(x, y).walkable {
                game.player.pos = Point(x, y);
                break;
            }
        }
        game.floors.push(floor);
        game
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
