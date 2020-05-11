use std::ops::Range;
use rand::{Rng, thread_rng, random};
use std::f32::consts::PI;

use crate::vector::Vec3;
pub trait Pdf<T>: Sync + Send {
    fn value(&self, x: T) -> f32 {
        0.0
    }
    fn sample(&self) -> T;
}

pub struct Pdf1D {
    pub x: Range<f32>,
    pub pdf: Vec<f32>,
    pub cum_pdf: Vec<f32>,
    
}

impl Pdf1D {
    pub fn new(pdf: Vec<f32>) -> Self {
        let x = 380.0..780.0;
        // let w = x.end - x.start;
        let pdf_sum: f32 = pdf.iter().sum();
        let pdf_normalized = pdf.iter().map(|v| {
            v / pdf_sum * pdf.len() as f32
        }).collect();
        let cum_pdf = pdf.iter().fold(vec![], |mut acc, v| {
            if acc.len() > 0 {
                acc.push(acc.last().unwrap() + v / pdf_sum)
            } else {
                acc.push(*v / pdf_sum)
            }
            acc
        });
        Self { x, pdf: pdf_normalized, cum_pdf }
    }
}

impl Pdf<f32> for Pdf1D {

    fn sample(&self) -> f32 {        
        let rnd_num = random::<f32>();

        let index_match = self.cum_pdf.binary_search_by(|v| {           
            v.partial_cmp(&rnd_num).unwrap()
        });
        let index = match index_match {
            Ok(i) => i,
            Err(i) => i
        };
        let indexf_normed = (index as f32 + (rnd_num - self.cum_pdf[index])) / self.cum_pdf.len() as f32;
        let wavelength = (self.x.end - self.x.start) * indexf_normed + self.x.start;
        wavelength
    }

    fn value(&self, wavelength: f32) -> f32 {
        let mut indexf = (wavelength - self.x.start) / ((self.x.end - self.x.start) / self.pdf.len() as f32);
        if indexf < 0.0 {
            indexf = 0.0;
        }
        let index = indexf.floor() as isize;
        let remainder = indexf - index as f32;

        let pdf_len = (self.pdf.len() - 1) as isize;

        if index < -1 || index > pdf_len {
            0.0
        } else if index == -1 {
            self.pdf[0] * remainder
        } else if index == pdf_len {          
            let i = pdf_len as usize;  
            self.pdf[i] * (1.0 - remainder)
        } else {
            let i = index as usize;
            self.pdf[i] * (1.0 - remainder) + self.pdf[i + 1] * remainder            
        }
    }
}

pub struct CosinePdf {
    pub w: Vec3,
}

impl CosinePdf {
    pub fn new(w: Vec3) -> Self {
        Self { w: w.normalize() }
    }
}

impl Pdf<Vec3> for CosinePdf {
    fn value(&self, direction: Vec3) -> f32 {
        let cosine = direction.normalize().dot(&self.w);
        if cosine <= 0.0 {
            0.0
        } else {
            cosine / PI
        }
    }
    fn sample(&self) -> Vec3 {
        onb_local(&self.w, &random_cosine_direction())
    }
}

pub fn random_cosine_direction() -> Vec3 {
    let mut rng = thread_rng();

    let r1 = random::<f32>();
    let r2 = random::<f32>();
    let z = (1.0 - r2).sqrt();

    let phi = 2.0 * PI * r1;
    let x = phi.cos() * r2.sqrt();
    let y = phi.sin() * r2.sqrt();

    Vec3::new(x, y, z)
}

pub fn onb_local(w: &Vec3, direction: &Vec3) -> Vec3 {
    let a = if w.x.abs() > 0.9 {
        Vec3::new(0.0, 1.0, 0.0)
    } else {
        Vec3::new(1.0, 0.0, 0.0)
    };
    let v = w.cross(&a).normalize();
    let u = w.cross(&v);
    direction.x * u + direction.y * v + direction.z * w
}

pub struct MixturePdf<T> {
    pub pdfs: Vec<Box<dyn Pdf<T>>>
}

impl<T> MixturePdf<T> {
    pub fn new_uniform(pdfs: Vec<Box<dyn Pdf<T>>>) -> Self {
        Self {
            pdfs
        }
    }
}

impl Pdf<Vec3> for MixturePdf<Vec3> {
    fn value(&self, direction: Vec3) -> f32 {
        let weight = 1.0 / self.pdfs.len() as f32;
        self.pdfs.iter().fold(0.0, |acc, p| {
            acc + p.value(direction) * weight
        })
    }
    fn sample(&self) -> Vec3 {
        let mut rng = thread_rng();
        let r = rng.gen_range(0, self.pdfs.len());
        let pdf_idx = r as usize;
        self.pdfs[pdf_idx].sample()
    }
}

impl Pdf<f32> for MixturePdf<f32> {
    fn value(&self, x: f32) -> f32 {
        let weight = 1.0 / self.pdfs.len() as f32;
        self.pdfs.iter().fold(0.0, |acc, p| {
            acc + p.value(x) * weight
        })
    }
    fn sample(&self) -> f32 {
        let mut rng = thread_rng();
        let r = rng.gen_range(0, self.pdfs.len());
        let pdf_idx = r as usize;
        self.pdfs[pdf_idx].sample()
    }
}