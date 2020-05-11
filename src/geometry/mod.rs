pub mod sphere;

use crate::ray::Ray;
use crate::vector::{Vec2, Vec3};

pub trait Geometry {
    fn hit(&self, ray: &Ray, tmin: f32, tmax: f32) -> Option<HitRecord>;
}

pub struct HitRecord {
    pub t: f32,
    pub p: Vec3,
    pub normal: Vec3,
    // pub front_face: bool,
    // pub material: &'a Box<dyn Material>,
    pub uv: Vec2
}
