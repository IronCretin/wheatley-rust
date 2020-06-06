use doryen_rs::Console;

use super::{handle_default, Action, Key, Screen};
use crate::colors::*;
use crate::game::Game;
use crate::map::tile::{Action as TAction, MapTile};
use crate::point::Point;

pub struct GameScreen;

impl GameScreen {
    pub fn new() -> GameScreen {
        GameScreen {}
    }
    fn player_move(&self, game: &mut Game, dpos: Point) {
        let mut pos = game.player.pos + dpos;
        let l = game.cur_level_mut();
        let mut tick = false;
        if 0 <= pos.0 && pos.0 < l.width && 0 <= pos.1 && pos.1 < l.height {
            let tile = l.get(pos.0, pos.1);
            if tile.walkable {
                game.player.pos = pos;
                tick = true
            } else if let TAction::Open(otile, otransparent, owalkable) = tile.action {
                let ctile = tile.clone();
                l.set(
                    pos.0,
                    pos.1,
                    MapTile {
                        tile: otile,
                        transparent: otransparent,
                        walkable: owalkable,
                        action: TAction::Close(ctile.tile, ctile.transparent, ctile.walkable),
                    },
                );
                pos = game.player.pos;
                tick = true
            }
        }
        if tick {
            let fov = game.settings.player.fov;
            game.cur_level_mut().compute_fov(pos.0, pos.1, fov);
        }
    }
}

impl Screen for GameScreen {
    fn render(&self, game: &mut Game, con: &mut Console) {
        let pos = game.player.pos;
        let level = game.cur_level_mut();
        let (w, h) = (con.get_width() as i32, con.get_height() as i32);
        let offset = pos - Point(w, h) / 2;
        for x in 0..w {
            for y in 0..h {
                let p = Point(x, y) + offset;
                if 0 <= p.0 && p.0 < level.width && 0 <= p.1 && p.1 < level.height {
                    if level.is_in_fov(p.0, p.1) {
                        level.seen[[p.0 as usize, p.1 as usize]] = true;
                        level.get(p.0, p.1).draw(Point(x, y), con);
                    } else if level.seen[[p.0 as usize, p.1 as usize]] {
                        con.cell(
                            x,
                            y,
                            Some(level.get(p.0, p.1).ch),
                            Some(DARK_GREY),
                            Some(BLACK),
                        );
                    }
                }
            }
        }
        game.player.draw(game.player.pos - offset, con);
    }
    fn handle(&self, game: &mut Game, key: Key) -> Action {
        let pos = game.player.pos;
        let l = game.cur_level_mut();
        match key {
            Key { key: "KeyC", .. } => {
                for x in -1..=1 {
                    for y in -1..=1 {
                        if x != 0 || y != 0 {
                            let p = pos + Point(x, y);
                            let tile = l.get(p.0, p.1);
                            if let TAction::Close(ctile, ctransparent, cwalkable) = tile.action {
                                let otile = tile.clone();
                                l.set(
                                    p.0,
                                    p.1,
                                    MapTile {
                                        tile: ctile,
                                        transparent: ctransparent,
                                        walkable: cwalkable,
                                        action: TAction::Open(
                                            otile.tile,
                                            otile.transparent,
                                            otile.walkable,
                                        ),
                                    },
                                );
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
        self.player_move(game, dpos);
    }
    fn enter(&self, game: &mut Game) {
        let pos = game.player.pos;
        let fov = game.settings.player.fov;
        game.cur_level_mut().compute_fov(pos.0, pos.1, fov);
    }
}
