use tcod::console::{Console, Root};
use tcod::input::Key;

use super::{handle_default, Action, Screen};
use crate::game::Game;
use crate::point::Point;

pub struct GameScreen;

impl Screen for GameScreen {
    fn render(&self, game: &Game, display: &mut Root) {
        let level = game.cur_level();
        let offset = game.player.pos - Point(display.width(), display.height()) / 2;
        for x in 0..display.width() {
            for y in 0..display.height() {
                let p = Point(x, y) + offset;
                if 0 <= p.0 && p.0 < level.width && 0 <= p.1 && p.1 < level.height {
                    level.get(p.0, p.1).draw(Point(x, y), display);
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
        }
        Action::Keep
    }
}
