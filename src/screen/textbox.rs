use doryen_rs::{Console, TextAlign};

use super::Screen;
use crate::game::Game;

pub struct TextBox {
    title: Option<String>,
    text: String,
    width: u32,
    height: u32,
    frame: bool,
}
impl TextBox {
    pub fn new(
        title: Option<String>,
        text: String,
        width: u32,
        height: u32,
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
    fn render(&self, _game: &mut Game, con: &mut Console) {
        let x = (con.get_width() - self.width) as i32 / 2;
        let y = (con.get_height() - self.height) as i32 / 2;
        if self.frame {
            con.rectangle(
                x - 1,
                y - 1,
                self.width + 2,
                self.height + 2,
                None,
                None,
                Some(' ' as u16),
            );
            if let Some(title) = &self.title {
                con.print_color(
                    con.get_width() as i32 / 2,
                    y - 1,
                    title,
                    TextAlign::Center,
                    None,
                );
            }
        }
        con.print_color(x, y, &self.text, TextAlign::Left, None);
    }
    fn transparent(&self) -> bool {
        true
    }
}
