use tcod::console::{Console, Root};
use tcod::input::Key;

use super::{handle_default, Action, Screen};
use crate::game::Game;
use crate::point::Point;

pub struct GameScreen;

impl Screen for GameScreen {
    fn render(&self, game: &Game, display: &mut Root) {
        let level = game.cur_level();
        let offset = Point(display.width(), display.height()) / 2 - game.player.pos;
        for x in 0..display.width() {
            for y in 0..display.height() {
                if 0 <= x && x < level.width && 0 <= y && y < level.height {
                    level.get(x, y).draw(Point(x, y) + offset, display);
                }
            }
        }
        game.player.draw(game.player.pos + offset, display);
    }
    fn handle(&self, game: &mut Game, key: Key) -> Action {
        use super::KeyCode::*;
        match key {
            Key { code: Up, .. } => game.player.pos.1 -= 1,
            Key { code: Down, .. } => game.player.pos.1 += 1,
            Key { code: Left, .. } => game.player.pos.0 -= 1,
            Key { code: Right, .. } => game.player.pos.0 += 1,
            _ => return handle_default(game, key),
        }
        Action::Keep
    }
}
