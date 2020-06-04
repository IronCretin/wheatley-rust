use std::ops::{Add, Div, Mul, Sub};

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub struct Point(pub i32, pub i32);

impl Add for Point {
    type Output = Self;
    fn add(self, other: Point) -> Point {
        Point(self.0 + other.0, self.1 + other.1)
    }
}

impl Sub for Point {
    type Output = Self;
    fn sub(self, other: Point) -> Point {
        Point(self.0 - other.0, self.1 - other.1)
    }
}

impl Mul<i32> for Point {
    type Output = Self;
    fn mul(self, other: i32) -> Point {
        Point(self.0 * other, self.1 * other)
    }
}

impl Div<i32> for Point {
    type Output = Self;
    fn div(self, other: i32) -> Point {
        Point(self.0 / other, self.1 / other)
    }
}
