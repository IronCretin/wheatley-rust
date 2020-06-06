use std::cell::Cell;
use std::cmp::max;

use doryen_rs::{Console, TextAlign};

use crate::colors::*;
use crate::game::Game;
use crate::screen::{handle_default, Action, Key, Screen};

pub struct MenuScreen {
    title: String,
    options: Vec<(String, Action)>,
    active: Cell<usize>,
}
impl MenuScreen {
    pub fn new(title: String, options: Vec<(String, Action)>) -> MenuScreen {
        MenuScreen {
            title,
            options,
            active: Cell::new(0),
        }
    }
}

impl Screen for MenuScreen {
    fn render(&self, _game: &mut Game, console: &mut Console) {
        let x = console.get_width() as i32 / 2;
        let mut y = max(
            0,
            console.get_height() as i32 / 2
                - self.title.lines().count() as i32
                - 1
                - self.options.len() as i32 / 2,
        );
        for line in self.title.lines() {
            console.print_color(x, y, line, TextAlign::Center, None);
            y += 1;
        }
        y += 1;
        for (i, (opt, _)) in self.options.iter().enumerate() {
            console.print_color(x, y, opt, TextAlign::Center, None);
            if i == self.active.get() {
                console.cell(
                    x - 1 - (opt.len() as i32) / 2,
                    y,
                    Some('[' as u16),
                    Some(GREY),
                    None,
                );
                console.cell(
                    x + (1 + opt.len() as i32) / 2,
                    y,
                    Some(']' as u16),
                    Some(GREY),
                    None,
                );
            }
            y += 1
        }
    }
    fn handle(&self, game: &mut Game, key: Key) -> Action {
        match key {
            Key {
                key: "ArrowDown", ..
            } => self
                .active
                .set((self.active.get() + 1) % self.options.len()),
            Key { key: "ArrowUp", .. } => {
                let active = self.active.get();
                if active == 0 {
                    self.active.set(self.options.len() - 1);
                } else {
                    self.active.set(active - 1)
                }
            }
            Key { key: "Enter", .. }
            | Key {
                key: "NumpadEnter", ..
            } => return self.options[self.active.get()].1.clone(),
            _ => return handle_default(game, key),
        }
        Action::Keep
    }
}
