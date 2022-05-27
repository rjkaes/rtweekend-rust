pub mod camera;
pub mod hittable;
pub mod hittable_list;
pub mod material;
pub mod perlin;
pub mod ray;
pub mod sphere;
pub mod texture;
pub mod vec3;
pub mod xy_rect;

// Re-export all the public traits, structs, methods.
pub use crate::camera::*;
pub use crate::hittable::*;
pub use crate::hittable_list::*;
pub use crate::material::*;
pub use crate::perlin::*;
pub use crate::ray::*;
pub use crate::sphere::*;
pub use crate::texture::*;
pub use crate::vec3::*;
pub use crate::xy_rect::*;

pub fn random() -> f32 {
    rand::random::<f32>()
}

#[inline]
pub fn random_range(min: f32, max: f32) -> f32 {
    // Returns a random real in [min,max).
    min + (max - min) * random()
}
