use std::collections::HashMap;
use std::rc::Rc;

use doryen_rs::{Console, DoryenApi, Engine, UpdateEvent};

use crate::colors::*;
use crate::game::Game;

pub mod game;
pub mod menu;
pub mod textbox;

pub struct WheatleyEngine {
    game: Game,
    screens: Vec<Rc<dyn Screen>>,
    held_keys: HashMap<String, u32>,
}

impl WheatleyEngine {
    pub fn new(mut game: Game) -> Self {
        let menu = game.menu.clone();
        menu.enter(&mut game);
        Self {
            game,
            screens: vec![menu],
            held_keys: HashMap::new(),
        }
    }
}
impl Engine for WheatleyEngine {
    fn init(&mut self, api: &mut dyn DoryenApi) {
        let con = api.con();
        con.register_color("gray", GREY);
    }
    fn update(&mut self, api: &mut dyn DoryenApi) -> Option<UpdateEvent> {
        let game = &mut self.game;
        let input = api.input();

        let mut clear = false;
        for key in input.keys_pressed() {
            if !self.held_keys.contains_key(key) {
                self.held_keys.insert(key.to_owned(), 0);
            }
            match key {
                "ControlLeft" | "AltLeft" | "ShiftLeft" | "ShiftRight" => {}
                _ => {
                    let action = self.screens.last_mut().unwrap().handle(
                        game,
                        Key {
                            key,
                            ctrl: input.key("ControlLeft"),
                            alt: input.key("AltLeft"),
                            shift: input.key("ShiftLeft") | input.key("ShiftRight"),
                        },
                    );
                    match action {
                        Action::Keep => {}
                        Action::Push(s) => {
                            s.enter(game);
                            self.screens.push(s.clone());
                            clear = true;
                        }
                        Action::Replace(s) => {
                            self.screens.pop().unwrap().exit(game);
                            s.enter(game);
                            self.screens.push(s.clone());
                            clear = true;
                        }
                        Action::Pop => {
                            if !cfg!(target_arch = "wasm32") || self.screens.len() != 1 {
                                self.screens.pop().unwrap().exit(game);
                                clear = true;
                            }
                        }
                    }
                }
            }
        }
        if clear {
            api.con().clear(
                Some((255, 255, 255, 255)),
                Some((0, 0, 0, 255)),
                Some(' ' as u16),
            );
        }

        for key in api.input().keys_released() {
            self.held_keys.remove(key);
        }

        let keys = &self.held_keys;
        let key_delay = game.info.settings.interface.key_delay;
        self.screens.last_mut().map(|s| {
            s.handle_held(
                game,
                Box::new(|k| {
                    if let Some(i) = keys.get(k) {
                        *i == 0 || *i > key_delay
                    } else {
                        false
                    }
                }),
            )
        });

        for i in self.held_keys.values_mut() {
            *i += 1;
        }

        if self.screens.is_empty() {
            Some(UpdateEvent::Exit)
        } else {
            None
        }
    }
    fn render(&mut self, api: &mut dyn DoryenApi) {
        let con = api.con();
        let game = &mut self.game;
        con.clear(None, None, Some(' ' as u16));

        let mut bottom = 0;
        for (i, s) in self.screens.iter().enumerate() {
            if !s.transparent() {
                bottom = i;
            }
        }
        for s in &self.screens[bottom..] {
            s.render(game, con);
        }
    }
}

#[derive(Clone)]
pub enum Action {
    Keep,
    Pop,
    Push(Rc<dyn Screen>),
    Replace(Rc<dyn Screen>),
}

#[allow(dead_code)]
#[derive(Debug)]
pub struct Key<'a> {
    key: &'a str,
    ctrl: bool,
    alt: bool,
    shift: bool,
}

pub trait Screen {
    fn enter(&self, _game: &mut Game) {}
    fn exit(&self, _game: &mut Game) {}
    fn render(&self, game: &mut Game, display: &mut Console);
    fn handle(&self, game: &mut Game, key: Key) -> Action {
        handle_default(game, key)
    }
    fn handle_held<'a>(&self, _game: &mut Game, _held: Box<dyn Fn(&str) -> bool + 'a>) {}
    fn transparent(&self) -> bool {
        false
    }
}

pub fn handle_default(game: &Game, key: Key) -> Action {
    use Action::*;
    match key {
        Key { key: "Escape", .. } => Pop,
        Key {
            key: "NumpadDivide",
            shift: true,
            ..
        } => Push(game.help.clone()),
        _ => Keep,
    }
}
