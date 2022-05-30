use crate::aabb::*;
use crate::hittable::*;
use crate::ray::*;

use rand::Rng;
use std::cmp::Ordering;
use std::rc::Rc;

pub struct BVHNode {
    left: HittableInstance,
    right: HittableInstance,
    aabb_box: AABB,
}

impl BVHNode {
    pub fn new(objects: &[HittableInstance], time0: f32, time1: f32) -> Self {
        // Randomly select which axis to partition on.
        let axis: u32 = rand::thread_rng().gen_range(0..=2);
        let comparator = match axis {
            0 => box_x_compare,
            1 => box_y_compare,
            _ => box_z_compare,
        };

        let span = objects.len();

        let (left, right): (HittableInstance, HittableInstance) = match span {
            1 => (objects[0].clone(), objects[0].clone()),
            2 => {
                if comparator(&objects[0], &objects[1]) == Ordering::Less {
                    (objects[0].clone(), objects[1].clone())
                } else {
                    (objects[1].clone(), objects[0].clone())
                }
            }
            _ => {
                let mut cloned: Vec<HittableInstance> = objects.to_vec();
                cloned.sort_by(comparator);

                let mid = cloned.len() / 2;

                (
                    Rc::new(Self::new(&cloned[0..mid], time0, time1)),
                    Rc::new(Self::new(&cloned[mid..], time0, time1)),
                )
            }
        };

        let box_left = left.bounding_box(time0, time1);
        let box_right = right.bounding_box(time0, time1);

        if box_left.is_none() || box_right.is_none() {
            panic!("No bounding box in BVHNode constructor.");
        }

        let aabb_box = AABB::surrounding_box(&box_left.unwrap(), &box_right.unwrap());

        Self {
            left,
            right,
            aabb_box,
        }
    }
}

impl Hittable for BVHNode {
    fn hit(&self, r: &Ray, t_min: f32, t_max: f32) -> Option<HitRecord> {
        if !self.aabb_box.hit(r, t_min, t_max) {
            return None;
        }

        let mut rec: Option<HitRecord> = None;
        let mut closest_t = t_max;

        for object in [&self.left, &self.right] {
            if let Some(r) = object.hit(r, t_min, closest_t) {
                closest_t = r.t;
                rec = Some(r);
            }
        }

        rec
    }

    fn bounding_box(&self, _time0: f32, _time1: f32) -> Option<AABB> {
        Some(self.aabb_box)
    }
}

#[inline]
fn box_x_compare(a: &HittableInstance, b: &HittableInstance) -> Ordering {
    boxes(a, b, 0)
}

#[inline]
fn box_y_compare(a: &HittableInstance, b: &HittableInstance) -> Ordering {
    boxes(a, b, 1)
}

#[inline]
fn box_z_compare(a: &HittableInstance, b: &HittableInstance) -> Ordering {
    boxes(a, b, 2)
}

fn boxes(a: &HittableInstance, b: &HittableInstance, axis: usize) -> Ordering {
    let box_a = a.bounding_box(0.0, 0.0);
    let box_b = b.bounding_box(0.0, 0.0);

    if box_a.is_none() || box_b.is_none() {
        panic!("No bounding box in BVHNode constructor.");
    }

    if box_a.unwrap().min[axis] < box_b.unwrap().min[axis] {
        Ordering::Less
    } else {
        Ordering::Greater
    }
}
