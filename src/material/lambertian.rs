use std::f32::consts::PI;

use crate::material::{Material, HitRecord, ScatterRecord};
use crate::pdf::CosinePdf;
use crate::ray::Ray;

#[derive(Clone)]
pub struct Lambertian {
    pub reflectance: f32,
}

impl Material for Lambertian {
    fn scatter(&self, _: &Ray, hit: &HitRecord) -> Option<ScatterRecord> {
        let w = hit.normal.normalize();
        let pdf = CosinePdf { w };

        // let scatter_direction = hit.normal + random_unit_vec();
        // let scattered = Ray::new(hit.p, scatter_direction);
        Some(ScatterRecord::Diffuse {
            // attenuation: self.albedo.value(hit.uv, hit.p),
            attenuation: self.reflectance,
            pdf: Box::new(pdf),
        })
    }
    fn scattering_pdf(&self, ray_scatterd: &Ray, hit: &HitRecord) -> f32 {
        let cosine = hit.normal.dot(&ray_scatterd.direction.normalize());
        if cosine < 0.0 {
            0.0
        } else {
            cosine / PI
        }
    }
    fn box_clone(&self) -> Box<dyn Material> {
        Box::new((*self).clone())
    }
}

