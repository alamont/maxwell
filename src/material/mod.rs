pub mod lambertian;
pub mod emissive;
pub mod ggx;
pub mod dielectric;
pub mod isotropic;
pub mod color;
pub mod spectrum;

use dyn_clone::DynClone;

use crate::geometry::HitRecord;
use crate::vector::Vec3;
use crate::ray::Ray;
use crate::pdf::Pdf;


pub enum ScatterRecord {
    Specular {
        attenuation: f32,
        ray: Ray,
    },
    Diffuse {
        attenuation: f32,
        pdf: Box<dyn Pdf<Vec3>>,
    },
}
pub trait Material: Sync + Send + DynClone {
    fn scatter(&self, _ray: &Ray, _hit: &HitRecord) -> Option<ScatterRecord> {
        // (Ray, Scatter direction, pdf)
        None
    }
    fn scattering_pdf(&self, _ray_scatterd: &Ray, _hit: &HitRecord) -> f32 {
        0.0
    }
    fn emitted(&self, _ray: &Ray, _hit: &HitRecord) -> f32 {
        0.0
    }
    // fn box_clone(&self) -> Box<dyn Material>;
}
dyn_clone::clone_trait_object!(Material);


// impl Clone for Box<dyn Material>
// {
//     fn clone(&self) -> Box<dyn Material> {
//         self.box_clone()
//     }
// }

pub fn reflect(v: &Vec3, n: &Vec3) -> Vec3 {
    v - 2.0 * v.dot(&n) * n
}

pub fn refract(uv: &Vec3, n: &Vec3, etai_over_etat: f32) -> Vec3 {
    let cos_theta = (-uv).dot(&n).min(1.0);
    let r_out_parallel = etai_over_etat * (uv + cos_theta * n);
    let r_out_perp = -(1.0 - r_out_parallel.magnitude_squared()).sqrt() * n;
    r_out_parallel + r_out_perp
}

pub fn schlick(cosine: f32, ref_idx: f32) -> f32 {
    let mut r0 = (1.0 - ref_idx) / (1.0 + ref_idx);
    r0 = r0 * r0;
    r0 + (1.0 - r0) * (1.0 - cosine).powf(5.0)
}

#[derive(Clone)]
pub struct EmptyMaterial {}
impl Material for EmptyMaterial {}