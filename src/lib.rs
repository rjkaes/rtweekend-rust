pub mod aabb;
pub mod bvh_node;
pub mod camera;
pub mod cube;
pub mod hittable;
pub mod hittable_list;
pub mod material;
pub mod perlin;
pub mod ray;
pub mod rect;
pub mod scenes;
pub mod sphere;
pub mod texture;
pub mod vec3;

// Re-export all the public traits, structs, methods.
pub use crate::aabb::*;
pub use crate::bvh_node::*;
pub use crate::camera::*;
pub use crate::cube::*;
pub use crate::hittable::*;
pub use crate::hittable_list::*;
pub use crate::material::*;
pub use crate::perlin::*;
pub use crate::ray::*;
pub use crate::sphere::*;
pub use crate::texture::*;
pub use crate::vec3::*;

pub fn random() -> f32 {
    rand::random::<f32>()
}

#[inline]
pub fn random_range(min: f32, max: f32) -> f32 {
    // Returns a random real in [min,max).
    min + (max - min) * random()
}
