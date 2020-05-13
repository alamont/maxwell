#[macro_use]
extern crate smallvec;

mod geometry;
mod ray;
mod vector;
mod mc;
mod color;
mod camera;
mod pdf;
mod constants;
mod material;
mod output;

use rayon::prelude::*;
use rand::random;
use minifb::{Key, ScaleMode, Window, WindowOptions};

use crate::vector::{Vec3};
use crate::camera::Camera;
use crate::geometry::{Geometry, sphere::Sphere, bvh::BVHNode};
use crate::ray::Ray;
use crate::color::{get_tristimulus, cie_to_rgb, find_exposure};
use crate::material::{lambertian::Lambertian, ScatterRecord, blackbody::{BlackBody, boltzmann}};
use crate::constants::{WIENS_CONSTANT};
use crate::output::write_exr_xyz;
use crate::pdf::{Pdf, GeometryPdf, MixturePdf};

fn main() {

    let width = 1000;
    let height = 500;
    let samples = 1000;

    let lookfrom = Vec3::new(0.0, 5.0,10.0);
    let lookat = Vec3::new(0.0, 0.5, 0.0);
    let vup = Vec3::new(0.0, 1.0, 0.0);
    let dist_to_focus = (lookfrom-lookat).magnitude();
    let aperture = 0.01;
    let vfov = 20.0;
    let aspect = width as f32 / height as f32;

    let camera = Camera::new(lookfrom, lookat, vup, vfov, aspect, aperture, dist_to_focus);

    let mut win = window(width, height);  
    let mut win_buffer: Vec<u32>;
    let mut tristimulus_buffer: Vec<Vec3> = vec![Vec3::zeros(); (width * height) as usize];

    let material = Lambertian {
        reflectance: 0.5
    };

    let lights: Vec<Box<dyn Geometry>> = vec![
        Box::new(Sphere {
            center: Vec3::new(-3.0, 1.6, 0.0),
            radius: 0.25,
            material: Box::new(BlackBody::new_ideal(2000.0, 10.0))
        }),
        Box::new(Sphere {
            center: Vec3::new(-2.0, 1.6, 0.0),
            radius: 0.25,
            material: Box::new(BlackBody::new_ideal(3000.0, 10.0))
        }),
        Box::new(Sphere {
            center: Vec3::new(-1.0, 1.6, 0.0),
            radius: 0.25,
            material: Box::new(BlackBody::new_ideal(4000.0, 10.0))
        }),
        Box::new(Sphere {
            center: Vec3::new(0.0, 1.6, 0.0),
            radius: 0.25,
            material: Box::new(BlackBody::new_ideal(5000.0, 10.0))
        }),
        Box::new(Sphere {
            center: Vec3::new(1.0, 1.6, 0.0),
            radius: 0.25,
            material: Box::new(BlackBody::new_ideal(7000.0, 10.0))
        }),
        Box::new(Sphere {
            center: Vec3::new(2.0, 1.6, 0.0),
            radius: 0.25,
            material: Box::new(BlackBody::new_ideal(10000.0, 10.0))
        }),
        Box::new(Sphere {
            center: Vec3::new(3.0, 1.6, 0.0),
            radius: 0.25,
            material: Box::new(BlackBody::new_ideal(15000.0, 10.0))
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
            material: Box::new(material.clone())
        }),
        Box::new(Sphere {
            center: Vec3::new(-1.0,0.5,0.0),
            radius: 0.50,
            material: Box::new(material.clone())
        }),
        Box::new(Sphere {
            center: Vec3::new(0.0,0.5,0.0),
            radius: 0.50,
            material: Box::new(material.clone())
        }),
        Box::new(Sphere {
            center: Vec3::new(1.0,0.5,0.0),
            radius: 0.50,
            material: Box::new(material.clone())
        }),
        Box::new(Sphere {
            center: Vec3::new(2.0,0.5,0.0),
            radius: 0.50,
            material: Box::new(material.clone())
        }),
        Box::new(Sphere {
            center: Vec3::new(3.0,0.5,0.0),
            radius: 0.50,
            material: Box::new(material.clone())
        }),

        Box::new(Sphere {
            center: Vec3::new(-3.0, 1.6, 0.0),
            radius: 0.25,
            material: Box::new(BlackBody::new_ideal(2000.0, 10.0))
        }),
        Box::new(Sphere {
            center: Vec3::new(-2.0, 1.6, 0.0),
            radius: 0.25,
            material: Box::new(BlackBody::new_ideal(3000.0, 10.0))
        }),
        Box::new(Sphere {
            center: Vec3::new(-1.0, 1.6, 0.0),
            radius: 0.25,
            material: Box::new(BlackBody::new_ideal(4000.0, 10.0))
        }),
        Box::new(Sphere {
            center: Vec3::new(0.0, 1.6, 0.0),
            radius: 0.25,
            material: Box::new(BlackBody::new_ideal(5000.0, 10.0))
        }),
        Box::new(Sphere {
            center: Vec3::new(1.0, 1.6, 0.0),
            radius: 0.25,
            material: Box::new(BlackBody::new_ideal(7000.0, 10.0))
        }),
        Box::new(Sphere {
            center: Vec3::new(2.0, 1.6, 0.0),
            radius: 0.25,
            material: Box::new(BlackBody::new_ideal(10000.0, 10.0))
        }),
        Box::new(Sphere {
            center: Vec3::new(3.0, 1.6, 0.0),
            radius: 0.25,
            material: Box::new(BlackBody::new_ideal(15000.0, 10.0))
        }),
    ];

    // println!("LIGHTS LEN: {}", lights.len());
    let world = BVHNode::build( objects, 0);
    let exposure_correction = 1.0;

    for n in 0..samples {
        tristimulus_buffer = (0..height)
            .into_par_iter()
            .flat_map(|y| {
                (0..width)
                    .map(|x| {
                        let u = (x as f32 + random::<f32>()) / width as f32;
                        let v = (height as f32 - (y as f32 + random::<f32>())) / height as f32;
                        
                        let ray = camera.get_ray_tri(u, v);                                                
                        let tristimulus_value = ray_tristimulus(&ray, &world, &lights, 50) / ray.pdf;
                        
                        let offset = ((y * width + x)) as usize;

                        if n > 0 {
                            running_mean(&tristimulus_buffer[offset], &tristimulus_value, n)
                        } else {
                            tristimulus_value
                        }                        
                    })
                    .collect::<Vec<Vec3>>()
            })
            .collect::<Vec<Vec3>>();

        println!("Samples per pixel: {}", n);

        let max_intensity = find_exposure(&tristimulus_buffer) / exposure_correction;
        let ln_4 = 4.0f32.ln();
        win_buffer = tristimulus_buffer
            .iter()
            .map(|tri| {
                let tri_scaled = tri / max_intensity + Vec3::new(1.0, 1.0, 1.0).map(|v| v.ln()) / ln_4;
                (cie_to_rgb(&tri_scaled) * 255.99).map(|v| v as u8)
            })
            .map(|v| ((v.x as u32) << 16) | ((v.y as u32) << 8) | v.z as u32)
            .collect();

            let mut paused = false;
            let mut exit = false;

        loop {
            win
                .update_with_buffer(&win_buffer, width, height)
                .unwrap();        

            if !win.is_open() || win.is_key_down(Key::Escape) || win.is_key_released(Key::Escape) {
                exit = true;
                paused = false;
            }

            if win.is_key_down(Key::S) || win.is_key_released(Key::S) {
                exit = true;
                paused = false;
            }

            if !paused {
                break;
            }
        }
        if exit == true {
            break;
        }
    }

    write_exr_xyz(&tristimulus_buffer, width, height, "output/test.exr")
    
}

fn ray_tristimulus<'a>(ray: &Ray, world: &Box<dyn Geometry>, lights: &'a Vec<Box<dyn Geometry>>, depth: u32) -> Vec3 {
    
    if depth <= 0 {
        return Vec3::zeros();
    }

    if let Some(hit_rec) = world.hit(&ray, 0.001, f32::MAX) {

        let emitted_intensity = hit_rec.material.emitted(&ray, &hit_rec);
        let emitted = emitted_intensity * get_tristimulus(ray.wavelength);  

        if let Some(scatter_record) = hit_rec.material.scatter(&ray, &hit_rec) {
            match scatter_record {
                ScatterRecord::Diffuse {attenuation, pdf} => {
                    let mut pdfs: Vec<Box<dyn Pdf<Vec3>>> = Vec::new();
                    if lights.len() > 0 {                
                        for light in lights.iter() {
                            let geom_pdf: Box<dyn Pdf<Vec3>> = Box::new(GeometryPdf { origin: hit_rec.p, geometry: light });
                            pdfs.push(geom_pdf);
                        }
                    }
                    let lights_mixture_pdfs = Box::new(MixturePdf::new_uniform(pdfs));
                    let mixture_pdf = MixturePdf::new_power(vec![lights_mixture_pdfs, pdf], 2.0);
                    let scattered_ray = Ray {
                        origin: hit_rec.p, 
                        direction: mixture_pdf.sample(),
                        wavelength: ray.wavelength,
                        pdf: ray.pdf
                    };
                    let pdf_val = mixture_pdf.value(scattered_ray.direction);
                    if pdf_val == 0.0 {
                        return Vec3::zeros();
                    }
                    // println!("{}", pdf_val);
                    let tri = emitted + attenuation * hit_rec.material.scattering_pdf(&scattered_ray, &hit_rec) * &ray_tristimulus(&scattered_ray, world, lights, depth - 1) / pdf_val;
                    if tri.x.is_nan() { Vec3::zeros() } else { tri }
                },
                _ => Vec3::zeros()
            }
        } else {
            emitted
        }
    } else {
        // let temperature = 6500.0;
        // get_tristimulus(ray.wavelength) * boltzmann(ray.wavelength, temperature) / boltzmann((WIENS_CONSTANT / temperature) * 1.0e9, temperature)
        Vec3::zeros()
    }
}


fn window(width: usize, height:usize) -> Window {
    let mut window = Window::new(
        "Maxwell",
        width,
        height,
        WindowOptions {
            resize: true,
            scale_mode: ScaleMode::AspectRatioStretch,
            ..WindowOptions::default()
        },
    )
    .unwrap_or_else(|e| {
        panic!("{}", e);
    });
    window.limit_update_rate(Some(std::time::Duration::from_micros(16600)));
    window
}

fn running_mean(last_mean: &Vec3, new_value: &Vec3, n: u32) -> Vec3 {
    last_mean + (new_value - last_mean) / (n + 1) as f32
}