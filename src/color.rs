use crate::vector::Vec3;

/// Returns the CIE 1931 tristimulus values for the given wavelength.
pub fn get_tristimulus(wavelength: f32) -> Vec3 {
    let indexf = (wavelength - 380.0) / 5.0;
    let index = indexf.floor() as isize;
    let remainder = indexf - index as f32;

    if index < -1 || index > 80 {
        // Wavelength is not in the visible spectrum.
        Vec3::zeros()
    } else if index == -1 {
        // No interpolation possible.
        Vec3::new(X[0] * remainder, Y[0] * remainder, Z[0] * remainder)
    } else if index == 80 {
        // No interpolation possible.
        Vec3::new(
            X[80] * (1.0 - remainder),
            Y[80] * (1.0 - remainder),
            Z[80] * (1.0 - remainder)
        )
    } else {
        let i = index as usize;

        // Interpolate between two measurements.
        Vec3::new(
            X[i] * (1.0 - remainder) + X[i + 1] * remainder,
            Y[i] * (1.0 - remainder) + Y[i + 1] * remainder,
            Z[i] * (1.0 - remainder) + Z[i + 1] * remainder
        )
    }
}

fn gamma_correct(f: f32) -> f32 {
    if f <= 0.0031308 {
        12.92 * f
    } else {
        1.055 * f.powf(1.0 / 2.4) - 0.055
    }
}

/// Converts a CIE XYZ tristimulus to an sRGB colour.
pub fn cie_to_rgb(cie: &Vec3) -> Vec3 {
    // Apply the sRGB matrix.
    let r =  3.2406 * cie.x - 1.5372 * cie.y - 0.4986 * cie.z;
    let g = -0.9689 * cie.x + 1.8758 * cie.y + 0.0415 * cie.z;
    let b =  0.0557 * cie.x - 0.2040 * cie.y + 1.0570 * cie.z;

    // Then do gamma correction.
    Vec3::new(
        gamma_correct(clamp(r)),
        gamma_correct(clamp(g)),
        gamma_correct(clamp(b))
    )
}

fn clamp(x: f32) -> f32 {
    if x.lt(&0.0) { 0.0 }
    else if 1.0f32.lt(&x) { 1.0 }
    else { x }
}

pub fn find_exposure(tristimulus_buffer: &Vec<Vec3>) -> f32 {
    let n = tristimulus_buffer.len() as f32;

    // Compute the average intensity.
    // Calculations are based on the CIE Y value,
    // which corresponds to lightness.
    let mean = tristimulus_buffer.iter().map(|cie| cie.y).sum::<f32>() / n;

    // Then compute the standard deviation.
    let sqr_mean = tristimulus_buffer.iter().map(|cie| cie.y * cie.y).sum::<f32>() / n;
    let variance = sqr_mean - mean * mean;

    // The desired 'white' is one standard deviation above average.
    mean + variance.sqrt()
}


// Data obtained from http://cvrl.ioo.ucl.ac.uk/index.htm.

/// CIE X tristimulus values, at 5nm intervals, starting at 380 nm.
pub const X: [f32; 81] = [
    0.001368,
    0.002236,
    0.004243,
    0.007650,
    0.014310,
    0.023190,
    0.043510,
    0.077630,
    0.134380,
    0.214770,
    0.283900,
    0.328500,
    0.348280,
    0.348060,
    0.336200,
    0.318700,
    0.290800,
    0.251100,
    0.195360,
    0.142100,
    0.095640,
    0.057950,
    0.032010,
    0.014700,
    0.004900,
    0.002400,
    0.009300,
    0.029100,
    0.063270,
    0.109600,
    0.165500,
    0.225750,
    0.290400,
    0.359700,
    0.433450,
    0.512050,
    0.594500,
    0.678400,
    0.762100,
    0.842500,
    0.916300,
    0.978600,
    1.026300,
    1.056700,
    1.062200,
    1.045600,
    1.002600,
    0.938400,
    0.854450,
    0.751400,
    0.642400,
    0.541900,
    0.447900,
    0.360800,
    0.283500,
    0.218700,
    0.164900,
    0.121200,
    0.087400,
    0.063600,
    0.046770,
    0.032900,
    0.022700,
    0.015840,
    0.011359,
    0.008111,
    0.005790,
    0.004109,
    0.002899,
    0.002049,
    0.001440,
    0.001000,
    0.000690,
    0.000476,
    0.000332,
    0.000235,
    0.000166,
    0.000117,
    0.000083,
    0.000059,
    0.000042
];

/// CIE Y tristimulus values, at 5nm intervals, starting at 380 nm.
pub const Y: [f32; 81] = [
    0.000039,
    0.000064,
    0.000120,
    0.000217,
    0.000396,
    0.000640,
    0.001210,
    0.002180,
    0.004000,
    0.007300,
    0.011600,
    0.016840,
    0.023000,
    0.029800,
    0.038000,
    0.048000,
    0.060000,
    0.073900,
    0.090980,
    0.112600,
    0.139020,
    0.169300,
    0.208020,
    0.258600,
    0.323000,
    0.407300,
    0.503000,
    0.608200,
    0.710000,
    0.793200,
    0.862000,
    0.914850,
    0.954000,
    0.980300,
    0.994950,
    1.000000,
    0.995000,
    0.978600,
    0.952000,
    0.915400,
    0.870000,
    0.816300,
    0.757000,
    0.694900,
    0.631000,
    0.566800,
    0.503000,
    0.441200,
    0.381000,
    0.321000,
    0.265000,
    0.217000,
    0.175000,
    0.138200,
    0.107000,
    0.081600,
    0.061000,
    0.044580,
    0.032000,
    0.023200,
    0.017000,
    0.011920,
    0.008210,
    0.005723,
    0.004102,
    0.002929,
    0.002091,
    0.001484,
    0.001047,
    0.000740,
    0.000520,
    0.000361,
    0.000249,
    0.000172,
    0.000120,
    0.000085,
    0.000060,
    0.000042,
    0.000030,
    0.000021,
    0.000015
];

/// CIE Z tristimulus values, at 5nm intervals, starting at 380 nm.
pub const Z: [f32; 81] = [
    0.006450,
    0.010550,
    0.020050,
    0.036210,
    0.067850,
    0.110200,
    0.207400,
    0.371300,
    0.645600,
    1.039050,
    1.385600,
    1.622960,
    1.747060,
    1.782600,
    1.772110,
    1.744100,
    1.669200,
    1.528100,
    1.287640,
    1.041900,
    0.812950,
    0.616200,
    0.465180,
    0.353300,
    0.272000,
    0.212300,
    0.158200,
    0.111700,
    0.078250,
    0.057250,
    0.042160,
    0.029840,
    0.020300,
    0.013400,
    0.008750,
    0.005750,
    0.003900,
    0.002750,
    0.002100,
    0.001800,
    0.001650,
    0.001400,
    0.001100,
    0.001000,
    0.000800,
    0.000600,
    0.000340,
    0.000240,
    0.000190,
    0.000100,
    0.000050,
    0.000030,
    0.000020,
    0.000010,
    0.000000,
    0.000000,
    0.000000,
    0.000000,
    0.000000,
    0.000000,
    0.000000,
    0.000000,
    0.000000,
    0.000000,
    0.000000,
    0.000000,
    0.000000,
    0.000000,
    0.000000,
    0.000000,
    0.000000,
    0.000000,
    0.000000,
    0.000000,
    0.000000,
    0.000000,
    0.000000,
    0.000000,
    0.000000,
    0.000000,
    0.000000
];