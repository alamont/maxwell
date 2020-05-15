use std::f32::consts::PI;
use rand::{random};

use crate::geometry::{Geometry, HitRecord, aabb::{AABB}};
use crate::material::Material;
use crate::ray::Ray;
use crate::vector::{Vec2, Vec3, onb_local, random_to_sphere, clamp};

#[derive(Clone)]
pub struct ConstantMedium {
    boundary:Box<dyn Geometry>,
    phase_function: Box<dyn Material>,
    neg_inv_density: f32
}

impl ConstantMedium {
    pub fn new(boundary: Box<dyn Geometry>, density: f32, material: Box<dyn Material>) -> Self {
        Self {
            boundary: boundary,
            phase_function: material,
            neg_inv_density: -1.0 / density
        }
    }
}

impl Geometry for ConstantMedium {
    fn hit(&self, ray: &Ray, tmin: f32, tmax: f32) -> Option<HitRecord> {
        // if ray.albedo_normal_ray {
        //     return None;
        // }

        if let Some(mut hit1) = self.boundary.hit(ray, f32::MIN, f32::MAX) {
            if let Some(mut hit2) = self.boundary.hit(ray, hit1.t + 0.0001, f32::MAX) {
                if hit1.t < tmin { hit1.t = tmin; }
                if hit2.t > tmax { hit2.t = tmax; }

                if hit1.t >= hit2.t { return None; }

                if hit1.t < 0.0 { hit1.t = 0.0; }

                let ray_length = ray.direction.magnitude();
                let distance_inside_boundary = (hit2.t - hit1.t) * ray_length;
                let hit_distance = self.neg_inv_density * random::<f32>().ln();

                if hit_distance > distance_inside_boundary {
                    // Extend ray to check for more hits in concave boundaries
                    return self.hit(ray, hit2.t + 0.0001, f32::MAX);
                }

                let t = hit1.t + hit_distance / ray_length;

                Some(HitRecord {
                    t,
                    p: ray.at(t),
                    normal: Vec3::new(1.0, 0.0, 0.0),
                    material: self.phase_function.clone(),
                    uv: Vec2::new(0.0, 0.0)
                })

            } else { None }
        } else { None }
    }

    fn aabb(&self) -> AABB {
        self.boundary.aabb()
    }
}

#[derive(Clone)]
pub struct NonUniformMedium {
    boundary:Box<dyn Geometry>,
    phase_function: Box<dyn Material>,
    // density: Box<dyn Texture>,
    density: f32,
    max_density: f32,
}

// impl NonUniformMedium {
//     // pub fn new(boundary: impl Geometry, density: Box<dyn Texture>, max_density: f32, material: Box<dyn Material>) -> Self {
//     pub fn new(boundary: Box<dyn Geometry>, density: f32, max_density: f32, material: Box<dyn Material>) -> Self {
//         Self {
//             boundary: boundary,
//             phase_function: material,
//             density,
//             max_density
//         }
//     }
// }

// impl Geometry for NonUniformMedium {
//     fn hit(&self, ray: &Ray, tmin: f32, tmax: f32) -> Option<HitRecord> {
//         // if ray.albedo_normal_ray {
//         //     return None;
//         // }

//         if let Some(mut hit1) = self.boundary.hit(ray, f32::MIN, f32::MAX) {
//             if let Some(mut hit2) = self.boundary.hit(ray, hit1.t + 0.0001, f32::MAX) {
//                 if hit1.t < tmin { hit1.t = tmin; }
//                 if hit2.t > tmax { hit2.t = tmax; }

//                 if hit1.t >= hit2.t { return None; }

//                 if hit1.t < 0.0 { hit1.t = 0.0; }

//                 let ray_length = ray.direction.magnitude();
//                 let distance_inside_boundary = (hit2.t - hit1.t) * ray_length;
//                 if distance_inside_boundary.is_nan() {
//                     return None;
//                 }

//                 let s_max = self.max_density;
//                 let mut d = 0.0;
//                 let t = loop {
//                     let x = random::<f32>();
//                     d += -(1.0 - x).ln() / s_max;
//                     let y = random::<f32>();
//                     if d > distance_inside_boundary {
//                         break 0.0;
//                     }
//                     let t = hit1.t + d / ray_length;
//                     if self.density.value(hit1.uv, ray.at(t)).x / s_max > y {
//                         break t;
//                     }
//                 };

//                 if d > distance_inside_boundary {
//                     // Extend ray to check for more hits in concave boundaries
//                     return self.hit(ray, hit2.t + 0.0001, f32::MAX);
//                 }

//                 Some(HitRecord {
//                     t,
//                     p :ray.at(t),
//                     normal: Vec3::new(1.0, 0.0, 0.0),
//                     material: self.phase_function,
//                     uv: Vec2::new(0.0, 0.0)
//                 })

//             } else { None }
//         } else { None }
//     }

//     fn aabb(&self) -> AABB {
//         self.boundary.aabb()
//     }
// }
