use std::rc::Rc;

use tcod::colors::WHITE;
use tcod::console::{Console, Root};
use tcod::input::{Key, KeyCode};

use crate::game::Game;

pub mod menu;
pub mod textbox;

pub struct ScreenStack {
    display: Root,
    screens: Vec<Rc<dyn Screen>>,
}

impl ScreenStack {
    pub fn new(display: Root) -> ScreenStack {
        ScreenStack {
            display,
            screens: Vec::new(),
        }
    }
    pub fn play(mut self, game: &mut Game) {
        game.menu.enter(game);
        self.screens.push(game.menu.clone());
        while !self.display.window_closed() && !self.screens.is_empty() {
            self.render(&game);
            let key = self.display.wait_for_keypress(true);
            let screen = self.screens.last_mut().unwrap();
            let act = screen.handle(game, key);
            match act {
                Action::Keep => {}
                Action::Push(s) => {
                    self.screens.push(s.clone());
                }
                Action::Pop => {
                    self.screens.pop().unwrap().exit(game);
                }
                Action::Help => self.screens.push(game.help.clone()),
            }
        }
    }
    fn render(&mut self, game: &Game) {
        self.display.set_default_foreground(WHITE);
        self.display.clear();
        let mut bottom = 0;
        for (i, s) in self.screens.iter().enumerate() {
            if !s.transparent() {
                bottom = i;
            }
        }
        for s in &self.screens[bottom..] {
            s.render(game, &mut self.display);
        }
        self.display.flush();
    }
}

#[derive(Clone)]
pub enum Action {
    Keep,
    Pop,
    Push(Rc<dyn Screen>),
    Help,
}

pub trait Screen {
    fn enter(&self, _game: &Game) {}
    fn exit(&self, _game: &Game) {}
    fn render(&self, game: &Game, display: &mut Root);
    fn handle(&self, _game: &mut Game, key: Key) -> Action {
        use Action::*;
        use KeyCode::*;
        match key {
            Key { code: Escape, .. } => Pop,
            Key {
                code: Char,
                shift: true,
                printable: '/',
                ..
            } => Help,
            _ => Keep,
        }
    }
    fn transparent(&self) -> bool {
        false
    }
}
