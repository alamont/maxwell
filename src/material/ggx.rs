use std::f32::consts::PI;
use rand::random;

use crate::material::{Material, HitRecord, ScatterRecord, reflect};
use crate::ray::Ray;
use crate::pdf::{Pdf};
use crate::vector::{Vec3, onb_local};

#[derive(Clone)]
pub struct GGX {
    pub reflectance: f32,
    pub roughness: f32
}

impl Material for GGX {
    fn scatter(&self, ray: &Ray, hit: &HitRecord) -> Option<ScatterRecord> {
        if self.roughness < 0.04 {
            // Treat as perfectly specular/mirror
            let reflected = reflect(&ray.direction.normalize(), &hit.normal);
            let specular_ray = Ray::new(hit.p, reflected, ray.wavelength, ray.pdf);
            Some(ScatterRecord::Specular {
                attenuation: self.reflectance,
                ray: specular_ray
            })
        } else {
            let w = hit.normal.normalize();
            let pdf = GGXPdf { w, roughness: self.roughness };
            
            Some(ScatterRecord::Diffuse {
                attenuation: self.reflectance,
                pdf: Box::new(pdf)
            })
        }
    }

    fn scattering_pdf(&self, ray_scattered: &Ray, hit: &HitRecord) -> f32 {
        let cosine_theta = hit.normal.dot(&ray_scattered.direction.normalize());
        if cosine_theta <= 0.0 {
            0.0
        } else {
            let alpha_squared = self.roughness * self.roughness;
            (alpha_squared * cosine_theta) / (PI * ((alpha_squared - 1.0) * cosine_theta * cosine_theta + 1.0).powi(2))
        }
    }
    fn box_clone(&self) -> Box<dyn Material> {
        Box::new((*self).clone())
    }
}

pub struct GGXPdf {
    pub w: Vec3,
    pub roughness: f32
}

impl Pdf<Vec3> for GGXPdf {
    fn value(&self, direction: Vec3) -> f32 {
        let cosine_theta = direction.normalize().dot(&self.w);
        if cosine_theta <= 0.0 {
            0.0
        } else {
            let alpha_squared = self.roughness * self.roughness;
            (alpha_squared * cosine_theta) / (PI * ((alpha_squared - 1.0) * cosine_theta * cosine_theta + 1.0).powi(2))        }
    }
    fn sample(&self) -> Vec3 {
        onb_local(&self.w, &random_ggx_direction(self.roughness * self.roughness))
    }
}

pub fn random_ggx_direction(alpha_squared: f32) -> Vec3 {
    let r1 = random::<f32>();
    let r2 = random::<f32>();

    let z = ((1.0 - r2) / (r2 * (alpha_squared - 1.0) + 1.0)).sqrt();
    let sine = (1.0 - z * z).sqrt();

    let phi = 2.0 * PI * r1;
    let x = phi.cos() * sine;
    let y = phi.sin() * sine;

    Vec3::new(x, y, z)
}