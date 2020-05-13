use smallvec;

use exr::image::simple::*;
use exr::prelude::*;
use std::convert::TryInto;

use crate::vector::Vec3;

pub fn write_exr_xyz(tristimulus_buffer: &Vec<Vec3>, width: usize, height: usize, output_path: &str) {
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
