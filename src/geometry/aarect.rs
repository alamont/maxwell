use rand::{Rng, thread_rng};

use crate::geometry::{Geometry, HitRecord, aabb::{AABB}};
use crate::material::Material;
use crate::ray::Ray;
use crate::vector::{Vec2, Vec3};

#[derive(Clone)]
pub enum AARectType {
    XY,
    XZ,
    YZ,
}


#[derive(Clone)]
pub struct AARect {
    pub xy0: Vec2,
    pub xy1: Vec2,
    pub k: f32,
    pub material: Box<dyn Material>,
    pub rect_type: AARectType
}

impl AARect {
    pub fn boxed(self) -> Box<dyn Geometry> {
        Box::from(self)
    }
}

impl Geometry for AARect {
    fn hit(&self, ray: &Ray, tmin: f32, tmax: f32) -> Option<HitRecord> {
        use AARectType::*;
        let t = match &self.rect_type {
            XY => (self.k - ray.origin.z) / ray.direction.z,
            XZ => (self.k - ray.origin.y) / ray.direction.y,
            YZ => (self.k - ray.origin.x) / ray.direction.x,
        };
        if t < tmin || t > tmax {
            return None;
        }
        let xy = match &self.rect_type {
            XY => ray.origin.xy() + t * ray.direction.xy(),
            XZ => ray.origin.xz() + t * ray.direction.xz(),
            YZ => ray.origin.yz() + t * ray.direction.yz(),
        };
        if xy.x < self.xy0.x || xy.x > self.xy1.x || xy.y < self.xy0.y || xy.y > self.xy1.y {
            return None;
        }
        let uv = (xy - self.xy0).component_div(&(self.xy1 - self.xy0));
        let p = ray.at(t);
        let normal = match &self.rect_type {
            XY => Vec3::new(0.0, 0.0, 1.0),
            XZ => Vec3::new(0.0, -1.0, 0.0),
            YZ => Vec3::new(1.0, 0.0, 0.0),
        };
        Some(HitRecord {
            t,
            p,
            normal,
            material: self.material.clone(),
            uv
        })
    }

    fn aabb(&self) -> AABB {
        use AARectType::*;
        let min = Vec3::new(self.xy0.x, self.xy0.y, self.k - 0.0001);
        let max = Vec3::new(self.xy1.x, self.xy1.y, self.k + 0.0001);
        match &self.rect_type {
            XY => AABB { min, max },
            XZ => AABB { min: min.xzy(), max: max.xzy() },
            YZ => AABB { min: min.zxy(), max: max.zxy() },
        }
    }

    fn pdf(&self, origin: &Vec3, direction: &Vec3) -> f32 {
        let ray = &Ray::new(*origin, *direction, 0.0);
        if let Some(hit) = &self.hit(ray, 0.001, f32::MAX) {
            let area = (self.xy1.x - self.xy0.x) * (self.xy1.y - self.xy0.y);
            let distance_squared = (ray.at(hit.t) - ray.origin).magnitude_squared();
            let cosine = (direction.dot(&hit.normal) / direction.magnitude()).abs();
            distance_squared / ( cosine  * area )
        } else { 0.0 }        
    }

    fn sample_direction(&self, origin: &Vec3) -> Vec3 {
        use AARectType::*;
        let mut rng = thread_rng();
        match &self.rect_type {
            XY => Vec3::new(rng.gen_range(self.xy0.x, self.xy1.x), rng.gen_range(self.xy0.y, self.xy1.y), self.k) - origin,
            XZ => Vec3::new(rng.gen_range(self.xy0.x, self.xy1.x), self.k, rng.gen_range(self.xy0.y, self.xy1.y)) - origin,
            YZ => Vec3::new(self.k, rng.gen_range(self.xy0.x, self.xy1.x), rng.gen_range(self.xy0.y, self.xy1.y)) - origin,
        }
    }
}