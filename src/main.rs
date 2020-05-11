mod geometry;
mod ray;
mod vector;
mod mc;
mod color;
mod camera;
mod pdf;
mod constants;
mod material;

use rayon::prelude::*;
use rand::random;
use minifb::{Key, ScaleMode, Window, WindowOptions, MouseMode, MouseButton};

use crate::vector::{Vec3};
use crate::camera::Camera;
use crate::geometry::{Geometry, sphere::Sphere, HittableList};
use crate::ray::Ray;
use crate::color::{get_tristimulus, cie_to_rgb};
use crate::material::{lambertian::Lambertian, ScatterRecord};
use constants::{BOLTZMANNS_CONSTANT, SPEED_OF_LIGHT, PLANCKS_CONSTANT, WIENS_CONSTANT};

fn main() {

    let width = 500;
    let height = 500;
    let samples = 1000;

    let lookfrom = Vec3::new(0.0, 1.0,2.0);
    let lookat = Vec3::new(0.0, 0.5, 0.0);
    let vup = Vec3::new(0.0, 1.0, 0.0);
    let dist_to_focus = (lookfrom-lookat).magnitude();
    let aperture = 0.01;
    let vfov = 50.0;
    let aspect = width as f32 / height as f32;

    let camera = Camera::new(lookfrom, lookat, vup, vfov, aspect, aperture, dist_to_focus);

    let mut win = window(width, height);  
    let mut win_buffer: Vec<u32>;
    let mut tristimulus_buffer: Vec<Vec3> = vec![Vec3::zeros(); (width * height) as usize];

    let material = Lambertian {
        reflectance: 0.5
    };

    let sphere = Sphere {
        center: Vec3::new(0.0,0.5,0.0),
        radius: 0.50,
        material: Box::new(material.clone())
    };
    let sphere_large = Sphere {
        center: Vec3::new(0.0,-1000.0,0.0),
        radius: 1000.0,
        material: Box::new(material.clone())
    };
   
    let world: Box<dyn Geometry> = Box::new(HittableList {
        objects: vec![Box::new(sphere), Box::new(sphere_large)]
    });


    for n in 0..samples {
        tristimulus_buffer = (0..height)
            .into_par_iter()
            .flat_map(|y| {
                (0..width)
                    .map(|x| {
                        let u = (x as f32 + random::<f32>()) / width as f32;
                        let v = (height as f32 - (y as f32 + random::<f32>())) / height as f32;
                        
                        let ray = camera.get_ray_tri(u, v);                                                
                        let tristimulus_value = ray_tristimulus(&ray, &world, 50) / ray.pdf;
                        
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

        win_buffer = tristimulus_buffer
            .iter()
            .map(|tri| (cie_to_rgb(tri) * 255.99).map(|v| v as u8))
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
}

fn ray_tristimulus(ray: &Ray, world: &Box<dyn Geometry>, depth: u32) -> Vec3 {
    
    if depth <= 0 {
        return Vec3::zeros();
    }

    if let Some(hit_rec) = world.hit(&ray, 0.001, f32::MAX) {

        if let Some(scatter_record) = hit_rec.material.scatter(&ray, &hit_rec) {
            match scatter_record {
                ScatterRecord::Diffuse {attenuation, pdf} => {
                    let scattered_ray = Ray {
                        origin: hit_rec.p, 
                        direction: pdf.sample(),
                        wavelength: ray.wavelength,
                        pdf: ray.pdf
                    };
                    let pdf_val = pdf.value(scattered_ray.direction);                
                    attenuation * hit_rec.material.scattering_pdf(&scattered_ray, &hit_rec) * &ray_tristimulus(&scattered_ray, world, depth - 1) / pdf_val
                },
                _ => Vec3::zeros()
            }
        } else {
            Vec3::zeros()               
        }
    } else {
        let temperature = 6500.0;
        get_tristimulus(ray.wavelength) * boltzmann(ray.wavelength, temperature) / boltzmann((WIENS_CONSTANT / temperature) * 1.0e9, temperature)
        // Vec3::zeros()
        // Vec3::new(0.8, 0.8, 0.8)
        // get_tristimulus(ray.wavelength) * 10.0
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

fn boltzmann(wavelength: f32, temperature: f32) -> f32 {
    // Use double precision here, the numbers are quite large/small,
    // which might cause precision loss.
    let h = PLANCKS_CONSTANT;
    let k = BOLTZMANNS_CONSTANT;
    let c = SPEED_OF_LIGHT;

    // Multiply by 1e-9 (nano), because the wavelength is specified in nm,
    // while m is the standard unit.
    let f = c / (wavelength * 1.0e-9);

    // Then evaluate the Boltzmann distribution.
    (2.0 * h * f * f * f) / (c * c * ((h * f / (k * temperature)).exp() - 1.0)) 
}