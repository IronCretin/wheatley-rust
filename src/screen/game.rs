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
            Key { code: Up, .. } => pos += Point(0, -1),
            Key { code: Down, .. } => pos += Point(0, 1),
            Key { code: Left, .. } => pos += Point(-1, 0),
            Key { code: Right, .. } => pos += Point(1, 0),
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
