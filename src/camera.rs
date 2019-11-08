use crate::{point3::Point3, ray::Ray, vec3::Vec3};

#[derive(Copy, Clone)]
pub struct Camera {
    lower_left_corner: Point3,
    horizontal: Vec3,
    vertical: Vec3,
    origin: Point3,
}

impl Camera {
    pub fn new(lookfrom: Point3, lookat: Point3, vup: Vec3, vfov: f32, aspect: f32)
               -> Self {
        let theta = vfov * std::f32::consts::PI / 180.0;
        let half_height = (0.5 * theta).tan();
        let half_width = aspect * half_height;

        let w = (lookfrom - lookat).normalize();
        let u = vup.cross(w).normalize();
        let v = w.cross(u);

        Camera {
            lower_left_corner: lookfrom - half_width * u - half_height * v - w,
            horizontal: 2.0 * half_width * u,
            vertical: 2.0 * half_height * v,
            origin: lookfrom,
        }
    }

    pub fn get_ray(&self, u: f32, v: f32) -> Ray {
        Ray::new(self.origin,
                 (self.lower_left_corner + u * self.horizontal + v * self.vertical -
                     self.origin).into())
    }
}

