use rand::distributions::WeightedIndex;
use rand::seq::SliceRandom;
use rand::Rng;
use rand_distr::{Distribution, Triangular, Uniform};

use super::Generator;
use crate::map::Level;
use crate::monster::Monster;
use crate::point::Point;
use crate::Game;

pub struct Hallways {
    minr: i32,
    depth: usize,
}
impl Hallways {
    pub fn new(depth: usize, minr: i32) -> Hallways {
        Hallways { depth, minr }
    }
}

impl Generator for Hallways {
    fn generate(&self, game: &mut Game, level: &mut Level) {
        create(
            level,
            self.depth,
            self.minr,
            game.map_rng.gen(),
            1,
            level.width as usize - 1,
            1,
            level.height as usize - 1,
            game,
        );
        let px = Uniform::from(0..level.width);
        let py = Uniform::from(0..level.height);
        let mon_names: Vec<&String> = game.info.monster.keys().collect();
        let dist =
            WeightedIndex::new(mon_names.iter().map(|n| game.info.monster[*n].weight)).unwrap();
        for _ in 0..game.info.settings.map.num_monsters {
            for _ in 0..game.info.settings.map.place_attempts {
                let x = px.sample(&mut game.map_rng);
                let y = py.sample(&mut game.map_rng);
                if level.tiles.get(x, y).walkable {
                    let name = mon_names[dist.sample(&mut game.map_rng)];
                    let info = game.info.monster[name].clone();
                    level.monsters.push(Monster {
                        pos: Point(x as i32, y as i32),
                        hp: info.health,
                        info,
                    });
                    break;
                }
            }
        }
    }
}

fn create(
    level: &mut Level,
    depth: usize,
    minr: i32,
    xaxis: bool,
    mut x0: usize,
    mut x1: usize,
    mut y0: usize,
    mut y1: usize,
    game: &mut Game,
) {
    let rng = &mut game.map_rng;
    let door = &game.info.map.tiles["door"];
    let floor = &game.info.map.tiles["floor"];
    // let uminr = minsize as usize;

    let hw = depth / 2 + 1;
    if (x1 > hw && x0 + 1 > x1 - hw) || (y1 > hw && y0 + 1 > y1 - hw) {
        return;
    }
    if depth > 0 {
        // hallways
        if xaxis {
            // x axis
            let (xmin, xmax) = ((x0 + 1) as f64, (x1 - hw) as f64);
            let x: usize = Triangular::new(xmin, xmax, (xmin + xmax) / 2.0)
                .unwrap()
                .sample(rng) as usize;
            let (doors1, doors2): (bool, bool) = rng.gen();

            for i in 0..hw {
                level.tiles.set(
                    x + i,
                    y0 - 1,
                    if doors1 { door.clone() } else { floor.clone() },
                );
                level
                    .tiles
                    .set(x + i, y1, if doors2 { door.clone() } else { floor.clone() });
            }
            for y in y0..y1 {
                for j in 0..hw {
                    level.tiles.set(x + j, y, floor.clone());
                }
            }
            create(level, depth - 1, minr, !xaxis, x0, x - 1, y0, y1, game);
            create(level, depth - 1, minr, !xaxis, x + hw + 1, x1, y0, y1, game);
        } else {
            // y axis
            let (ymin, ymax) = ((y0 + 1) as f64, (y1 - hw) as f64);
            let y: usize = Triangular::new(ymin, ymax, (ymin + ymax) / 2.0)
                .unwrap()
                .sample(rng) as usize;
            let (doors1, doors2): (bool, bool) = rng.gen();

            for i in 0..hw {
                level.tiles.set(
                    x0 - 1,
                    y + i,
                    if doors1 { door.clone() } else { floor.clone() },
                );
                level
                    .tiles
                    .set(x1, y + i, if doors2 { door.clone() } else { floor.clone() });
            }
            for x in x0..x1 {
                for j in 0..hw {
                    level.tiles.set(x, y + j, floor.clone());
                }
            }
            create(level, depth - 1, minr, !xaxis, x0, x1, y0, y - 1, game);
            create(level, depth - 1, minr, !xaxis, x0, x1, y + hw + 1, y1, game);
        }
    } else {
        // rooms
        use Side::*;
        // if rng.gen_bool(0.1) {
        //     // empty block
        //     return;
        // }
        let nsides = Uniform::new_inclusive(3, 4).sample(rng);
        for side in [Top, Bottom, Left, Right].choose_multiple(rng, nsides) {
            match side {
                Top => {
                    if y0 != 1 && (y1 as i32 - y0 as i32) >= minr {
                        let mut x = x0;
                        let mut height = 0;
                        while (x1 as i32 - x as i32) >= minr {
                            let (w, h) = create_room(
                                level,
                                x,
                                y0,
                                (1, 0),
                                (0, 1),
                                minr,
                                (x1 - x) as i32,
                                minr,
                                (y1 - y0) as i32,
                                game,
                            );
                            x += w + 1;
                            if h > height {
                                height = h;
                            }
                        }
                        y0 += height + 1;
                    }
                }
                Bottom => {
                    if y0 != level.height - 2 && (y1 as i32 - y0 as i32) >= minr {
                        let mut x = x0;
                        let mut height = 0;
                        while (x1 as i32 - x as i32) >= minr {
                            let (w, h) = create_room(
                                level,
                                x,
                                y1 - 1,
                                (1, 0),
                                (0, -1),
                                minr,
                                (x1 - x) as i32,
                                minr,
                                (y1 - y0) as i32,
                                game,
                            );
                            x += w + 1;
                            if h > height {
                                height = h;
                            }
                        }
                        y1 -= height + 1;
                    }
                }
                Left => {
                    if x0 != 1 && (x1 as i32 - x0 as i32) >= minr {
                        let mut y = y0;
                        let mut height = 0;
                        while (y1 as i32 - y as i32) >= minr {
                            let (w, h) = create_room(
                                level,
                                x0,
                                y,
                                (0, 1),
                                (1, 0),
                                minr,
                                (y1 - y) as i32,
                                minr,
                                (x1 - x0) as i32,
                                game,
                            );
                            y += w + 1;
                            if h > height {
                                height = h;
                            }
                        }
                        x0 += height + 1;
                    }
                }
                Right => {
                    if x0 != level.height - 2 && (x1 as i32 - x0 as i32) >= minr {
                        let mut y = y0;
                        let mut height = 0;
                        while (y1 as i32 - y as i32) >= minr {
                            let (w, h) = create_room(
                                level,
                                x1 - 1,
                                y,
                                (0, 1),
                                (-1, 0),
                                minr,
                                (y1 - y) as i32,
                                minr,
                                (x1 - x0) as i32,
                                game,
                            );
                            y += w + 1;
                            if h > height {
                                height = h;
                            }
                        }
                        x1 -= height + 1;
                    }
                }
            }
        }
    }
}
fn create_room(
    level: &mut Level,
    x0: usize,
    y0: usize,
    dx: (i32, i32),
    dy: (i32, i32),
    xmin: i32,
    xmax: i32,
    ymin: i32,
    ymax: i32,
    game: &mut Game,
) -> (usize, usize) {
    let rng = &mut game.map_rng;
    let door = &game.info.map.tiles["door"];
    let floor = &game.info.map.tiles["floor"];

    let mut place = |x: i32, y: i32, t| {
        level.tiles.set(
            (x0 as i32 + x * dx.0 + y * dy.0) as usize,
            (y0 as i32 + x * dx.1 + y * dy.1) as usize,
            t,
        );
    };
    let w = if xmin > xmax / 2 {
        xmax
    } else {
        Uniform::new_inclusive(xmin, xmax).sample(rng)
    };
    let h = if ymin > ymax / 2 { ymax } else { ymin };
    for x in 0..w {
        for y in 0..h {
            place(x, y, floor.clone());
        }
    }
    let dx = *[0, w - 1].choose(rng).unwrap();
    place(dx, -1, door.clone());
    (w as usize, h as usize)
}

#[derive(Debug)]
enum Side {
    Top,
    Bottom,
    Left,
    Right,
}
