use rand::Rng;

pub mod camera;
pub mod hittable;
pub mod hittable_list;
pub mod material;
pub mod ray;
pub mod sphere;
pub mod vec3;

// Re-export all the public traits, structs, methods.
pub use crate::camera::*;
pub use crate::hittable::*;
pub use crate::hittable_list::*;
pub use crate::material::*;
pub use crate::ray::*;
pub use crate::sphere::*;
pub use crate::vec3::*;

pub fn random() -> f32 {
    rand::random::<f32>()
}

pub fn random_range(min: f32, max: f32) -> f32 {
    let mut rng = rand::thread_rng();
    rng.gen_range(min..max)
}
