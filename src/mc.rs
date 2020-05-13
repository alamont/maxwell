use rand::random;
use crate::color::{X, Y, Z};
use crate::pdf::{Pdf1D, Pdf, MixturePdf};

pub fn get_wavelength_uniform() -> f32 {
    random::<f32>() * 400.0 + 380.0
}

pub struct WavelengthSampler {
    pdf: Box<dyn Pdf<f32>>
}

impl WavelengthSampler {
    pub fn new() -> Self {
        let pdf = MixturePdf::new_uniform(vec![
            Box::new(Pdf1D::new(Y.to_vec(), 380.0..780.0;)),
            Box::new(Pdf1D::new(Z.to_vec(), 380.0..780.0;)),
            Box::new(Pdf1D::new(X.to_vec(), 380.0..780.0;)),
        ]);
        Self {
            pdf: Box::new(pdf)
        }
    }

    pub fn get_wavelengths(&self) -> (f32, f32) {
        sample_clamped(&self.pdf, 0.001)
    }
}

fn sample_clamped(ws: &Box<dyn Pdf<f32>>, min_pdf: f32) -> (f32, f32) {
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