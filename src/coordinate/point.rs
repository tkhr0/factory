use std::cmp::{Ordering, PartialEq};
use std::ops::{Add, Sub};

use super::Size;

#[derive(Debug)]
pub struct Point {
    pub x: f64,
    pub y: f64,
}

impl Point {
    pub fn new(x: f64, y: f64) -> Self {
        Self { x, y }
    }
}

impl Add<&Size> for &Point {
    type Output = Point;

    fn add(self, rhs: &Size) -> Self::Output {
        Point::new(self.x + rhs.width, self.y + rhs.height)
    }
}

impl Sub for &Point {
    type Output = Point;

    fn sub(self, rhs: Self) -> Self::Output {
        Point::new(self.x - rhs.x, self.y - rhs.y)
    }
}

impl PartialEq for Point {
    fn eq(&self, other: &Self) -> bool {
        self.x == other.x && self.y == other.y
    }
}

impl PartialOrd for Point {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        if self.x < other.x && self.y < other.y {
            return Some(Ordering::Less);
        }

        if self.x > other.x && self.y > other.y {
            return Some(Ordering::Greater);
        }

        None
    }
}
