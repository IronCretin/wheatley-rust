use rand::Rng;
use rand_distr::{Distribution, Triangular};

use super::{Empty, Generator};
use crate::map::tile::*;
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
    fn generate<R: Rng>(&self, rng: &mut R, level: &mut Level) {
        create(
            level,
            self.depth,
            self.minr,
            true, // rng.gen(),
            1,
            level.width - 1,
            1,
            level.height - 1,
            rng,
        );
        Empty.generate(rng, level);
    }
}

fn create<R: Rng>(
    level: &mut Level,
    depth: i32,
    minr: i32,
    xaxis: bool,
    x0: i32,
    x1: i32,
    y0: i32,
    y1: i32,
    rng: &mut R,
) {
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
                level.set(x + i, y0 - 1, if doors1 { DOOR } else { FLOOR });
                level.set(x + i, y1, if doors2 { DOOR } else { FLOOR });
            }
            for y in y0..y1 {
                for j in 0..hw {
                    level.set(x + j, y, FLOOR);
                }
            }
            create(level, depth - 1, minr, !xaxis, x0, x - 1, y0, y1, rng);
            create(level, depth - 1, minr, !xaxis, x + hw + 1, x1, y0, y1, rng);
        } else {
            // y axis
            let (ymin, ymax) = ((y0 + 1 + minr) as f64, (y1 - hw - minr) as f64);
            let y: i32 = Triangular::new(ymin, ymax, (ymin + ymax) / 2.0)
                .unwrap()
                .sample(rng) as i32;
            let (doors1, doors2): (bool, bool) = rng.gen();

            for i in 0..hw {
                level.set(x0 - 1, y + i, if doors1 { DOOR } else { FLOOR });
                level.set(x1, y + i, if doors2 { DOOR } else { FLOOR });
            }
            for x in x0..x1 {
                for j in 0..hw {
                    level.set(x, y + j, FLOOR);
                }
            }
            create(level, depth - 1, minr, !xaxis, x0, x1, y0, y - 1, rng);
            create(level, depth - 1, minr, !xaxis, x0, x1, y + hw + 1, y1, rng);
        }
    }
}
