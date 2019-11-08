use rand::thread_rng;

use crate::{point3::Point3, ray::Ray, vec3::Vec3};
use crate::vec3::random_in_unit_disk;

#[derive(Copy, Clone)]
pub struct Camera {
    lower_left_corner: Point3,
    horizontal: Vec3,
    vertical: Vec3,
    origin: Point3,
    u: Vec3,
    v: Vec3,
    w: Vec3,
    lens_radius: f32,
}

impl Camera {
    pub fn new(lookfrom: Point3, lookat: Point3, vup: Vec3, vfov: f32, aspect: f32,
               aperture: f32, focus_dist: f32) -> Self {
        let theta = vfov * std::f32::consts::PI / 180.0;
        let half_height = (0.5 * theta).tan();
        let half_width = aspect * half_height;

        let w = (lookfrom - lookat).normalize();
        let u = vup.cross(w).normalize();
        let v = w.cross(u);

        Camera {
            lower_left_corner: lookfrom - half_width * focus_dist * u - half_height * focus_dist * v -
                focus_dist * w,
            horizontal: 2.0 * half_width * focus_dist * u,
            vertical: 2.0 * half_height * focus_dist * v,
            origin: lookfrom,
            lens_radius: aperture / 2.0,
            u,
            v,
            w,
        }
    }

    pub fn get_ray(&self, u: f32, v: f32) -> Ray {
        let rd = self.lens_radius * random_in_unit_disk(&mut thread_rng());
        let offset = rd.x * self.u + rd.y * self.v;
        Ray::new(self.origin + offset,
                 (self.lower_left_corner + u * self.horizontal + v * self.vertical -
                     self.origin - offset).into())
    }
}

