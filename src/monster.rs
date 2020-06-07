use std::rc::Rc;

use serde_derive::Deserialize;

use crate::combat::AttackFlavor;
use crate::tile::Tile;

#[derive(Debug, Deserialize, Clone)]
pub struct MonsterInfo {
    name: String,
    #[serde(flatten)]
    tile: Tile,
    health: u32,
    attacks: Vec<Attack>,
    #[serde(default)]
    friendly: bool,
}

#[derive(Debug, Deserialize, Clone)]
pub struct Attack {
    pub dam: String,
    pub class: String,
    pub text: Option<Rc<AttackFlavor>>,
}

pub struct Monster {
    pub info: Rc<MonsterInfo>,
    pub health: u32,
}
