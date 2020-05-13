use std::f32::consts::PI;

use crate::geometry::{Geometry, HitRecord, aabb::{AABB}};
use crate::material::Material;
use crate::ray::Ray;
use crate::vector::{Vec2, Vec3, onb_local, random_to_sphere, clamp};

pub struct Sphere {
    pub center: Vec3,
    pub radius: f32,
    pub material: Box<dyn Material>,
}

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
                    material: self.material.box_clone(),
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
                    material: self.material.box_clone(),
                    uv
                });
            }
        }
        None
    }

    fn aabb(&self) -> AABB {
        let half_size = Vec3::new(self.radius, self.radius, self.radius);
        let min = self.center - half_size;
        let max = self.center + half_size;
        AABB { min, max }
    }

    fn pdf(&self, origin: &Vec3, direction: &Vec3) -> f32 {
        let ray = Ray::new(*origin, *direction, 0.0, 0.0);
        if let Some(hit) = &self.hit(&ray, 0.001, f32::MAX) {
            let cos_theta_max = (1.0 - self.radius * self.radius / (self.center - origin).magnitude_squared()).sqrt();
            // println!("cos_theta_max: {}", cos_theta_max);
            let solid_angle = 2.0 * PI * (1.0 - cos_theta_max);
            1.0 / solid_angle
        } else { 0.0 }
    }
    fn sample_direction(&self, origin: &Vec3) -> Vec3 {        
        let direction = self.center - origin;
        let distance_squared = direction.magnitude_squared();
        onb_local(&direction.normalize(), &random_to_sphere(self.radius, distance_squared))
    }    
}

impl Sphere {
    pub fn is_inside(&self, point: Vec3) -> bool {
        (self.center - point).magnitude() < (self.radius - 0.001)
    }
}

fn get_sphere_uv(p: Vec3) -> Vec2 {
    let phi = p.z.atan2(p.x);
    let theta = p.y.asin();
    let u = 1.0 - (phi + PI) / (2.0 * PI);
    let v = (theta + PI / 2.0) / PI;
    Vec2::new(u, v)
}
