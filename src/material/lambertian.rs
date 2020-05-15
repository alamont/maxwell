use std::f32::consts::PI;

use crate::material::{Material, HitRecord, ScatterRecord, color::Reflectance};
use crate::pdf::CosinePdf;
use crate::ray::Ray;

#[derive(Clone)]
pub struct Lambertian {
    pub reflectance: Reflectance,
}

impl Material for Lambertian {
    fn scatter(&self, ray: &Ray, hit: &HitRecord) -> Option<ScatterRecord> {
        let w = hit.normal.normalize();
        let pdf = CosinePdf { w };

        // let scatter_direction = hit.normal + random_unit_vec();
        // let scattered = Ray::new(hit.p, scatter_direction);

        let attenuation = match self.reflectance {
            Reflectance::Uniform(reflectance) => reflectance,
            Reflectance::Normal(reflectance, wavelength, deviation) => {
                let p = (wavelength - ray.wavelength) / deviation;
                let q = (-0.5 * p * p).exp();
                reflectance * q
            }
        };

        Some(ScatterRecord::Diffuse {
            // attenuation: self.albedo.value(hit.uv, hit.p),
            attenuation,
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
}

