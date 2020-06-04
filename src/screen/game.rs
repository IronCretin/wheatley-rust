use tcod::colors::*;
use tcod::console::{Console, Root};
use tcod::input::Key;
use tcod::map::FovAlgorithm::Basic;

use super::{handle_default, Action, Screen};
use crate::game::Game;
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
            _ => return handle_default(game, key),
        }
        let l = game.cur_level();
        if 0 <= pos.0
            && pos.0 < l.width
            && 0 <= pos.1
            && pos.1 < l.height
            && l.get(pos.0, pos.1).walkable
        {
            game.player.pos = pos;
            game.cur_level_mut()
                .map
                .compute_fov(pos.0, pos.1, PLAYER_FOV, true, Basic);
        }
        Action::Keep
    }
    fn enter(&self, game: &mut Game) {
        let pos = game.player.pos;
        game.cur_level_mut()
            .map
            .compute_fov(pos.0, pos.1, PLAYER_FOV, true, Basic);
    }
}
