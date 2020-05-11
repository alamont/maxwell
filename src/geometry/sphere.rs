use crate::geometry::{Geometry, HitRecord};
// use crate::material::Material;
use crate::ray::Ray;
use crate::vector::{Vec2, Vec3};
use std::f32::consts::PI;

pub struct Sphere {
    pub center: Vec3,
    pub radius: f32
    // pub material: &Material,
}

// impl Sphere {
//     pub fn new(center: Vec3, radius:Vec2al: Box<dyn Material>) -> Self {
//         Sphere {
//             center,
//             radius,
//             material,
//         }
//     }
// }

impl Geometry for Sphere {
    fn hit(&self, ray: &Ray, tmin: f32, tmax: f32) -> Option<HitRecord> {
        let oc = ray.origin - self.center;
        let a = ray.direction.magnitude_squared();
        let half_b = oc.dot(&ray.direction);
        let c = oc.magnitude_squared() - self.radius * self.radius;
        let discriminant = half_b * half_b - a * c;
        if discriminant > 0.0 {
            let root = discriminant.sqrt();
            let mut t = (-half_b - root) / a;
            if t < tmax && t > tmin {
                let p = ray.at(t);
                let outward_normal = (p - self.center) / self.radius;
                let uv = get_sphere_uv(outward_normal);
                return Some(HitRecord {
                    t,
                    p,
                    normal: outward_normal,
                    // ray,
                    // &self.material,
                    uv
                });
            }
            t = (-half_b + root) / a;
            if t < tmax && t > tmin {
                let p = ray.at(t);                
                let outward_normal = (p - self.center) / self.radius;
                let uv = get_sphere_uv(outward_normal);
                return Some(HitRecord {
                    t,
                    p,
                    normal: outward_normal,
                    // ray,
                    // &self.material,
                    uv
                });
            }
        }
        None
    }
}

// fn bounding_box(&self) -> Option<AABB> {
//     Some(AABB {
//         min: self.center - vec(self.radius, self.radius, self.radius),
//         max: self.center + vec(self.radius, self.radius, self.radius),
//     })
// }

fn get_sphere_uv(p: Vec3) -> Vec2 {
    let phi = p.z.atan2(p.x);
    let theta = p.y.asin();
    let u = 1.0 - (phi + PI) / (2.0 * PI);
    let v = (theta + PI / 2.0) / PI;
    Vec2::new(u, v)
}
