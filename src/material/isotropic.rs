use std::f32::consts::PI;

use crate::material::{Material, HitRecord, ScatterRecord};
use crate::pdf::UniformPdf;
use crate::ray::Ray;
#[derive(Clone)]
pub struct Isotropic {
    // pub albedo: Box<dyn Texture>,
    pub albedo: f32,
}

impl Material for Isotropic {
    fn scatter(&self, _ray: &Ray, hit: &HitRecord) -> Option<ScatterRecord> {
        let pdf = UniformPdf {};
        Some(ScatterRecord::Diffuse {
            // attenuation: self.albedo.value(hit.uv, hit.p),
            attenuation: self.albedo,
            pdf: Box::new(pdf),
        })
    }
    fn scattering_pdf(&self, _ray_scatterd: &Ray, _hit: &HitRecord) -> f32 {
        1.0 / (4.0 * PI)
    }
    // fn is_solid(&self) -> bool {
    //     false
    // }
}
