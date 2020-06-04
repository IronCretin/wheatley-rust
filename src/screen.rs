use std::rc::Rc;


use tcod::console::{ Root, Console };
use tcod::input::{ Key, KeyCode };
use tcod::colors::WHITE;

use crate::game::Game;

pub mod menu;
pub mod textbox;

pub struct ScreenStack {
    display: Root,
    screens: Vec<Rc<dyn Screen>>
}

impl ScreenStack {
    pub fn new(display: Root) -> ScreenStack {
        ScreenStack {
            display,
            screens: Vec::new()
        }
    }
    pub fn play(mut self, game: &mut Game, screen: Box<dyn Screen>) {
        screen.enter(game);
        self.screens.push(Rc::from(screen));
        while !self.display.window_closed() && !self.screens.is_empty() {
            self.render(&game);
            let key = self.display.wait_for_keypress(true);
            let act = self.screens
                        .last_mut()
                        .unwrap()
                        .handle(game, key);
            match act {
                Action::Keep => {}
                Action::Push(s) => {
                    self.screens.push(s.clone());
                }
                Action::Pop => {
                    self.screens.pop();
                }
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
}

pub trait Screen {
    fn enter(&self, _game: &mut Game) { }
    fn exit(&self, _game: &mut Game) { }
    fn render(&self, game: &Game, display: &mut Root);
    fn handle(&self, _game: &mut Game, key: Key) -> Action {
        match key {
            Key { code: KeyCode::Escape, .. } => {
                Action::Pop
            }
            _ => {
                Action::Keep
            }
        }
    }
    fn transparent(&self) -> bool { false }
}
