use std::collections::HashMap;
use std::rc::Rc;

use doryen_rs::{Console, DoryenApi, Engine, FileLoader, TextAlign, UpdateEvent};

use crate::colors::*;
use crate::game::{Game, GameSettings};

pub mod game;
pub mod menu;
pub mod textbox;

pub struct WheatleyEngine {
    game: Option<Game>,
    menu: Rc<dyn Screen>,
    help: Rc<dyn Screen>,
    screens: Vec<Rc<dyn Screen>>,
    held_keys: HashMap<String, u32>,
    seed: u64,
    loader: FileLoader,
    settings_handle: usize,
}

impl WheatleyEngine {
    pub fn new(menu: Rc<dyn Screen>, help: Rc<dyn Screen>, seed: u64) -> Self {
        let mut loader = FileLoader::new();
        Self {
            game: None,
            menu,
            help,
            screens: Vec::new(),
            held_keys: HashMap::new(),
            seed,
            settings_handle: loader.load_file("settings.toml").unwrap(),
            loader,
        }
    }
}
impl Engine for WheatleyEngine {
    fn init(&mut self, api: &mut dyn DoryenApi) {
        let con = api.con();
        con.register_color("gray", GREY);
    }
    fn update(&mut self, api: &mut dyn DoryenApi) -> Option<UpdateEvent> {
        if let Some(ref mut game) = self.game {
            let input = api.input();

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
                            }
                            Action::Replace(s) => {
                                self.screens.pop().unwrap().exit(game);
                                s.enter(game);
                                self.screens.push(s.clone());
                            }
                            Action::Pop => {
                                self.screens.pop().unwrap().exit(game);
                            }
                        }
                    }
                }
            }
            for key in input.keys_released() {
                self.held_keys.remove(key);
            }

            let keys = &self.held_keys;
            let key_delay = game.settings.interface.key_delay;
            self.screens.last_mut().unwrap().handle_held(
                game,
                Box::new(|k| {
                    if let Some(i) = keys.get(k) {
                        *i == 0 || *i > key_delay
                    } else {
                        false
                    }
                }),
            );

            for i in self.held_keys.values_mut() {
                *i += 1;
            }

            if self.screens.is_empty() {
                Some(UpdateEvent::Exit)
            } else {
                None
            }
        } else {
            if self.loader.check_file_ready(0) {
                let settings: GameSettings =
                    toml::from_slice(&self.loader.get_file_content(self.settings_handle)).unwrap();

                api.set_font_path(&settings.interface.font);
                api.con().resize(
                    settings.interface.width,
                    settings.interface.height,
                );

                let mut game = Game::new(settings, self.menu.clone(), self.help.clone(), self.seed);
                let menu = game.menu.clone();
                menu.enter(&mut game);
                self.screens.push(menu);
                self.game = Some(game);
            }
            None
        }
    }
    fn render(&mut self, api: &mut dyn DoryenApi) {
        let con = api.con();
        if let Some(ref mut game) = self.game {
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
        } else {
            con.print(
                con.get_width() as i32 / 2,
                con.get_height() as i32 / 2,
                "Loading...",
                TextAlign::Center,
                None,
                None,
            );
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
