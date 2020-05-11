use std::ops::Range;
use rand::{Rng, thread_rng, random};

use crate::vector::Vec3;
pub trait Pdf: Sync + Send {
    fn value(&self, _direction: Vec3) -> f32 {
        0.0
    }
    fn sample(&self) -> Vec3 {
        Vec3::zeros()
    }
}

pub struct Pdf1D {
    pub x: Range<f32>,
    pub pdf: Vec<f32>,
    pub cum_pdf: Vec<f32>,
    
}

impl Pdf1D {
    pub fn new(pdf: Vec<f32>) -> Self {
        let x = 380.0..780.0;
        let w = x.end - x.start;
        let pdf_sum: f32 = pdf.iter().sum();
        let pdf_normalized = pdf.iter().map(|v| {
            v / pdf_sum / w
        }).collect();
        let cum_pdf = pdf.iter().fold(vec![], |mut acc, v| {
            if acc.len() > 0 {
                acc.push(acc.last().unwrap() + v / pdf_sum)
            } else {
                acc.push(*v / pdf_sum)
            }
            acc
        });
        println!("CUMPDF: {}", cum_pdf.last().unwrap());
        Self { x, pdf: pdf_normalized, cum_pdf }
    }

    pub fn sample(&self) -> f32 {        
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

    pub fn value(&self, wavelength: f32) -> f32 {
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