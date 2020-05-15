use nalgebra::Rotation3;

use crate::geometry::{aabb::AABB, Geometry, HitRecord};
use crate::ray::Ray;
use crate::vector::{Vec2, Vec3, deg_to_rad};

#[derive(Clone)]
pub struct Transform {
    pub object: Box<dyn Geometry>,
    pub offset: Vec3,
    pub rotation: Rotation3<f32>,
    pub bbox: AABB,
}

impl Transform {
    pub fn new(object: Box<dyn Geometry>, offset: Vec3, rotation_deg: Vec3) -> Self {
        let rotation = Rotation3::from_euler_angles(
            deg_to_rad(rotation_deg.x),
            deg_to_rad(rotation_deg.y),
            deg_to_rad(rotation_deg.z),
        );
        let bb_min_rot = rotation * object.aabb().min + offset;
        let bb_max_rot = rotation * object.aabb().max + offset;

        let bb_min = Vec3::new(
            bb_min_rot.x.min(bb_max_rot.x),
            bb_min_rot.y.min(bb_max_rot.y),
            bb_min_rot.z.min(bb_max_rot.z),
        );

        let bb_max = Vec3::new(
            bb_min_rot.x.max(bb_max_rot.x),
            bb_min_rot.y.max(bb_max_rot.y),
            bb_min_rot.z.max(bb_max_rot.z),
        );

        Self {
            object,
            offset,
            rotation,
            bbox: AABB {
                min: bb_min,
                max: bb_max,
            },
        }
    }
}

impl Geometry for Transform {
    fn hit(&self, ray: &Ray, tmin: f32, tmax: f32) -> Option<HitRecord> {
        let inv_rot = self.rotation.inverse();
        let moved_ray = Ray::new(
            inv_rot * (ray.origin - self.offset),
            inv_rot * ray.direction,
            ray.wavelength
        );
        // moved_ray.albedo_normal_ray = ray.albedo_normal_ray;

        if let Some(mut hit_rec) = self.object.hit(&moved_ray, tmin, tmax) {
            hit_rec.p = self.rotation * hit_rec.p + self.offset;
            hit_rec.normal = self.rotation * hit_rec.normal;
            Some(hit_rec)
        } else {
            None
        }
    }

    fn aabb(&self) -> AABB {
        self.bbox
    }
}
