#[macro_use]
extern crate smallvec;

mod camera;
mod color;
mod constants;
mod geometry;
mod material;
mod mc;
mod output;
mod pdf;
mod ray;
mod scenes;
mod vector;

use minifb::{Key, ScaleMode, Window, WindowOptions};
use rand::random;
use rayon::prelude::*;

use crate::color::{cie_to_rgb, find_exposure, get_tristimulus};
use crate::geometry::{bvh::BVHNode, sphere::Sphere, Geometry, HittableList};
use crate::material::{ScatterRecord};
use crate::output::{get_next_output_image_name, write_exr_xyz, write_png};
use crate::pdf::{GeometryPdf, MixturePdf, Pdf};
use crate::ray::Ray;
use crate::vector::Vec3;

fn main() {
    let width = 500;
    let height = 500;
    let samples = 2000;
    let exposure_compensation = 1.0;

    let mut win = window(width, height);
    let mut win_buffer: Vec<u32>;
    let mut tristimulus_buffer: Vec<Vec3> = vec![Vec3::zeros(); (width * height) as usize];

    let (world, attractors, camera) = scenes::lights::scene(width, height);

    for n in 0..samples {
        tristimulus_buffer = (0..height)
            .into_par_iter()
            .flat_map(|y| {
                (0..width)
                    .map(|x| {
                        let u = (x as f32 + random::<f32>()) / width as f32;
                        let v = (height as f32 - (y as f32 + random::<f32>())) / height as f32;

                        let (ray, ray_pdf) = camera.get_ray_tri(u, v);
                        let tristimulus_value =
                            ray_tristimulus(&ray, &world, &attractors, 50) / ray_pdf;

                        let offset = (y * width + x) as usize;

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
        let max_intensity = find_exposure(&tristimulus_buffer) * exposure_compensation;
        let ln_4 = 4.0f32.ln();

        win_buffer = tristimulus_buffer
            .iter()
            .map(|tri| {
                let tri_scaled =
                    tri / max_intensity + Vec3::new(1.0, 1.0, 1.0).map(|v| v.ln()) / ln_4;
                (cie_to_rgb(&tri_scaled) * 255.99).map(|v| v as u8)
            })
            .map(|v| ((v.x as u32) << 16) | ((v.y as u32) << 8) | v.z as u32)
            .collect();

        let mut paused = false;
        let mut exit = false;

        loop {
            win.update_with_buffer(&win_buffer, width, height).unwrap();

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

    let image_name_base = &*get_next_output_image_name("output/png/").unwrap();
    write_exr_xyz(
        &tristimulus_buffer,
        width,
        height,
        format!("output/exr/{}.exr", image_name_base),
    );
    write_png(
        &tristimulus_buffer,
        width,
        height,
        exposure_compensation,
        format!("output/png/{}.png", image_name_base),
    );
}

fn ray_tristimulus<'a>(
    ray: &Ray,
    world: &Box<dyn Geometry>,
    attractors: &'a HittableList,
    depth: u32,
) -> Vec3 {
    if depth <= 0 {
        return Vec3::zeros();
    }

    if let Some(hit_rec) = world.hit(&ray, 0.001, f32::MAX) {
        let emitted_intensity = hit_rec.material.emitted(&ray, &hit_rec);
        let emitted = emitted_intensity * get_tristimulus(ray.wavelength);

        if let Some(scatter_record) = hit_rec.material.scatter(&ray, &hit_rec) {
            match scatter_record {
                ScatterRecord::Diffuse { attenuation, pdf } => {
                    let attractors_pdf: Box<dyn Pdf<Vec3>> =
                        Box::new(attractors.generate_mixture_pdf(hit_rec.p));
                    let mixture_pdf = MixturePdf::new_power(vec![attractors_pdf, pdf], 2.0);
                    let scattered_ray = Ray {
                        origin: hit_rec.p,
                        direction: mixture_pdf.sample(),
                        wavelength: ray.wavelength,
                    };
                    let pdf_val = mixture_pdf.value(scattered_ray.direction);
                    if pdf_val == 0.0 {
                        return Vec3::zeros();
                    }
                    let tri = emitted
                        + attenuation
                            * hit_rec.material.scattering_pdf(&scattered_ray, &hit_rec)
                            * &ray_tristimulus(&scattered_ray, world, attractors, depth - 1)
                            / pdf_val;
                    if tri.x.is_nan() {
                        Vec3::zeros()
                    } else {
                        tri
                    }
                }
                ScatterRecord::Specular {
                    attenuation,
                    ray: specular_ray,
                } => attenuation * &ray_tristimulus(&specular_ray, world, attractors, depth - 1),
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

fn window(width: usize, height: usize) -> Window {
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
