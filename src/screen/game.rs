use tcod::colors::*;
use tcod::console::{Console, Root};
use tcod::input::Key;
use tcod::map::FovAlgorithm::Permissive2;

use super::{handle_default, Action, Screen};
use crate::game::Game;
use crate::map::tile::{Action as TAction, MapTile};
use crate::point::Point;
use crate::PLAYER_FOV;

pub struct GameScreen;

impl Screen for GameScreen {
    fn render(&self, game: &mut Game, display: &mut Root) {
        let pos = game.player.pos;
        let level = game.cur_level_mut();
        let offset = pos - Point(display.width(), display.height()) / 2;
        for x in 0..display.width() {
            for y in 0..display.height() {
                let p = Point(x, y) + offset;
                if 0 <= p.0 && p.0 < level.width && 0 <= p.1 && p.1 < level.height {
                    if level.map.is_in_fov(p.0, p.1) {
                        level.seen[[p.0 as usize, p.1 as usize]] = true;
                        level.get(p.0, p.1).draw(Point(x, y), display);
                    } else if level.seen[[p.0 as usize, p.1 as usize]] {
                        display.put_char_ex(x, y, level.get(p.0, p.1).ch, DARK_GREY, BLACK);
                    }
                }
            }
        }
        game.player.draw(game.player.pos - offset, display);
    }
    fn handle(&self, game: &mut Game, key: Key) -> Action {
        use super::KeyCode::*;
        let mut pos = game.player.pos;
        match key {
            Key { printable: 'y', .. } | Key { code: NumPad7, .. } => pos += Point(-1, -1),
            Key { printable: 'k', .. } | Key { code: NumPad8, .. } | Key { code: Up, .. } => {
                pos += Point(0, -1)
            }
            Key { printable: 'u', .. } | Key { code: NumPad9, .. } => pos += Point(1, -1),
            Key { printable: 'h', .. } | Key { code: NumPad4, .. } | Key { code: Left, .. } => {
                pos += Point(-1, 0)
            }
            Key { printable: 'l', .. } | Key { code: NumPad6, .. } | Key { code: Right, .. } => {
                pos += Point(1, 0)
            }
            Key { printable: 'b', .. } | Key { code: NumPad1, .. } => pos += Point(-1, 1),
            Key { printable: 'j', .. } | Key { code: NumPad2, .. } | Key { code: Down, .. } => {
                pos += Point(0, 1)
            }
            Key { printable: 'n', .. } | Key { code: NumPad3, .. } => pos += Point(1, 1),
            Key { printable: 'x', .. } => println!("{:?}", pos),
            Key { printable: 'c', .. } => {
                let l = game.cur_level_mut();
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
            game.cur_level_mut()
                .map
                .compute_fov(pos.0, pos.1, PLAYER_FOV, true, Permissive2);
        }
        Action::Keep
    }
    fn enter(&self, game: &mut Game) {
        let pos = game.player.pos;
        game.cur_level_mut()
            .map
            .compute_fov(pos.0, pos.1, PLAYER_FOV, true, Permissive2);
    }
}
