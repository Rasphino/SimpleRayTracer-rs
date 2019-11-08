use std::ops::{Add, Neg, Sub};

use crate::vec3::Vec3;

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct Point3 {
    pub x: f32,
    pub y: f32,
    pub z: f32,
}

impl Point3 {
    pub fn new(x: f32, y: f32, z: f32) -> Point3 {
        Point3 { x, y, z }
    }

    pub fn zeros() -> Point3 {
        Point3 { x: 0.0, y: 0.0, z: 0.0 }
    }
}

impl Add<Vec3> for Point3 {
    type Output = Point3;

    fn add(self, rhs: Vec3) -> Point3 {
        Point3 {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
            z: self.z + rhs.z,
        }
    }
}

impl Neg for Point3 {
    type Output = Point3;

    fn neg(self) -> Point3 {
        Point3 {
            x: -self.x,
            y: -self.y,
            z: -self.z,
        }
    }
}

impl Sub<Point3> for Point3 {
    type Output = Vec3;

    fn sub(self, rhs: Point3) -> Vec3 {
        Vec3 {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
            z: self.z - rhs.z,
        }
    }
}

impl Sub<Vec3> for Point3 {
    type Output = Point3;
    fn sub(self, rhs: Vec3) -> Point3 {
        Point3 {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
            z: self.z - rhs.z,
        }
    }
}

impl From<Point3> for Vec3 {
    fn from(p: Point3) -> Self {
        Vec3 { x: p.x, y: p.y, z: p.z }
    }
}

#[cfg(test)]
mod test {
    use crate::vec3::Vec3;

    use super::Point3;

    #[test]
    fn add_point_with_vec() {
        assert_eq!(
            Point3 { x: 1.0, y: 0.0, z: 3.0 } + Vec3 { x: 1.0, y: 2.0, z: 1.4 },
            Point3 { x: 2.0, y: 2.0, z: 4.4 }
        );
    }

    #[test]
    fn sub_point_with_vec() {
        assert_eq!(
            Point3 { x: 1.0, y: 0.0, z: 3.0 } - Vec3 { x: 1.0, y: 2.0, z: 1.4 },
            Point3 { x: 0.0, y: -2.0, z: 1.6 }
        );
    }

    #[test]
    fn point_difference() {
        assert_eq!(
            Point3 { x: 1.0, y: 1.0, z: 3.0 } - Point3 { x: 0.1, y: 2.0, z: 2.5 },
            Vec3 { x: 0.9, y: -1.0, z: 0.5 }
        )
    }
}