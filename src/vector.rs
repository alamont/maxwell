use nalgebra::{Vector2, Vector3};
use std::f32::consts::PI;
use rand::{thread_rng, Rng, random};

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

pub fn onb_local(w: &Vec3, direction: &Vec3) -> Vec3 {
    let a = if w.x.abs() > 0.9 {
        Vec3::new(0.0, 1.0, 0.0)
    } else {
        Vec3::new(1.0, 0.0, 0.0)
    };
    let v = w.cross(&a).normalize();
    let u = w.cross(&v);
    direction.x * u + direction.y * v + direction.z * w
}

pub fn random_to_sphere(radius: f32, distance_squared: f32) -> Vec3 {
    let r1 = random::<f32>();
    let r2 = random::<f32>();
    let z = 1.0 + r2 * ((1.0 - radius * radius / distance_squared).sqrt() - 1.0);

    let phi = 2.0 * PI * r1;
    let x = phi.cos() * (1.0 - z * z).sqrt();
    let y = phi.sin() * (1.0 - z * z).sqrt();

    Vec3::new(x, y, z)
}

pub fn clamp(x: f32) -> f32 {
    if x.lt(&0.0) { 0.0 }
    else if 1.0f32.lt(&x) { 1.0 }
    else { x }
}

pub fn random_unit_vec() -> Vector3<f32> {
    let mut rng = thread_rng();
    let a = rng.gen_range(0.0, 2.0 * PI);
    let z = rng.gen_range(-1.0, 1.0) as f32;
    let r = (1.0 - z * z).sqrt();
    Vector3::new(r * a.cos(), r * a.sin(), z)
}