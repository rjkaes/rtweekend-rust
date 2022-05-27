use crate::material::*;
use crate::ray::*;
use crate::vec3::*;
use std::rc::Rc;

pub trait Hittable {
    fn hit(&self, r: &Ray, t_min: f32, t_max: f32) -> Option<HitRecord>;
}

// TODO: Should return an Enum rather than a tuple with a bool!
#[inline]
pub fn face_normal_and_is_front(r: &Ray, n: Vec3) -> (Vec3, bool) {
    match r.direction.dot(&n) < 0.0 {
        true => (n, true),
        false => (-n, false),
    }
}

pub struct HitRecord {
    pub p: Point3,
    pub normal: Vec3,
    pub material: Rc<dyn Material>,
    pub t: f32,
    pub u: f32,
    pub v: f32,
    pub front_face: bool,
}

pub struct Translate {
    offset: Vec3,
    instance: Box<dyn Hittable>,
}

impl Translate {
    pub fn new(instance: Box<dyn Hittable>, offset: Vec3) -> Self {
        Self { instance, offset }
    }
}

impl Hittable for Translate {
    fn hit(&self, r: &Ray, t_min: f32, t_max: f32) -> Option<HitRecord> {
        let moved_r = Ray::new(r.origin - self.offset, r.direction, r.time);

        if let Some(rec) = self.instance.hit(&moved_r, t_min, t_max) {
            let (normal, front_face) = face_normal_and_is_front(&moved_r, rec.normal);

            Some(HitRecord {
                p: rec.p + self.offset,
                normal,
                front_face,
                ..rec
            })
        } else {
            None
        }
    }
}

pub struct RotateY {
    sin_theta: f32,
    cos_theta: f32,
    instance: Box<dyn Hittable>,
}

impl RotateY {
    pub fn new(instance: Box<dyn Hittable>, angle: f32) -> Self {
        let radians = angle.to_radians();

        Self {
            instance,
            sin_theta: radians.sin(),
            cos_theta: radians.cos(),
        }
    }
}

impl Hittable for RotateY {
    fn hit(&self, r: &Ray, t_min: f32, t_max: f32) -> Option<HitRecord> {
        let mut origin = r.origin;
        let mut direction = r.direction;

        origin.x = self.cos_theta * r.origin.x - self.sin_theta * r.origin.z;
        origin.z = self.sin_theta * r.origin.x + self.cos_theta * r.origin.z;

        direction.x = self.cos_theta * r.direction.x - self.sin_theta * r.direction.z;
        direction.z = self.sin_theta * r.direction.x + self.cos_theta * r.direction.z;

        let rotated_r = Ray::new(origin, direction, r.time);

        if let Some(rec) = self.instance.hit(&rotated_r, t_min, t_max) {
            let mut p = rec.p;
            let mut normal = rec.normal;

            p.x = self.cos_theta * rec.p.x + self.sin_theta * rec.p.z;
            p.z = -self.sin_theta * rec.p.x + self.cos_theta * rec.p.z;

            normal.x = self.cos_theta * rec.normal.x + self.sin_theta * rec.normal.z;
            normal.z = -self.sin_theta * rec.normal.x + self.cos_theta * rec.normal.z;

            let (face_normal, front_face) = face_normal_and_is_front(&rotated_r, normal);

            Some(HitRecord {
                normal: face_normal,
                p,
                front_face,
                ..rec
            })
        } else {
            None
        }
    }
}
