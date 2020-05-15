use crate::constants::{BOLTZMANNS_CONSTANT, PLANCKS_CONSTANT, SPEED_OF_LIGHT, WIENS_CONSTANT};
use crate::geometry::HitRecord;
use crate::material::{
    color::Reflectance,
    lambertian::Lambertian,
    spectrum::{BlackBody, Spectrum},
    Material, ScatterRecord,
};
use crate::ray::Ray;
use crate::vector::deg_to_rad;

#[derive(Clone)]
pub struct DiffuseEmissive {
    spectrum: Box<dyn Spectrum>,
    intensity: f32,
}

impl DiffuseEmissive {
    pub fn new(spectrum: Box<dyn Spectrum>, intensity: f32) -> Self {
        DiffuseEmissive {
            spectrum,
            intensity: intensity,
        }
    }
}

impl Material for DiffuseEmissive {
    fn emitted(&self, ray: &Ray, hit: &HitRecord) -> f32 {
        if ray.direction.dot(&hit.normal) < 0.0 {
            self.spectrum.value(ray.wavelength) * self.intensity
        } else {
            0.0
        }
    }
}

#[derive(Clone)]
pub struct FalloffEmissive {
    spectrum: Box<dyn Spectrum>,
    intensity: f32,
    cos_theta_in: f32,
    cos_theta_out: f32,
    falloff: f32,
}

impl FalloffEmissive {
    pub fn new(
        spectrum: Box<dyn Spectrum>,
        intensity: f32,
        cos_theta_in: f32,
        cos_theta_out: f32,
        falloff: f32,
    ) -> Self {
        FalloffEmissive {
            spectrum,
            intensity,
            cos_theta_in,
            cos_theta_out,
            falloff,
        }
    }
}

impl Material for FalloffEmissive {
    fn emitted(&self, ray: &Ray, hit: &HitRecord) -> f32 {
        let cosine_theta = hit.normal.dot(&-ray.direction.normalize());
        let falloff = if cosine_theta < self.cos_theta_out {
            0.0
        } else if cosine_theta < self.cos_theta_in {
            1.0
        } else {
            ((cosine_theta - self.cos_theta_out) / (self.cos_theta_in - self.cos_theta_out))
                .powf(self.falloff)
        };
        self.spectrum.value(ray.wavelength) * self.intensity * falloff
    }
}
