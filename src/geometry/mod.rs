pub mod sphere;
pub mod bvh;
pub mod aabb;

use crate::ray::Ray;
use crate::vector::{Vec2, Vec3};
use crate::material::Material;
use crate::geometry::aabb::{AABB, surrounding_box};

pub trait Geometry: Sync + Send{
    fn hit(&self, ray: &Ray, tmin: f32, tmax: f32) -> Option<HitRecord>;
    fn aabb(&self) -> AABB;
    fn pdf(&self, _origin: &Vec3, _direction: &Vec3) -> f32 {
        0.0
    }
    fn sample_direction(&self, _origin: &Vec3) -> Vec3 {
        Vec3::new(1.0, 0.0, 0.0)
    }
}

pub struct HitRecord {
    pub t: f32,
    pub p: Vec3,
    pub normal: Vec3,
    pub material: Box<dyn Material>,
    pub uv: Vec2
}

#[derive(Default)]
pub struct HittableList {
    pub objects: Vec<Box<dyn Geometry>>,
}

impl HittableList {
    pub fn push(&mut self, geom: Box<dyn Geometry>) {
        self.objects.push(geom);
    }
}

impl Geometry for HittableList {
    fn hit(&self, ray: &Ray, t_min: f32, t_max: f32) -> Option<HitRecord> {
        let mut hit_closest: Option<HitRecord> = None;
        let mut closest_so_far = t_max;
        for hittable_obj in self.objects.iter() {
            if let Some(hit) = hittable_obj.hit(ray, t_min, closest_so_far) {
                closest_so_far = hit.t;
                hit_closest = Some(hit);
            }
        }
        return hit_closest;
    }
    fn aabb(&self) -> AABB {
        if !&self.objects.is_empty() {
            let mut output_box = self.objects[0].aabb();
            if self.objects.len() > 1 {
                for object in &self.objects[1..] {
                    let bb = object.aabb();
                    output_box = surrounding_box(output_box, bb);                    
                }
                output_box
            } else {
                AABB::zero()
            }
        } else {
            AABB::zero()
        }
    }
}