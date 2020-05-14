use smallvec;

use exr::image::simple::*;
use exr::prelude::*;
use std::{convert::TryInto, fs};
use image::{ImageBuffer, Rgb};

use crate::vector::Vec3;
use crate::color::{find_exposure, cie_to_rgb};

pub fn write_exr_xyz(tristimulus_buffer: &Vec<Vec3>, width: usize, height: usize, output_path: String) {
    let x = Channel::new(
        "R".try_into().unwrap(),
        true,
        Samples::F32(tristimulus_buffer.iter().map(|tri| tri.x).collect()),
    );

    let y = Channel::new(
        "G".try_into().unwrap(),
        true,
        Samples::F32(tristimulus_buffer.iter().map(|tri| tri.y).collect()),
    );

    let z = Channel::new(
        "B".try_into().unwrap(),
        true,
        Samples::F32(tristimulus_buffer.iter().map(|tri| tri.z).collect()),
    );

    let layer = Layer::new(
        "test-image".try_into().unwrap(),
        (width, height),
        smallvec![x, y, z],
    );

    let layer = layer
        .with_compression(Compression::RLE)
        .with_block_format(None, attributes::LineOrder::Increasing);

    let mut image = Image::new_from_single_layer(layer);
    let chromaticities = exr::meta::attributes::Chromaticities {
        red: exr::math::Vec2(1.0, 0.0),
        green: exr::math::Vec2(0.0, 1.0),
        blue: exr::math::Vec2(0.0, 0.0),
        white: exr::math::Vec2(1.0 / 3.0, 1.0 / 3.0),
    };
    image.attributes.chromaticities = Some(chromaticities);
    image
        .write_to_file(output_path, write_options::high())
        .unwrap();
}

pub fn write_png(tristimulus_buffer: &Vec<Vec3>, width: usize, height: usize, output_path: String) {
    let max_intensity = find_exposure(&tristimulus_buffer);
    let ln_4 = 4.0f32.ln();
    let image_vec = tristimulus_buffer
        .iter()
        .map(|tri| {
            let tri_scaled = tri / max_intensity + Vec3::new(1.0, 1.0, 1.0).map(|v| v.ln()) / ln_4;
            (cie_to_rgb(&tri_scaled) * 255.99).map(|v| v as u8)
        })
        .flat_map(|v| vec![v.x, v.y, v.z])
        .collect::<Vec<u8>>();
        // .map(|v| image::Rgb([v.x, v.y, v.z]))
        // .collect::<Vec<Rgb<u8>>>();
    let image_buffer: ImageBuffer<Rgb<u8>, std::vec::Vec<u8>> = ImageBuffer::from_raw(width as u32, height as u32, image_vec).unwrap();
    image_buffer.save(output_path).unwrap();
}

pub fn get_next_output_image_name(path: &str) -> Option<String> {
    let paths = fs::read_dir(path).unwrap();
    let mut names =
    paths.filter_map(|entry| {
    entry.ok().and_then(|e|
        e.path().file_name()
        .and_then(|n| n.to_str().map(|s| String::from(s)))
    )
    }).collect::<Vec<String>>();

    names.sort();
    if let Some(name) = names.last() {
        let s: String = name.chars().take(name.len() - 4).collect();
        Some(format!("{:03}", (s.parse::<i32>().unwrap() + 1)))
    } else { None }
}
