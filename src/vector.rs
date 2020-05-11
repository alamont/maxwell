use nalgebra::{Vector2, Vector3};
use std::f32::consts::PI;
use rand::{thread_rng, Rng};

pub type Vec2 = Vector2<f32>;
pub type Vec3 = Vector3<f32>;

pub fn random_unit_in_disk() -> Vec3 {
    let mut rng = thread_rng();
    let mut p;
    loop {
        p = Vec3::new(rng.gen_range(-1.0, 1.0), rng.gen_range(-1.0, 1.0), 0.0);
        if p.magnitude_squared() < 1.0 {
            break;
        }
    }
    p
}

pub fn deg_to_rad(deg: f32) -> f32 {
    deg * PI / 180.0
}