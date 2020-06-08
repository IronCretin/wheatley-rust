use std::ops::Deref;
use std::rc::Rc;

use serde_derive::Deserialize;

use crate::combat::AttackFlavor;
use crate::point::Point;
use crate::tile::Tile;

#[derive(Debug, Deserialize, Clone)]
pub struct MonsterInfo {
    pub name: String,
    pub weight: f64,
    #[serde(flatten)]
    pub tile: Tile,
    pub health: u32,
    pub attacks: Vec<Attack>,
    #[serde(default)]
    pub friendly: bool,
}
impl Deref for MonsterInfo {
    type Target = Tile;
    fn deref(&self) -> &Tile {
        &self.tile
    }
}

#[derive(Debug, Deserialize, Clone)]
pub struct Attack {
    pub dam: String,
    pub class: String,
    pub text: Option<Rc<AttackFlavor>>,
}

#[derive(Debug)]
pub struct Monster {
    pub info: Rc<MonsterInfo>,
    pub pos: Point,
    pub hp: u32,
}
impl Deref for Monster {
    type Target = MonsterInfo;
    fn deref(&self) -> &MonsterInfo {
        &self.info
    }
}
