use std::{thread, time};
use std::process::exit;

use minifb::{Key, Window, WindowOptions};
use rand::{Rng, thread_rng};

use crate::camera::Camera;
use crate::hitable::{Hitable, HitableList, HitRecord, Sphere};
use crate::material::Material;
use crate::point3::Point3;
use crate::ray::Ray;
use crate::vec3::{random_in_unit_sphere, Vec3};

mod vec3;
mod ray;
mod point3;
mod hitable;
mod camera;
mod material;

fn color(ray: &Ray, world: &HitableList, depth: i32) -> Vec3 {
    if let Some(rec) = world.hit(ray, 0.001, std::f32::MAX) {
        if depth < 50 {
            let scatter = rec.material.scatter(ray, &rec);
            scatter.attenuation * color(&scatter.ray, world, depth + 1)
        } else {
            Vec3::zeros()
        }
    } else {
        let unit_direction = ray.direction.normalize();
        let t = 0.5 * (unit_direction.y + 1.0);
        (1.0 - t) * Vec3::new(1.0, 1.0, 1.0) + t * Vec3::new(0.5, 0.7, 1.0)
    }
}

fn to_bgra(r: u32, g: u32, b: u32) -> u32 {
    255 << 24 | r << 16 | g << 8 | b
}

fn main() {
    let (nx, ny, ns) = (400, 200, 100);

    let cam = Camera::new(Point3::new(-2.0, 2.0, 1.0),
                          Point3::new(0.0, 0.0, -1.0),
                          Vec3::new(0.0, 1.0, 0.0),
                          40.0,
                          nx as f32 / ny as f32);

    let world = HitableList::new(vec![
        Box::new(Sphere::new(Point3::new(0.0, 0.0, -1.0), 0.5,
                             Material::lambertian(Vec3::new(0.1, 0.2, 0.5)))),
        Box::new(Sphere::new(Point3::new(0.0, -100.5, -1.0), 100.0,
                             Material::lambertian(Vec3::new(0.8, 0.8, 0.0)))),
        Box::new(Sphere::new(Point3::new(1.0, 0.0, -1.0), 0.5,
                             Material::metal(Vec3::new(0.8, 0.6, 0.2), 0.0))),
        Box::new(Sphere::new(Point3::new(-1.0, 0.0, -1.0), 0.5,
                             Material::dielectric(1.5))),
    ]);

    let mut buffer: Vec<u32> = vec![0; nx * ny];
    let mut window = Window::new("SimpleRayTracer-rs",
                                 nx,
                                 ny,
                                 WindowOptions::default()).unwrap_or_else(|e| {
        panic!("{}", e);
    });

    let mut is_finished = false;
    while window.is_open() {
        if !is_finished {
            for y in (0..ny).rev() {
                for x in 0..nx {
                    let mut col = Vec3::zeros();
                    for s in 0..ns {
                        let mut rng = thread_rng();

                        let (u, v) = ((x as f32 + rng.gen::<f32>()) / nx as f32,
                                      (y as f32 + rng.gen::<f32>()) / ny as f32);
                        let r = cam.get_ray(u, v);
                        col += color(&r, &world, 0);
                    }
                    col = col / ns as f32;
                    col = col.sqrt();
                    let mut i = buffer.get_mut(nx * (ny - 1) - nx * y + x).unwrap();
                    *i = to_bgra((255.99 * col.x) as u32,
                                 (255.99 * col.y) as u32,
                                 (255.99 * col.z) as u32);
                }
                window.update_with_buffer(&buffer).unwrap();
            }
            is_finished = true;
        } else {
            window.get_keys().map(|keys| {
                for t in keys {
                    match t {
                        Key::Escape => {
                            println!("Goodbye!");
                            exit(0);
                        }
                        _ => (),
                    }
                }
            });
            window.update_with_buffer(&buffer).unwrap();
            thread::sleep(time::Duration::from_millis(50));
        }
    }
}
