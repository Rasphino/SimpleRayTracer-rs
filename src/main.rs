use crate::hitable::{Hitable, HitableList, HitRecord, Sphere};
use crate::point3::Point3;
use crate::ray::Ray;
use crate::vec3::Vec3;

mod vec3;
mod ray;
mod point3;
mod hitable;

fn color(ray: &Ray, world: &HitableList) -> Vec3 {
    if let Some(rec) = world.hit(ray, 0.0, std::f32::MAX) {
        0.5 * Vec3::new(rec.n.x + 1.0, rec.n.y + 1.0, rec.n.z + 1.0)
    } else {
        let unit_direction = ray.direction.normalize();
        let t = 0.5 * (unit_direction.y + 1.0);
        (1.0 - t) * Vec3::new(1.0, 1.0, 1.0) + t * Vec3::new(0.5, 0.7, 1.0)
    }
}

fn main() {
    let (nx, ny) = (200, 100);
    print!("P3\n{} {}\n255\n", nx, ny);

    let lower_left_corner = Point3::new(-2.0, -1.0, -1.0);
    let horizontal = Vec3::new(4.0, 0.0, 0.0);
    let vertical = Vec3::new(0.0, 2.0, 0.0);
    let origin = Point3::zeros();

    let world = HitableList::new(vec![
        Box::new(Sphere::new(Point3::new(0.0, 0.0, -1.0), 0.5)),
        Box::new(Sphere::new(Point3::new(0.0, -100.5, -1.0), 100.0))
    ]);

    for y in (0..ny).rev() {
        for x in (0..nx) {
            let (u, v) = (x as f32 / nx as f32, y as f32 / ny as f32);
            let r = Ray::new(origin, (lower_left_corner + u * horizontal + v * vertical).into());
            let col = color(&r, &world);
            println!("{} {} {}",
                     (255.99 * col.x) as i32,
                     (255.99 * col.y) as i32,
                     (255.99 * col.z) as i32);
        }
    }
}
