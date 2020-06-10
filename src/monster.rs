use std::borrow::Borrow;
use std::collections::VecDeque;
use std::convert::TryInto;
use std::fmt;
use std::ops::Deref;
use std::rc::Rc;

use doryen_extra::random::{Dice, Rng};
use rand::seq::SliceRandom;
use rand::RngCore;
use serde::de::{Deserializer, Error, Visitor};
use serde_derive::Deserialize;

use crate::combat::AttackFlavor;
use crate::game::GameInfo;
use crate::map::Level;
use crate::point::Point;
use crate::tile::Tile;

#[derive(Debug, Deserialize, Clone)]
pub struct MonsterInfo {
    pub name: String,
    pub weight: f64,
    #[serde(flatten)]
    pub tile: Tile,
    pub health: i32,
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
    #[serde(deserialize_with = "de_die")]
    pub dam: Dice,
    pub class: String,
    pub text: Option<Vec<AttackFlavor>>,
}
fn de_die<'de, D: Deserializer<'de>>(de: D) -> Result<Dice, D::Error> {
    struct DVis;
    impl<'de> Visitor<'de> for DVis {
        type Value = Dice;
        fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
            write!(formatter, "a die")
        }
        fn visit_str<E: Error>(self, v: &str) -> Result<Dice, E> {
            Ok(Dice::new(v))
        }
    }
    de.deserialize_string(DVis)
}

#[derive(Debug)]
pub struct Monster {
    pub info: Rc<MonsterInfo>,
    pub pos: Point,
    pub hp: i32,
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
    fn is_player(&self) -> bool {
        false
    }
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
pub fn move_to<R>(
    idx: usize,
    dpos: Point,
    level: &mut Level,
    info: &GameInfo,
    log: &mut VecDeque<String>,
    rng: &mut R,
) -> bool
where
    R: Rng + RngCore,
{
    let pos = level.monsters[idx].get_pos() + dpos;
    if let Ok((ux, uy)) = pos.try_into() {
        if ux < level.width && uy < level.height {
            let tile = level.tiles.get(ux, uy);
            if tile.walkable {
                let minfo = level.monsters[idx].info.clone();
                for (i, mon) in level.monsters.iter_mut().enumerate() {
                    if i != idx && mon.pos == pos && mon.hp > 0 {
                        let attack = minfo.attacks.choose(rng).unwrap();
                        let damage = attack.dam.roll(rng);
                        let flavor = attack
                            .text
                            .as_ref()
                            .unwrap_or_else(|| &info.damage[&attack.class].attacks);
                        if idx == 0 {
                            let (pre, post) = &flavor.choose(rng).unwrap().player;
                            log.push_back("you".to_owned() + pre + "the " + &mon.info.name + post);
                        } else if i == 0 {
                            let (pre, post) = &flavor.choose(rng).unwrap().monster_p;
                            log.push_back("the ".to_owned() + &minfo.name + pre + "you" + post);
                        } else {
                            let (pre, post) = &flavor.choose(rng).unwrap().monster_p;
                            log.push_back(
                                "the ".to_owned()
                                    + &minfo.name
                                    + pre
                                    + "the "
                                    + &mon.info.name
                                    + post,
                            );
                        }
                        mon.hp -= damage;
                        if mon.hp <= 0 {
                            if i == 0 {
                                log.push_back(
                                    info.damage[&attack.class]
                                        .deaths
                                        .choose(rng)
                                        .unwrap()
                                        .player
                                        .clone(),
                                );
                            } else {
                                log.push_back(
                                    "the".to_owned()
                                        + &mon.info.name
                                        + &info.damage[&attack.class]
                                            .deaths
                                            .choose(rng)
                                            .unwrap()
                                            .monster,
                                );
                            }
                        }
                        return true;
                    }
                }
                level.monsters[idx].set_pos(pos);
                true
            } else if let Some(oname) = &tile.open {
                let otile = info.map.tiles[Borrow::<String>::borrow(oname)].clone();
                level.tiles.set(ux, uy, otile);
                true
            } else {
                false
            }
        } else {
            false
        }
    } else {
        false
    }
}
