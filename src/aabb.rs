use crate::ray::*;
use crate::vec3::*;
use std::mem;

#[derive(Copy, Clone, Debug)]
pub struct AABB {
    pub min: Point3,
    pub max: Point3,
}

impl AABB {
    #[inline]
    fn new(min: Point3, max: Point3) -> Self {
        Self { min, max }
    }

    pub fn hit(&self, r: &Ray, t_min: f32, t_max: f32) -> bool {
        for a in 0..3 {
            let invd = 1.0 / r.direction[a];

            let mut t0 = (self.min[a] - r.origin[a]) * invd;
            let mut t1 = (self.max[a] - r.origin[a]) * invd;

            if invd < 0.0 {
                mem::swap(&mut t0, &mut t1);
            }

            let t_min_p = f32::max(t0, t_min);
            let t_max_p = f32::min(t1, t_max);

            if t_max_p <= t_min_p {
                return false;
            }
        }

        true
    }

    pub fn surrounding_box(box0: &AABB, box1: &AABB) -> Self {
        let small = point3(
            f32::min(box0.min.x, box1.min.x),
            f32::min(box0.min.y, box1.min.y),
            f32::min(box0.min.z, box1.min.z),
        );

        let big = point3(
            f32::max(box0.max.x, box1.max.x),
            f32::max(box0.max.y, box1.max.y),
            f32::max(box0.max.z, box1.max.z),
        );

        aabb(small, big)
    }
}

#[inline]
pub fn aabb(a: Point3, b: Point3) -> AABB {
    AABB::new(a, b)
}
