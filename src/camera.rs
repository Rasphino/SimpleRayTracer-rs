use crate::{point3::Point3, ray::Ray, vec3::Vec3};

#[derive(Copy, Clone)]
pub struct Camera {
    lower_left_corner: Point3,
    horizontal: Vec3,
    vertical: Vec3,
    origin: Point3,
}

impl Camera {
    pub fn new() -> Self {
        Camera {
            lower_left_corner: Point3::new(-2.0, -1.0, -1.0),
            horizontal: Vec3::new(4.0, 0.0, 0.0),
            vertical: Vec3::new(0.0, 2.0, 0.0),
            origin: Point3::zeros(),
        }
    }

    pub fn get_ray(&self, u: f32, v: f32) -> Ray {
        Ray::new(self.origin, (self.lower_left_corner + u * self.horizontal + v * self.vertical)
            .into())
    }
}

