use std::cmp::max;
use std::cell::Cell;


use tcod::console::{ Root, Console, TextAlignment, BackgroundFlag };
use tcod::colors;
use tcod::input::{ Key, KeyCode };



use crate::screen::{ Screen, Action };
use crate::game::Game;

pub struct MenuScreen {
    title: String,
    options: Vec<(String, Action)>,
    active: Cell<usize>,
    hline: i32
}
impl MenuScreen {
    pub fn new(title: String, options: Vec<(String, Action)>) -> MenuScreen {
        let mut maxl = 0;
        for (opt, _) in &options {
            let l = opt.len();
            if l > maxl {
                maxl = l;
            }
        }
        let s = MenuScreen {
            title, options,
            active: Cell::new(0),
            hline: maxl as i32 + 4
        };
        return s;
    }
}

impl Screen for MenuScreen {
    fn render(&self, _game: &Game, display: &mut Root) {
        let x = display.width() / 2;
        let mut y = max(0, display.height() / 2 - self.title.lines().count() as i32 - 1
            - self.options.len() as i32 / 2);
        for line in self.title.lines() {
            display.print_ex(x, y,
                BackgroundFlag::None,
                TextAlignment::Center,
                line);
            y += 1;
        }
        y += 1;
        for (i, (opt, _)) in self.options.iter().enumerate() {
            if i == self.active.get() {
                display.horizontal_line(x-self.hline/2, y, self.hline, BackgroundFlag::None);
                display.print_ex(x, y,
                    BackgroundFlag::None,
                    TextAlignment::Center,
                    opt);
                display.put_char_ex(x - 1 - (opt.len() as i32)/2, y, '[', colors::GREY, colors::BLACK);
                display.put_char_ex(x + (1 + opt.len() as i32)/2, y, ']', colors::GREY, colors::BLACK);
            } else {
                display.print_ex(x, y,
                    BackgroundFlag::None,
                    TextAlignment::Center,
                    opt);
            }
            y += 1
        }
    }
    fn handle(&self, _game: &mut Game, key: Key) -> Action {
        match key {
            Key { code: KeyCode::Down, .. } => {
                self.active.set((self.active.get() + 1) % self.options.len())
            }
            Key { code: KeyCode::Up, .. } => {
                let active = self.active.get();
                if active == 0 {
                    self.active.set(self.options.len() - 1);
                } else {
                    self.active.set(active - 1)
                }
            }
            Key {code: KeyCode::Enter, .. } => {
                return self.options[self.active.get()].1.clone()
            }
            Key { code: KeyCode::Escape, .. } => {
                return Action::Pop
            }
            _ => { }
        }
        Action::Keep
    }
}