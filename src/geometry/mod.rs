pub mod sphere;

use crate::ray::Ray;
use crate::vector::{Vec2, Vec3};
use crate::material::Material;

pub trait Geometry: Sync + Send{
    fn hit(&self, ray: &Ray, tmin: f32, tmax: f32) -> Option<HitRecord>;
}

pub struct HitRecord {
    pub t: f32,
    pub p: Vec3,
    pub normal: Vec3,
    // pub front_face: bool,
    // pub material: &'a Box<dyn Material>,
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
    // fn bounding_box(&self) -> Option<AABB> {
    //     if !self.objects.is_empty() {
    //         if let Some(mut output_box) = self.objects[0].bounding_box() {
    //             for object in &self.objects[1..] {
    //                 if let Some(bb) = object.bounding_box() {
    //                     output_box = surrounding_box(output_box, bb);
    //                 }
    //             }
    //             Some(output_box)
    //         } else {
    //             None
    //         }
    //     } else {
    //         None
    //     }
    // }
}