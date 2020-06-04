use std::ops::{Add, AddAssign, Div, DivAssign, Mul, MulAssign, Sub, SubAssign};

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub struct Point(pub i32, pub i32);

impl Add for Point {
    type Output = Self;
    fn add(self, other: Point) -> Point {
        Point(self.0 + other.0, self.1 + other.1)
    }
}
impl AddAssign for Point {
    fn add_assign(&mut self, other: Point) {
        self.0 += other.0;
        self.1 += other.1;
    }
}

impl Sub for Point {
    type Output = Self;
    fn sub(self, other: Point) -> Point {
        Point(self.0 - other.0, self.1 - other.1)
    }
}
impl SubAssign for Point {
    fn sub_assign(&mut self, other: Point) {
        self.0 -= other.0;
        self.1 -= other.1;
    }
}

impl Mul<i32> for Point {
    type Output = Self;
    fn mul(self, other: i32) -> Point {
        Point(self.0 * other, self.1 * other)
    }
}
impl MulAssign<i32> for Point {
    fn mul_assign(&mut self, other: i32) {
        self.0 *= other;
        self.1 *= other;
    }
}

impl Div<i32> for Point {
    type Output = Self;
    fn div(self, other: i32) -> Point {
        Point(self.0 / other, self.1 / other)
    }
}
impl DivAssign<i32> for Point {
    fn div_assign(&mut self, other: i32) {
        self.0 /= other;
        self.1 /= other;
    }
}
