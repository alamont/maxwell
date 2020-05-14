pub mod sphere;
pub mod bvh;
pub mod aabb;
pub mod aarect;
pub mod aabox;

use crate::ray::Ray;
use crate::vector::{Vec2, Vec3};
use crate::material::Material;
use crate::geometry::aabb::{AABB, surrounding_box};
use crate::pdf::{Pdf, MixturePdf, GeometryPdf};

pub trait Geometry: Sync + Send{
    fn hit(&self, ray: &Ray, tmin: f32, tmax: f32) -> Option<HitRecord>;
    fn aabb(&self) -> AABB;
    fn pdf(&self, _origin: &Vec3, _direction: &Vec3) -> f32 {
        0.0
    }
    fn sample_direction(&self, _origin: &Vec3) -> Vec3 {
        Vec3::new(1.0, 0.0, 0.0)
    }
    fn is_inside(&self, point: Vec3) -> bool {
        false
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
    pub fn generate_mixture_pdf(&self, hit_position: Vec3) -> MixturePdf<Vec3> {
        let pdfs = self.objects.iter().map(|object| {
            let geom_pdf: Box<dyn Pdf<Vec3>> = Box::new(GeometryPdf { origin: hit_position, geometry: object });
            geom_pdf
        }).collect::<Vec<Box<dyn Pdf<Vec3>>>>();
        MixturePdf::new_uniform(pdfs)
    }
}

impl Geometry for HittableList {
    fn hit(&self, ray: &Ray, tmin: f32, tmax: f32) -> Option<HitRecord> {
        let mut hit_closest: Option<HitRecord> = None;
        let mut closest_so_far = tmax;
        for hittable_obj in self.objects.iter() {
            if let Some(hit) = hittable_obj.hit(ray, tmin, closest_so_far) {
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
    fn is_inside(&self, point: Vec3) -> bool {
        self.aabb().is_inside(point)
    }
}

pub struct FlipNormals {
    pub object: Box<dyn Geometry>
}

impl FlipNormals {
    pub fn new(object: Box<dyn Geometry>) -> Self {
        Self {
            object
        }
    }
    pub fn boxed(self) -> Box<Self> {
        Box::from(self)
    }
}

impl Geometry for FlipNormals {
    fn hit(&self, ray: &Ray, tmin: f32, tmax: f32) -> Option<HitRecord> {
        if let Some(mut hit_rec) = self.object.hit(ray, tmin, tmax) {
            hit_rec.normal = -hit_rec.normal;
            Some(hit_rec)
        } else {
            None
        }
    }
    fn aabb(&self) -> AABB {
        self.object.aabb()
    }
}