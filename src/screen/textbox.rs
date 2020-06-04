use tcod::console::{BackgroundFlag, Console, Root};

use super::Screen;
use crate::game::Game;

pub struct TextBox {
    title: Option<String>,
    text: String,
    width: i32,
    height: i32,
    frame: bool,
}
impl TextBox {
    pub fn new(
        title: Option<String>,
        text: String,
        width: i32,
        height: i32,
        frame: bool,
    ) -> TextBox {
        TextBox {
            title,
            text,
            width,
            height,
            frame,
        }
    }
}

impl Screen for TextBox {
    fn render(&self, _game: &Game, display: &mut Root) {
        let x = (display.width() - self.width) / 2;
        let y = (display.height() - self.height) / 2;
        if self.frame {
            display.print_frame(
                x - 1,
                y - 1,
                self.width + 2,
                self.height + 2,
                true,
                BackgroundFlag::Set,
                self.title.as_ref(),
            );
        }
        display.print_rect(x, y, self.width + 2, self.height, &self.text);
    }
    fn transparent(&self) -> bool {
        true
    }
}
