use crate::camera::Camera;
use crate::geometry::{
    aabox::AABox,
    aarect::{AARect, AARectType},
    bvh::BVHNode,
    sphere::Sphere,
    transform::Transform,
    volume::ConstantMedium,
    FlipNormals, Geometry, HittableList,
};
use crate::material::{
    color::Reflectance,
    emissive::{DiffuseEmissive, FalloffEmissive},
    isotropic::Isotropic,
    lambertian::Lambertian,
    spectrum::BlackBody,
    EmptyMaterial,
};
use crate::vector::{Vec2, Vec3, deg_to_rad};

pub fn scene(width: usize, height: usize) -> (Box<dyn Geometry>, Box<HittableList>, Camera) {
    let lookfrom = Vec3::new(0.0, 5.0, 10.0);
    let lookat = Vec3::new(0.0, 0.5, 0.0);
    let vup = Vec3::new(0.0, 1.0, 0.0);
    let dist_to_focus = (lookfrom - lookat).magnitude();
    let aperture = 0.01;
    let vfov = 20.0;
    let aspect = width as f32 / height as f32;

    let camera = Camera::new(lookfrom, lookat, vup, vfov, aspect, aperture, dist_to_focus);

    // let lambertian = Box::new(Lambertian { reflectance: Reflectance::Normal(0.9, 500.0, 50.0) });
    let lambertian = Box::new(Lambertian {
        reflectance: Reflectance::Uniform(0.9),
    });
    let blackbody = Box::new(BlackBody::new(6500.0));

    let lights_vec: Vec<Box<dyn Geometry>> = vec![
        Box::new(Sphere {
            center: Vec3::new(0.0, 2.5, 0.0),
            radius: 0.25,
            material: Box::new(DiffuseEmissive::new(blackbody.clone(), 5.0)),
        }),
        // Box::new(AARect {
        //     xy0: Vec2::new(-0.2,-0.2),
        //     xy1: Vec2::new(0.2,0.2),
        //     k: 2.0,
        //     material: Box::new(DiffuseEmissive::new(blackbody, 20.0)),
        //     rect_type: AARectType::XZ
        // }),
        Box::new(AARect {
            xy0: Vec2::new(-0.2, -0.2),
            xy1: Vec2::new(0.2, 0.2),
            k: 2.0,
            material: Box::new(FalloffEmissive::new(
                blackbody.clone(),
                100.0,
                deg_to_rad(3.0).cos(),
                deg_to_rad(5.0).cos(),
                2.0,
            )),
            rect_type: AARectType::XZ,
        }),
    ];

    let medium_boundary = Box::new(AABox::new(
        Vec3::new(10.0, 10.0, 10.0),
        Box::new(EmptyMaterial {}),
    ));

    let mut objects: Vec<Box<dyn Geometry>> = vec![
        FlipNormals::new(Box::new(AARect {
            xy0: Vec2::new(-1000.0, -1000.0),
            xy1: Vec2::new(1000.0, 1000.0),
            k: 0.0,
            material: lambertian,
            rect_type: AARectType::XZ,
        }))
        .boxed(),
        Box::new(ConstantMedium::new(
            medium_boundary,
            0.1,
            Box::new(Isotropic { albedo: 1.0 }),
        )),
    ];

    for light in &lights_vec {
        objects.push(light.clone());
    }

    let lights = Box::new(HittableList {
        objects: lights_vec,
    });

    let world = BVHNode::build(objects, 0);
    (world, lights, camera)
}
