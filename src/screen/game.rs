use std::borrow::Borrow;
use std::cell::Cell;
use std::convert::TryInto;

use doryen_rs::Console;
use rand::distributions::{Distribution, Uniform};

use super::{handle_default, Action, Key, Screen};
use crate::colors::*;
use crate::game::Game;
use crate::map::{gen::Hallways, Level};
use crate::point::Point;
use crate::monster::move_to;

pub struct GameScreen {
    entered: Cell<bool>,
}

impl GameScreen {
    pub fn new() -> GameScreen {
        GameScreen {
            entered: Cell::new(false),
        }
    }
}

impl Screen for GameScreen {
    fn enter(&self, game: &mut Game) {
        if !self.entered.get() {
            let floor = Level::generate(
                game.info.settings.map.width,
                game.info.settings.map.height,
                game,
                Hallways::new(7, 6),
            );
            let px = Uniform::from(0..floor.width);
            let py = Uniform::from(0..floor.height);
            for _ in 0..game.info.settings.map.place_attempts {
                let x = px.sample(&mut game.map_rng);
                let y = py.sample(&mut game.map_rng);
                if floor.get(x, y).walkable {
                    game.player.pos = Point(x as i32, y as i32);
                    break;
                }
            }
            game.levels.add_top(floor);
            let pos = game.player.pos;
            game.levels
                .cur_mut()
                .compute_fov(pos.0, pos.1, game.info.settings.player.fov);
            self.entered.set(false);
        }
    }
    fn render(&self, game: &mut Game, con: &mut Console) {
        let pos = game.player.pos;
        let level = game.levels.cur_mut();
        let (w, h) = (con.get_width() as i32, con.get_height() as i32);
        let offset = pos - Point(w, h) / 2;
        for x in 0..w {
            for y in 0..h {
                let p = Point(x, y) + offset;
                if 0 <= p.0 && p.0 < level.width as i32 && 0 <= p.1 && p.1 < level.height as i32 {
                    let (ux, uy) = p.try_into().unwrap();
                    if level.is_in_fov(ux, uy) {
                        let t = level.get(ux, uy);
                        t.draw(Point(x, y), con);
                        let ch = t.ch;
                        level.seen.set_seen(ux, uy, ch);
                    } else if let Some(ch) = level.seen.is_seen(ux, uy) {
                        con.cell(x, y, Some(ch), Some(DARKER_GREY), Some(BLACK));
                    }
                }
            }
        }
        for mon in &level.monsters {
            if level.is_in_fov(mon.pos.0 as usize, mon.pos.1 as usize) {
                let p = mon.pos - offset;
                mon.draw(p, con);
                level.seen.set_seen(mon.pos.0 as usize, mon.pos.1 as usize, mon.ch);
            }
        }
        game.player.draw(game.player.pos - offset, con);
    }
    fn handle(&self, game: &mut Game, key: Key) -> Action {
        let pos = game.player.pos;
        let l = game.levels.cur_mut();
        match key {
            Key { key: "KeyC", .. } => {
                for x in -1..=1 {
                    for y in -1..=1 {
                        if x != 0 || y != 0 {
                            let (ux, uy) = (pos + Point(x, y)).try_into().unwrap();
                            if let Some(cname) = &l.get(ux, uy).close {
                                let ctile =
                                    game.info.map.tiles[Borrow::<String>::borrow(cname)].clone();
                                l.set(ux, uy, ctile);
                            }
                        }
                    }
                }
            }
            _ => return handle_default(game, key),
        }
        Action::Keep
    }
    fn handle_held<'a>(&self, game: &mut Game, held: Box<dyn Fn(&str) -> bool + 'a>) {
        let dpos = if held("KeyY") || held("Numpad7") {
            Point(-1, -1)
        } else if held("KeyK") || held("Numpad8") || held("ArrowUp") {
            Point(0, -1)
        } else if held("KeyU") || held("Numpad9") {
            Point(1, -1)
        } else if held("KeyH") || held("Numpad4") || held("ArrowLeft") {
            Point(-1, 0)
        } else if held("KeyL") || held("Numpad6") || held("ArrowRight") {
            Point(1, 0)
        } else if held("KeyB") || held("Numpad1") {
            Point(-1, 1)
        } else if held("KeyJ") || held("Numpad2") || held("ArrowDown") {
            Point(0, 1)
        } else if held("KeyN") || held("Numpad3") {
            Point(1, 1)
        } else {
            Point(0, 0)
        };
        let tick = move_to(&mut game.player, dpos, game.levels.cur_mut(), &game.info);
        if tick {
            let fov = game.info.settings.player.fov;
            let pos = game.player.pos;
            game.levels.cur_mut().compute_fov(pos.0, pos.1, fov);
        }
    }
}
