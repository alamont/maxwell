
use crate::ray::Ray;
use crate::vector::Vec3;

#[derive(Clone, Copy, Debug)]
pub struct AABB {
    pub min: Vec3,
    pub max: Vec3
}

impl AABB {
    pub fn hit(&self, ray: &Ray, t_min: f32, t_max: f32) -> bool {
        let inv_d = Vec3::new(1.0, 1.0, 1.0).component_div(&ray.direction);
        let t0 = (self.min - ray.origin).component_mul(&inv_d);
        let t1 = (self.max - ray.origin).component_mul(&inv_d);

        let t_small = t0.zip_map(&t1, |a, b| a.min(b));
        let t_big = t0.zip_map(&t1, |a, b| a.max(b));

        t_min.max(t_small.max()) < t_max.min(t_big.min())        
    }

    pub fn zero() -> Self {
        AABB {
            min: Vec3::zeros(),
            max: Vec3::zeros(),
        }
    }
}

pub fn surrounding_box(box0: AABB, box1: AABB) -> AABB {
    let min = Vec3::new(
        box0.min.x.min(box1.min.x),
        box0.min.y.min(box1.min.y),
        box0.min.z.min(box1.min.z),
    );
    let max = Vec3::new(
        box0.max.x.max(box1.max.x),
        box0.max.y.max(box1.max.y),
        box0.max.z.max(box1.max.z),
    );
    AABB { min, max }
}