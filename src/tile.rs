use std::fmt;

use doryen_rs::{Color, Console};
use serde::de::{Deserializer, Error, Visitor};
use serde_derive::Deserialize;

use crate::point::Point;

#[derive(Copy, Clone, Debug, Deserialize)]
pub struct Tile {
    #[serde(deserialize_with = "ch_to_u16")]
    pub ch: u16,
    pub fg: Color,
    #[serde(default = "black")]
    pub bg: Color,
}
impl Tile {
    pub fn draw(&self, p: Point, con: &mut Console) {
        con.cell(p.0, p.1, Some(self.ch), Some(self.fg), Some(self.bg))
    }
}

fn ch_to_u16<'de, D: Deserializer<'de>>(de: D) -> Result<u16, D::Error> {
    struct V;
    impl<'de> Visitor<'de> for V {
        type Value = u16;
        fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
            write!(formatter, "a character")
        }
        fn visit_str<E: Error>(self, v: &str) -> Result<u16, E> {
            v.encode_utf16()
                .next()
                .ok_or_else(|| E::custom(format!("could not encode char as u16: {}", v)))
        }
    }
    de.deserialize_char(V)
}
fn black() -> (u8, u8, u8, u8) {
    (0, 0, 0, 255)
}
