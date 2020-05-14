use rand::{thread_rng, Rng};

use crate::geometry::{
    aabb::AABB,
    aarect::{AARect, AARectType::*},
    FlipNormals, Geometry, HitRecord, HittableList,
};
use crate::material::{Material, EmptyMaterial};
use crate::ray::Ray;
use crate::vector::{Vec2, Vec3};
pub struct AABox {
    pub box_min: Vec3,
    pub box_max: Vec3,
    pub sides: HittableList,
    pub material: Box<dyn Material>
}

impl AABox {
    // pub fn new(p0: Vec3, p1: Vec3, material: Box<dyn Material>) -> Self{
    pub fn new(scale: Vec3, material: Box<dyn Material>) -> Self {
        let p = Vec3::new(0.0, 0.0, 0.0);
        let half_scale = scale / 2.0;

        let box_min = p - half_scale;
        let box_max = p + half_scale;

        let mut sides = HittableList::default();

        sides.push(
            AARect {
                xy0: p.xy() - half_scale.xy(),
                xy1: p.xy() + half_scale.xy(),
                k: p.z + half_scale.z,
                material: EmptyMaterial{}.box_clone(),
                rect_type: XY,
            }
            .boxed(),
        );
        sides.push(
            FlipNormals::new(
                AARect {
                    xy0: p.xy() - half_scale.xy(),
                    xy1: p.xy() + half_scale.xy(),
                    k: p.z - half_scale.z,
                    material: EmptyMaterial{}.box_clone(),
                    rect_type: XY,
                }
                .boxed(),
            )
            .boxed(),
        );
        sides.push(
            AARect {
                xy0: p.xz() - half_scale.xz(),
                xy1: p.xz() + half_scale.xz(),
                k: p.y + half_scale.y,
                material: EmptyMaterial{}.box_clone(),
                rect_type: XZ,
            }
            .boxed(),
        );
        sides.push(
            FlipNormals::new(
                AARect {
                    xy0: p.xz() - half_scale.xz(),
                    xy1: p.xz() + half_scale.xz(),
                    k: p.y - half_scale.y,
                    material: EmptyMaterial{}.box_clone(),
                    rect_type: XZ,
                }
                .boxed(),
            )
            .boxed(),
        );
        sides.push(
            AARect {
                xy0: p.yz() - half_scale.yz(),
                xy1: p.yz() + half_scale.yz(),
                k: p.x + half_scale.x,
                material: EmptyMaterial{}.box_clone(),
                rect_type: YZ,
            }
            .boxed(),
        );
        sides.push(
            FlipNormals::new(
                AARect {
                    xy0: p.yz() - half_scale.yz(),
                    xy1: p.yz() + half_scale.yz(),
                    k: p.x - half_scale.x,
                    material: EmptyMaterial{}.box_clone(),
                    rect_type: YZ,
                }
                .boxed(),
            )
            .boxed(),
        );

        Self {
            box_min,
            box_max,
            sides,
            material
        }
    }
}

impl Geometry for AABox {
    fn hit(&self, ray: &Ray, t_min: f32, t_max: f32) -> Option<HitRecord> {
        if let Some(mut hit) = self.sides.hit(ray, t_min, t_max) {
            hit.material = self.material.box_clone();
            Some(hit)
        } else {
            None
        }
    }
    fn aabb(&self) -> AABB {
        AABB {
            min: self.box_min,
            max: self.box_max,
        }
    }
}
