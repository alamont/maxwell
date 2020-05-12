pub mod lambertian;
pub mod blackbody;

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
pub trait Material: Sync + Send {
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
    fn box_clone(&self) -> Box<dyn Material>;
}

impl Clone for Box<dyn Material>
{
    fn clone(&self) -> Box<dyn Material> {
        self.box_clone()
    }
}
