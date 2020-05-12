use crate::constants::{BOLTZMANNS_CONSTANT, SPEED_OF_LIGHT, PLANCKS_CONSTANT, WIENS_CONSTANT};
use crate::material::{Material};
use crate::ray::Ray;
use crate::geometry::HitRecord;

#[derive(Clone)]
pub struct BlackBody {
    temperature: f32,
    normalisation_factor: f32
}

impl BlackBody {
    pub fn new(temperature: f32, intensity: f32) -> Self {
        BlackBody {
            temperature,
            normalisation_factor: intensity / boltzmann((WIENS_CONSTANT / temperature) * 1.0e9, temperature)
        }
    }
}

impl Material for BlackBody {
    fn emitted(&self, ray: &Ray, _hit: &HitRecord) -> f32 {
        boltzmann(ray.wavelength, self.temperature) * self.normalisation_factor
    }
    fn box_clone(&self) -> Box<dyn Material> {
        Box::new((*self).clone())
    }
}

pub fn boltzmann(wavelength: f32, temperature: f32) -> f32 {
    // Use double precision here, the numbers are quite large/small,
    // which might cause precision loss.
    let h = PLANCKS_CONSTANT;
    let k = BOLTZMANNS_CONSTANT;
    let c = SPEED_OF_LIGHT;

    // Multiply by 1e-9 (nano), because the wavelength is specified in nm,
    // while m is the standard unit.
    let f = c / (wavelength * 1.0e-9);

    // Then evaluate the Boltzmann distribution.
    (2.0 * h * f * f * f) / (c * c * ((h * f / (k * temperature)).exp() - 1.0)) 
}