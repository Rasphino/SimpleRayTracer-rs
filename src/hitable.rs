use crate::{material::Material, point3::Point3, ray::Ray, vec3::Vec3};

#[derive(Copy, Clone)]
pub struct HitRecord<'a> {
    pub t: f32,
    pub p: Point3,
    pub n: Vec3,
    pub material: &'a Material,
}

#[derive(Copy, Clone)]
pub struct Sphere {
    pub center: Point3,
    pub radius: f32,
    pub material: Material,
}

pub trait Hitable {
    fn hit(&self, r: &Ray, t_min: f32, t_max: f32) -> Option<HitRecord>;
}

impl Sphere {
    pub fn new(center: Point3, radius: f32, material: Material) -> Self {
        Sphere {
            center,
            radius,
            material,
        }
    }
}

impl Hitable for Sphere {
    fn hit(&self, r: &Ray, t_min: f32, t_max: f32) -> Option<HitRecord> {
        let oc = r.origin - self.center;
        let a = r.direction.dot(r.direction);
        let b = oc.dot(r.direction);
        let c = oc.dot(oc) - self.radius * self.radius;
        let discriminant = b * b - a * c;

        if discriminant > 0.0 {
            let mut temp = (-b - (b * b - a * c).sqrt()) / a;
            if temp < t_max && temp > t_min {
                let hit_point = r.point_at_parameter(temp);
                return Some(HitRecord {
                    t: temp,
                    p: hit_point,
                    n: (1.0 / self.radius) * (hit_point - self.center),
                    material: &self.material,
                });
            }

            temp = (-b + (b * b - a * c).sqrt()) / a;
            if temp < t_max && temp > t_min {
                let hit_point = r.point_at_parameter(temp);
                return Some(HitRecord {
                    t: temp,
                    p: hit_point,
                    n: (1.0 / self.radius) * (hit_point - self.center),
                    material: &self.material,
                });
            }
        }
        None
    }
}

pub struct HitableList {
    hitables: Vec<Box<dyn Hitable>>
}

impl HitableList {
    pub fn new(hitables: Vec<Box<dyn Hitable>>) -> HitableList {
        HitableList { hitables }
    }
}

impl Hitable for HitableList {
    fn hit(&self, r: &Ray, t_min: f32, t_max: f32) -> Option<HitRecord> {
        let mut closest_so_far = t_max;
        let mut maybe_hit: Option<HitRecord> = None;
        for hitable in self.hitables.iter() {
            if let Some(hit) = hitable.hit(&r, t_min, t_max) {
                closest_so_far = if hit.t < closest_so_far {
                    maybe_hit = Some(hit);
                    hit.t
                } else {
                    closest_so_far
                };
            }
        }
        maybe_hit
    }
}