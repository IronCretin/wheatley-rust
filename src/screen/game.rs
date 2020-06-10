use std::borrow::Borrow;
use std::cell::Cell;
use std::convert::TryInto;

use doryen_rs::{Console, TextAlign};

use super::{handle_default, Action, Key, Screen};
use crate::colors::*;
use crate::game::Game;
use crate::monster::move_to;
use crate::point::Point;

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
            game.messages.push_back("Welcome to Wheatley!".to_owned());
            self.entered.set(false);
        }
    }
    fn render(&self, game: &mut Game, con: &mut Console) {
        let level = game.levels.cur_mut();
        let (w, h) = (con.get_width() as i32, con.get_height() as i32);
        let offset = level.monsters[0].pos - Point(w, h) / 2;
        for x in 0..w {
            for y in 0..h - 5 {
                let p = Point(x, y) + offset;
                if 0 <= p.0 && p.0 < level.width as i32 && 0 <= p.1 && p.1 < level.height as i32 {
                    let (ux, uy) = p.try_into().unwrap();
                    if level.tiles.is_in_fov(ux, uy) {
                        let t = &level.tiles.get(ux, uy);
                        t.draw(Point(x, y), con);
                        let ch = t.ch;
                        level.seen[[ux, uy]] = Some(ch);
                    } else if let Some(ch) = level.seen[[ux, uy]] {
                        con.cell(x, y, Some(ch), Some(DARKER_GREY), Some(BLACK));
                    }
                }
            }
        }
        for mon in &level.monsters {
            if level
                .tiles
                .is_in_fov(mon.pos.0 as usize, mon.pos.1 as usize)
            {
                let p = mon.pos - offset;
                mon.draw(p, con);
                level.seen[[mon.pos.0 as usize, mon.pos.1 as usize]] = Some(mon.ch);
            }
        }
        for (i, msg) in game.messages.iter().rev().take(5).enumerate() {
            // println!("{}: {}", con.get_height() as i32 - i as i32, msg);
            con.print_color(
                0,
                con.get_height() as i32 - i as i32 - 1,
                msg,
                TextAlign::Left,
                None,
            );
        }
    }
    fn handle(&self, game: &mut Game, key: Key) -> Action {
        let pos = game.levels.cur().monsters[0].pos;
        let l = game.levels.cur_mut();
        match key {
            Key { key: "KeyC", .. } => {
                for x in -1..=1 {
                    for y in -1..=1 {
                        if x != 0 || y != 0 {
                            let (ux, uy) = (pos + Point(x, y)).try_into().unwrap();
                            if let Some(cname) = &l.tiles.get(ux, uy).close {
                                let ctile =
                                    game.info.map.tiles[Borrow::<String>::borrow(cname)].clone();
                                l.tiles.set(ux, uy, ctile);
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
        let level = game.levels.cur_mut();
        let tick = move_to(0, dpos, level, &game.info);
        if tick {
            let player = &level.monsters[0];
            let fov = game.info.settings.player.fov;
            let pos = player.pos;
            game.levels
                .cur_mut()
                .tiles
                .compute_fov(pos.0 as usize, pos.1 as usize, fov);
        }
    }
}
