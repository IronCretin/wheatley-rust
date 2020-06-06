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
}

impl WheatleyEngine {
    pub fn new(game: Game) -> WheatleyEngine {
        Self {
            game,
            screens: Vec::new(),
        }
    }
}
impl Engine for WheatleyEngine {
    fn init(&mut self, api: &mut dyn DoryenApi) {
        let con = api.con();
        con.register_color("gray", GREY);

        let menu = self.game.menu.clone();
        menu.enter(&mut self.game);
        self.screens.push(menu);
    }
    fn update(&mut self, api: &mut dyn DoryenApi) -> Option<UpdateEvent> {
        let input = api.input();

        self.screens
            .last_mut()
            .unwrap()
            .handle_held(&mut self.game, Box::new(|k| input.key(k)));

        for key in input.keys_pressed() {
            match key {
                "ControlLeft" | "AltLeft" | "ShiftLeft" | "ShiftRight" => {}
                _ => {
                    let action = self.screens.last_mut().unwrap().handle(
                        &mut self.game,
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
                            s.enter(&mut self.game);
                            self.screens.push(s.clone());
                        }
                        Action::Replace(s) => {
                            self.screens.pop().unwrap().exit(&mut self.game);
                            s.enter(&mut self.game);
                            self.screens.push(s.clone());
                        }
                        Action::Pop => {
                            self.screens.pop().unwrap().exit(&mut self.game);
                        }
                    }
                }
            }
        }
        if self.screens.is_empty() {
            Some(UpdateEvent::Exit)
        } else {
            None
        }
    }
    fn render(&mut self, api: &mut dyn DoryenApi) {
        let con = api.con();
        con.clear(None, None, Some(' ' as u16));

        let mut bottom = 0;
        for (i, s) in self.screens.iter().enumerate() {
            if !s.transparent() {
                bottom = i;
            }
        }
        for s in &self.screens[bottom..] {
            s.render(&mut self.game, con);
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
        } => {
            println!("help");
            Push(game.help.clone())
        }
        _ => Keep,
    }
}
