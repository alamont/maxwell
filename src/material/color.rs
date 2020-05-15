#[derive(Clone)]
pub enum Reflectance {
    Uniform(f32),
    Normal(f32, f32, f32),
}

// pub struct Refl
// pub struct DiffuseColouredMaterial {
//     /// How much the material reflects; 0.0 is black, 1.0 is white.
//     reflectance: f32,

//     /// The wavelength that is best reflected, in nm.
//     wavelength: f32,

//     /// The standard deviation of the reflectance distribution.
//     deviation: f32
// }

// impl DiffuseColouredMaterial {
//     pub fn new(refl: f32, wavel: f32, dev: f32) -> DiffuseColouredMaterial {
//         DiffuseColouredMaterial {
//             reflectance: refl,
//             wavelength: wavel,
//             deviation: dev
//         }
//     }
// }

// impl Material for DiffuseColouredMaterial {
//     fn get_new_ray(&self, incoming_ray: &Ray, intersection: &Intersection) -> Ray {
//         // Compute the probability using Gaussian falloff.
//         let p = (self.wavelength - incoming_ray.wavelength) / self.deviation;
//         let q = (-0.5 * p * p).exp();

//         let mut ray = get_diffuse_ray(incoming_ray, intersection);
        
//         // The probablity is a combination of reflectance, and the probability
//         // based on the wavelength.
//         ray.probability = self.reflectance * q;
//         ray
//     }
// }