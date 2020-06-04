use std::rc::Rc;

use crate::screen::Screen;

pub struct Game {
    pub menu: Rc<dyn Screen>,
    pub help: Rc<dyn Screen>,
}

impl Game {
    pub fn new(menu: Rc<dyn Screen>, help: Rc<dyn Screen>) -> Game {
        Game { menu, help }
    }
}
