use rand::random;
use crate::vector::Vec3;
use crate::color::{X, Y, Z};
use crate::pdf::Pdf1D;

pub fn get_wavelength_uniform() -> f32 {
    random::<f32>() * 400.0 + 380.0
}

pub struct WavelengthSampler {
    x_pdf: Pdf1D,
    y_pdf: Pdf1D,
    z_pdf: Pdf1D,
    pub x_scale: f32,
    pub y_scale: f32,
    pub z_scale: f32,
}

impl WavelengthSampler {
    pub fn new() -> Self {

        let x_sum: f32 = X.to_vec().iter().sum();
        let y_sum: f32 = Y.to_vec().iter().sum();
        let z_sum: f32 = Z.to_vec().iter().sum();
        let xyz_sum = x_sum + y_sum + z_sum;

        Self {
            x_pdf: Pdf1D::new(X.to_vec()),
            y_pdf: Pdf1D::new(Y.to_vec()),
            z_pdf: Pdf1D::new(Z.to_vec()),
            x_scale: x_sum / xyz_sum,
            y_scale: y_sum / xyz_sum,
            z_scale: z_sum / xyz_sum
        }
    }

    pub fn get_wavelengths(&self) -> ((f32, f32), (f32, f32), (f32, f32)) {
        (
            sample_clamped(&self.x_pdf, 0.00001),
            sample_clamped(&self.y_pdf, 0.00001),
            sample_clamped(&self.z_pdf, 0.00001)
        )
    }
}

fn sample_clamped(ws: &Pdf1D, min_pdf: f32) -> (f32, f32) {
    let mut wavelength;
    let mut pdf;
    loop {
        wavelength = ws.sample();
        pdf = ws.value(wavelength);
        if pdf > min_pdf {
            break;
        }
    }
    (wavelength, pdf)
}