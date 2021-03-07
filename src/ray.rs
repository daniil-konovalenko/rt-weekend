use crate::vec3::{Point, Vec3};
use std::fmt;

#[derive(Copy, Clone, Debug)]
pub struct Ray {
    pub origin: Point,
    pub direction: Vec3,
}

impl Ray {
    pub fn new(origin: Point, direction: Vec3) -> Self {
        Self { origin, direction }
    }

    pub fn at(&self, t: f64) -> Point {
        self.origin + self.direction * t
    }
}

impl fmt::Display for Ray {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{};{}", self.origin, self.direction)
    }
}
