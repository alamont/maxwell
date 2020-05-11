use crate::vector::Vec3;

pub struct Ray {
    pub origin: Vec3,
    pub direction: Vec3,
    pub wavelength: f32,
    pub pdf: f32,
}

impl Ray {
    pub fn at (&self, t: f32) -> Vec3 {
        return self.origin + t * self.direction;
    }
}