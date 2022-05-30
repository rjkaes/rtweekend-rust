use crate::aabb::*;
use crate::hittable::*;
use crate::material::*;
use crate::ray::*;
use crate::texture::*;
use crate::vec3::*;

use std::rc::Rc;

pub struct ConstantMedium {
    boundary: HittableInstance,
    phase_function: Rc<dyn Material>,
    neg_inv_density: f32,
}

impl ConstantMedium {
    pub fn with_texture(b: HittableInstance, d: f32, a: Rc<dyn Texture>) -> Self {
        Self {
            boundary: b,
            neg_inv_density: -1.0 / d,
            phase_function: Rc::new(Isotropic::with_texture(a)),
        }
    }

    pub fn with_color(b: HittableInstance, d: f32, c: Color) -> Self {
        Self {
            boundary: b,
            neg_inv_density: -1.0 / d,
            phase_function: Rc::new(Isotropic::with_color(c)),
        }
    }
}

impl Hittable for ConstantMedium {
    fn hit(&self, r: &Ray, t_min: f32, t_max: f32) -> Option<HitRecord> {
        let h1 = self.boundary.hit(r, -f32::INFINITY, f32::INFINITY);
        if h1.is_none() {
            return None;
        }
        let rec1 = h1.unwrap();

        let h2 = self.boundary.hit(r, rec1.t + 0.0001, f32::INFINITY);
        if h2.is_none() {
            return None;
        }
        let rec2 = h2.unwrap();

        let mut rec1_t = rec1.t;
        let mut rec2_t = rec2.t;

        if rec1_t < t_min {
            rec1_t = t_min;
        }
        if rec2_t > t_max {
            rec2_t = t_max;
        }

        if rec1_t >= rec2_t {
            return None;
        }

        if rec1_t < 0.0 {
            rec1_t = 0.0;
        }

        let ray_length = r.direction.length();
        let distance_inside_boundary = (rec2_t - rec1_t) * ray_length;
        let hit_distance = self.neg_inv_density * super::random().ln();

        if hit_distance > distance_inside_boundary {
            return None;
        }

        let t = rec1_t + hit_distance / ray_length;

        Some(HitRecord {
            t,
            p: r.at(t),
            normal: vec3(1.0, 0.0, 0.0),
            front_face: true,
            material: self.phase_function.clone(),
            u: 0.0,
            v: 0.0,
        })
    }

    fn bounding_box(&self, time0: f32, time1: f32) -> Option<AABB> {
        self.boundary.bounding_box(time0, time1)
    }
}
