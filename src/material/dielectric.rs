use std::f32::consts::PI;
use rand::random;

use crate::material::{Material, HitRecord, ScatterRecord, reflect, refract, schlick};
use crate::ray::Ray;
use crate::pdf::{Pdf};
use crate::vector::{Vec3, onb_local};

#[derive(Clone)]
pub struct Sf10Glass;

impl Sf10Glass {
    pub fn get_index_of_refraction(wavelength: f32) -> f32 {
        // See http://refractiveindex.info/?group=GLASSES&material=SF11

        // Square and convert nanometer to micrometer
        let w2 = (wavelength * wavelength * 1.0e-6) as f64;
        (1.0
            + 1.737596950 * w2 / (w2 - 0.0131887070)
            + 0.313747346 * w2 / (w2 - 0.0623068142)
            + 1.898781010 * w2 / (w2 - 155.23629000))
        .sqrt() as f32
    }
}

impl Material for Sf10Glass {
    fn scatter(&self, ray: &Ray, hit: &HitRecord) -> Option<ScatterRecord> {
        let ior = Sf10Glass::get_index_of_refraction(ray.wavelength);
        let mut normal = hit.normal;

        let etai_over_etat = if ray.direction.dot(&hit.normal) < 0.0 {            
            1.0 / ior
        } else {
            normal = -normal;
            ior
        };

        let unit_direction = ray.direction.normalize();
        let cos_theta = (-unit_direction).dot(&normal).min(1.0);
        let sin_theta = (1.0 - cos_theta * cos_theta).sqrt();

        let scattered = if etai_over_etat * sin_theta > 1.0 {
            let reflected = reflect(&unit_direction, &normal);
            Ray::new(hit.p, reflected, ray.wavelength, ray.pdf)
        } else {
            let reflect_prob = schlick(cos_theta, etai_over_etat);
            let refracted_or_reflected = if random::<f32>() < reflect_prob  {
                reflect(&unit_direction, &normal)               
            } else {
                refract(&unit_direction, &normal, etai_over_etat)
            };
            Ray::new(hit.p, refracted_or_reflected, ray.wavelength, ray.pdf)
        };

        Some(ScatterRecord::Specular {
            attenuation: 1.0,
            ray: scattered
        })
    }
    fn box_clone(&self) -> Box<dyn Material> {
        Box::new((*self).clone())
    }
}