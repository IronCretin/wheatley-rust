use rand::seq::SliceRandom;
use rand::Rng;
use rand_distr::{Distribution, Triangular, Uniform};

use super::Generator;
use crate::Game;
use crate::map::Level;

pub struct Hallways {
    depth: i32,
    minr: i32,
}
impl Hallways {
    pub fn new(depth: i32, minr: i32) -> Hallways {
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
            level.width - 1,
            1,
            level.height - 1,
            game,
        );
    }
}

fn create(
    level: &mut Level,
    depth: i32,
    minr: i32,
    xaxis: bool,
    mut x0: i32,
    mut x1: i32,
    mut y0: i32,
    mut y1: i32,
    game: &mut Game,
) {
    let rng = &mut game.map_rng;
    let door = &game.map.tiles["door"];
    let floor = &game.map.tiles["floor"];

    let hw = depth / 2 + 1;
    if x0 + 1 + minr > x1 - hw - minr || y0 + 1 + minr > y1 - hw - minr {
        return;
    }
    if depth > 0 {
        // hallways
        if xaxis {
            // x axis
            let (xmin, xmax) = ((x0 + 1 + minr) as f64, (x1 - hw - minr) as f64);
            let x: i32 = Triangular::new(xmin, xmax, (xmin + xmax) / 2.0)
                .unwrap()
                .sample(rng) as i32;
            let (doors1, doors2): (bool, bool) = rng.gen();

            for i in 0..hw {
                level.set(x + i, y0 - 1, if doors1 { door.clone() } else { floor.clone() });
                level.set(x + i, y1, if doors2 { door.clone() } else { floor.clone() });
            }
            for y in y0..y1 {
                for j in 0..hw {
                    level.set(x + j, y, floor.clone());
                }
            }
            create(level, depth - 1, minr, !xaxis, x0, x - 1, y0, y1, game);
            create(level, depth - 1, minr, !xaxis, x + hw + 1, x1, y0, y1, game);
        } else {
            // y axis
            let (ymin, ymax) = ((y0 + 1 + minr) as f64, (y1 - hw - minr) as f64);
            let y: i32 = Triangular::new(ymin, ymax, (ymin + ymax) / 2.0)
                .unwrap()
                .sample(rng) as i32;
            let (doors1, doors2): (bool, bool) = rng.gen();

            for i in 0..hw {
                level.set(x0 - 1, y + i, if doors1 { door.clone() } else { floor.clone() });
                level.set(x1, y + i, if doors2 { door.clone() } else { floor.clone() });
            }
            for x in x0..x1 {
                for j in 0..hw {
                    level.set(x, y + j, floor.clone());
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
                    if y0 != 1 && y1 - y0 >= minr {
                        let mut x = x0;
                        let mut height = 0;
                        while x1 - x >= minr {
                            let (w, h) = create_room(
                                level,
                                x,
                                y0,
                                (1, 0),
                                (0, 1),
                                minr,
                                x1 - x,
                                minr,
                                y1 - y0,
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
                    if y0 != level.height - 2 && y1 - y0 >= minr {
                        let mut x = x0;
                        let mut height = 0;
                        while x1 - x >= minr {
                            let (w, h) = create_room(
                                level,
                                x,
                                y1 - 1,
                                (1, 0),
                                (0, -1),
                                minr,
                                x1 - x,
                                minr,
                                y1 - y0,
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
                    if x0 != 1 && x1 - x0 >= minr {
                        let mut y = y0;
                        let mut height = 0;
                        while y1 - y >= minr {
                            let (w, h) = create_room(
                                level,
                                x0,
                                y,
                                (0, 1),
                                (1, 0),
                                minr,
                                y1 - y,
                                minr,
                                x1 - x0,
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
                    if x0 != level.height - 2 && x1 - x0 >= minr {
                        let mut y = y0;
                        let mut height = 0;
                        while y1 - y >= minr {
                            let (w, h) = create_room(
                                level,
                                x1 - 1,
                                y,
                                (0, 1),
                                (-1, 0),
                                minr,
                                y1 - y,
                                minr,
                                x1 - x0,
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
    x0: i32,
    y0: i32,
    dx: (i32, i32),
    dy: (i32, i32),
    xmin: i32,
    xmax: i32,
    ymin: i32,
    ymax: i32,
    game: &mut Game,
) -> (i32, i32) {
    let rng = &mut game.map_rng;
    let door = &game.map.tiles["door"];
    let floor = &game.map.tiles["floor"];


    let mut place = |x: i32, y: i32, t| {
        if x0 + x * dx.0 + y * dy.0 < 0 || y0 + x * dx.1 + y * dy.1 < 0 {
            println!(
                "tried to draw {},{} (really {},{})",
                x,
                y,
                x0 + x * dx.0 + y * dy.0,
                y0 + x * dx.1 + y * dy.1
            );
        }
        level.set(x0 + x * dx.0 + y * dy.0, y0 + x * dx.1 + y * dy.1, t);
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
    (w, h)
}

#[derive(Debug)]
enum Side {
    Top,
    Bottom,
    Left,
    Right,
}
