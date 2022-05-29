use crate::aabb::*;
use crate::hittable::{HitRecord, Hittable};
use crate::ray::*;

pub struct HittableList {
    pub objects: Vec<Box<dyn Hittable>>,
}

impl HittableList {
    pub fn new() -> Self {
        Self {
            objects: Vec::new(),
        }
    }

    pub fn clear(&mut self) {
        self.objects.clear();
    }

    pub fn add(&mut self, object: Box<dyn Hittable>) {
        self.objects.push(object);
    }
}

impl Hittable for HittableList {
    fn hit(&self, r: &Ray, t_min: f32, t_max: f32) -> Option<HitRecord> {
        let mut hit_anything: Option<HitRecord> = None;
        let mut closest_so_far = t_max;

        for object in self.objects.iter() {
            if let Some(rec) = object.hit(r, t_min, closest_so_far) {
                closest_so_far = rec.t;
                hit_anything = Some(rec);
            }
        }

        hit_anything
    }

    // TODO: Find a way to accomplish this without the mutable state.
    fn bounding_box(&self, time0: f32, time1: f32) -> Option<AABB> {
        if self.objects.is_empty() {
            return None;
        }

        let mut output_box: Option<AABB> = None;
        let mut first_box = true;

        for object in &self.objects {
            if let Some(temp_box) = object.bounding_box(time0, time1) {
                output_box = if first_box {
                    Some(temp_box)
                } else {
                    Some(AABB::surrounding_box(&output_box.unwrap(), &temp_box))
                };

                first_box = false;
            } else {
                return None;
            }
        }

        output_box
    }
}
