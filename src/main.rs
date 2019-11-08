use rand::{Rng, thread_rng};

use crate::camera::Camera;
use crate::hitable::{Hitable, HitableList, HitRecord, Sphere};
use crate::point3::Point3;
use crate::ray::Ray;
use crate::vec3::{random_in_unit_sphere, Vec3};

mod vec3;
mod ray;
mod point3;
mod hitable;
mod camera;

fn color(ray: &Ray, world: &HitableList) -> Vec3 {
    if let Some(rec) = world.hit(ray, 0.001, std::f32::MAX) {
        let target = rec.p + rec.n + random_in_unit_sphere(&mut thread_rng());
        0.5 * color(&Ray::new(rec.p, target - rec.p), world)
    } else {
        let unit_direction = ray.direction.normalize();
        let t = 0.5 * (unit_direction.y + 1.0);
        (1.0 - t) * Vec3::new(1.0, 1.0, 1.0) + t * Vec3::new(0.5, 0.7, 1.0)
    }
}

fn main() {
    let (nx, ny, ns) = (400, 200, 100);
    print!("P3\n{} {}\n255\n", nx, ny);

    let cam = Camera::new();

    let world = HitableList::new(vec![
        Box::new(Sphere::new(Point3::new(0.0, 0.0, -1.0), 0.5)),
        Box::new(Sphere::new(Point3::new(0.0, -100.5, -1.0), 100.0))
    ]);

    for y in (0..ny).rev() {
        for x in 0..nx {
            let mut col = Vec3::zeros();
            for s in 0..ns {
                let mut rng = thread_rng();

                let (u, v) = ((x as f32 + rng.gen::<f32>()) / nx as f32,
                              (y as f32 + rng.gen::<f32>()) / ny as f32);
                let r = cam.get_ray(u, v);
                let p = r.point_at_parameter(2.0);
                col += color(&r, &world);
            }
            col = col / ns as f32;
            col = col.sqrt();
            println!("{} {} {}",
                     (255.99 * col.x) as i32,
                     (255.99 * col.y) as i32,
                     (255.99 * col.z) as i32);
        }
    }
}
