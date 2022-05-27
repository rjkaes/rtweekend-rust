use crate::hittable::*;
use crate::hittable_list::*;
use crate::material::*;
use crate::ray::*;
use crate::rect;
use crate::vec3::*;

use std::rc::Rc;

pub struct Cube {
    min: Point3,
    max: Point3,
    sides: HittableList,
}

impl Cube {
    pub fn new(p0: Point3, p1: Point3, material: Rc<dyn Material>) -> Self {
        let mut sides = HittableList::new();

        sides.add(Box::new(rect::XY::new(
            p0.x,
            p1.x,
            p0.y,
            p1.y,
            p1.z,
            material.clone(),
        )));
        sides.add(Box::new(rect::XY::new(
            p0.x,
            p1.x,
            p0.y,
            p1.y,
            p0.z,
            material.clone(),
        )));

        sides.add(Box::new(rect::XZ::new(
            p0.x,
            p1.x,
            p0.z,
            p1.z,
            p1.y,
            material.clone(),
        )));
        sides.add(Box::new(rect::XZ::new(
            p0.x,
            p1.x,
            p0.z,
            p1.z,
            p0.y,
            material.clone(),
        )));

        sides.add(Box::new(rect::YZ::new(
            p0.y,
            p1.y,
            p0.z,
            p1.z,
            p1.x,
            material.clone(),
        )));
        sides.add(Box::new(rect::YZ::new(
            p0.y,
            p1.y,
            p0.z,
            p1.z,
            p0.x,
            material.clone(),
        )));

        Self {
            sides,
            min: p0,
            max: p1,
        }
    }
}

impl Hittable for Cube {
    fn hit(&self, r: &Ray, t_min: f32, t_max: f32) -> Option<HitRecord> {
        self.sides.hit(r, t_min, t_max)
    }
}
