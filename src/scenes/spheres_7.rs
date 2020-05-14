use crate::vector::{Vec3};
use crate::camera::Camera;
use crate::geometry::{Geometry, sphere::Sphere, bvh::BVHNode};
use crate::material::{lambertian::Lambertian, blackbody::{BlackBody}, ggx::GGX, dielectric::Sf10Glass};

pub fn sphere_7_scene(width: usize, height: usize) -> (Box<dyn Geometry>, Vec<Box<dyn Geometry>>, Camera) {
    let lookfrom = Vec3::new(0.0, 5.0,10.0);
    let lookat = Vec3::new(0.0, 0.5, 0.0);
    let vup = Vec3::new(0.0, 1.0, 0.0);
    let dist_to_focus = (lookfrom-lookat).magnitude();
    let aperture = 0.01;
    let vfov = 20.0;
    let aspect = width as f32 / height as f32;

    let camera = Camera::new(lookfrom, lookat, vup, vfov, aspect, aperture, dist_to_focus);    

    let material = Lambertian {
        reflectance: 0.5
    };

    let ggx_material_09 = GGX {
        reflectance: 0.9,
        roughness: 0.9
    };

    let ggx_material_05 = GGX {
        reflectance: 0.9,
        roughness: 0.5
    };

    let ggx_material_02 = GGX {
        reflectance: 0.9,
        roughness: 0.2
    };

    let ggx_material_01 = GGX {
        reflectance: 0.9,
        roughness: 0.1
    };

    let ggx_material_00 = GGX {
        reflectance: 0.9,
        roughness: 0.0
    };

    let glass = Sf10Glass {};

    let lights: Vec<Box<dyn Geometry>> = vec![
        Box::new(Sphere {
            center: Vec3::new(-3.0, 1.6, -1.0),
            radius: 0.25,
            material: Box::new(BlackBody::new_ideal(2000.0, 10.0))
        }),
        Box::new(Sphere {
            center: Vec3::new(-2.0, 1.6, -1.0),
            radius: 0.25,
            material: Box::new(BlackBody::new_ideal(3000.0, 10.0))
        }),
        Box::new(Sphere {
            center: Vec3::new(-1.0, 1.6, -1.0),
            radius: 0.25,
            material: Box::new(BlackBody::new_ideal(4000.0, 10.0))
        }),
        Box::new(Sphere {
            center: Vec3::new(0.0, 1.6, -1.0),
            radius: 0.25,
            material: Box::new(BlackBody::new_ideal(5000.0, 10.0))
        }),
        Box::new(Sphere {
            center: Vec3::new(1.0, 1.6, -1.0),
            radius: 0.25,
            material: Box::new(BlackBody::new_ideal(7000.0, 10.0))
        }),
        Box::new(Sphere {
            center: Vec3::new(2.0, 1.6, -1.0),
            radius: 0.25,
            material: Box::new(BlackBody::new_ideal(10000.0, 10.0))
        }),
        Box::new(Sphere {
            center: Vec3::new(3.0, 1.6, -1.0),
            radius: 0.25,
            material: Box::new(BlackBody::new_ideal(15000.0, 10.0))
        }),
        Box::new(Sphere {
            center: Vec3::new(-2.0,0.5,0.0),
            radius: 0.50,
            material: Box::new(glass.clone())
        }),
    ];

    let objects: Vec<Box<dyn Geometry>> = vec![
        Box::new(Sphere {
            center: Vec3::new(0.0,-1000.0,0.0),
            radius: 1000.0,
            material: Box::new(material.clone())
        }),

        Box::new(Sphere {
            center: Vec3::new(-3.0,0.5,0.0),
            radius: 0.50,
            material: Box::new(material.clone())
        }),
        Box::new(Sphere {
            center: Vec3::new(-2.0,0.5,0.0),
            radius: 0.50,
            material: Box::new(glass.clone())
        }),
        Box::new(Sphere {
            center: Vec3::new(-1.0,0.5,0.0),
            radius: 0.50,
            material: Box::new(ggx_material_00.clone())
        }),
        Box::new(Sphere {
            center: Vec3::new(0.0,0.5,0.0),
            radius: 0.50,
            material: Box::new(ggx_material_01.clone())
        }),
        Box::new(Sphere {
            center: Vec3::new(1.0,0.5,0.0),
            radius: 0.50,
            material: Box::new(ggx_material_02.clone())
        }),
        Box::new(Sphere {
            center: Vec3::new(2.0,0.5,0.0),
            radius: 0.50,
            material: Box::new(ggx_material_05.clone())
        }),
        Box::new(Sphere {
            center: Vec3::new(3.0,0.5,0.0),
            radius: 0.50,
            material: Box::new(ggx_material_09.clone())
        }),

        Box::new(Sphere {
            center: Vec3::new(-3.0, 1.6, -1.0),
            radius: 0.25,
            material: Box::new(BlackBody::new_ideal(2000.0, 10.0))
        }),
        Box::new(Sphere {
            center: Vec3::new(-2.0, 1.6, -1.0),
            radius: 0.25,
            material: Box::new(BlackBody::new_ideal(3000.0, 10.0))
        }),
        Box::new(Sphere {
            center: Vec3::new(-1.0, 1.6, -1.0),
            radius: 0.25,
            material: Box::new(BlackBody::new_ideal(4000.0, 10.0))
        }),
        Box::new(Sphere {
            center: Vec3::new(0.0, 1.6, -1.0),
            radius: 0.25,
            material: Box::new(BlackBody::new_ideal(5000.0, 10.0))
        }),
        Box::new(Sphere {
            center: Vec3::new(1.0, 1.6, -1.0),
            radius: 0.25,
            material: Box::new(BlackBody::new_ideal(7000.0, 10.0))
        }),
        Box::new(Sphere {
            center: Vec3::new(2.0, 1.6, -1.0),
            radius: 0.25,
            material: Box::new(BlackBody::new_ideal(10000.0, 10.0))
        }),
        Box::new(Sphere {
            center: Vec3::new(3.0, 1.6, -1.0),
            radius: 0.25,
            material: Box::new(BlackBody::new_ideal(15000.0, 10.0))
        }),
    ];

    let world = BVHNode::build( objects, 0);
    (world, lights, camera)
}