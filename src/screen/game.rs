use doryen_rs::Console;

use super::{handle_default, Action, Key, Screen};
use crate::colors::*;
use crate::game::Game;
use crate::map::tile::{Action as TAction, MapTile};
use crate::point::Point;
use crate::PLAYER_FOV;

pub struct GameScreen;

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
        let mut pos = game.player.pos;
        let l = game.cur_level_mut();
        match key {
            Key { key: "KeyY", .. } | Key { key: "Numpad7", .. } => pos += Point(-1, -1),
            Key { key: "KeyK", .. } | Key { key: "Numpad8", .. } | Key { key: "ArrowUp", .. } => {
                pos += Point(0, -1)
            }
            Key { key: "KeyU", .. } | Key { key: "Numpad9", .. } => pos += Point(1, -1),
            Key { key: "KeyH", .. }
            | Key { key: "Numpad4", .. }
            | Key {
                key: "ArrowLeft", ..
            } => pos += Point(-1, 0),
            Key { key: "KeyL", .. }
            | Key { key: "Numpad6", .. }
            | Key {
                key: "ArrowRight", ..
            } => pos += Point(1, 0),
            Key { key: "KeyB", .. } | Key { key: "Numpad1", .. } => pos += Point(-1, 1),
            Key { key: "KeyJ", .. }
            | Key { key: "Numpad2", .. }
            | Key {
                key: "ArrowDown", ..
            } => pos += Point(0, 1),
            Key { key: "KeyN", .. } | Key { key: "Numpad3", .. } => pos += Point(1, 1),
            Key { key: "KeyX", .. } => println!(
                "{}, {}: {}",
                pos.0,
                pos.1,
                l.fov_data.is_transparent(pos.0 as usize, pos.1 as usize)
            ),
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
            game.cur_level_mut().compute_fov(pos.0, pos.1, PLAYER_FOV);
        }
        Action::Keep
    }
    fn enter(&self, game: &mut Game) {
        let pos = game.player.pos;
        game.cur_level_mut().compute_fov(pos.0, pos.1, PLAYER_FOV);
    }
}
