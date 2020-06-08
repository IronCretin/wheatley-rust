use std::ops::Deref;
use std::rc::Rc;
use std::convert::TryInto;
use std::borrow::Borrow;

use serde_derive::Deserialize;

use crate::combat::AttackFlavor;
use crate::point::Point;
use crate::tile::Tile;
use crate::game::GameInfo;
use crate::map::Level;

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

pub trait Creature {
    fn get_pos(&self) -> Point;
    fn set_pos(&mut self, pos: Point);
}
impl Creature for Monster {
    fn get_pos(&self) -> Point {
        self.pos
    }
    fn set_pos(&mut self, pos: Point) {
        self.pos = pos
    }
}

// find a way to express this type signature, its awkward to pass the parts separately
pub fn move_to<C: Creature>(creature: &mut C, dpos: Point, level: &mut Level, info: &GameInfo) -> bool {
    let pos = creature.get_pos() + dpos;
    let (ux, uy) = pos.try_into().unwrap();
    if ux < level.width && uy < level.height {
        let tile = level.get(ux, uy);
        if tile.walkable {
            creature.set_pos(pos);
            true
        } else if let Some(oname) = &tile.open {
            let otile = info.map.tiles[Borrow::<String>::borrow(oname)].clone();
            level.set(ux, uy, otile);
            true
        } else {
            false
        }
    } else {
        false
    }
}
