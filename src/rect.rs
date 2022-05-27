use crate::hittable::*;
use crate::material::*;
use crate::ray::*;
use crate::vec3::*;
use std::rc::Rc;

pub struct XY {
    material: Rc<dyn Material>,
    x0: f32,
    x1: f32,
    y0: f32,
    y1: f32,
    k: f32,
}

impl XY {
    pub fn new(x0: f32, x1: f32, y0: f32, y1: f32, k: f32, material: Rc<dyn Material>) -> Self {
        Self {
            material,
            x0,
            x1,
            y0,
            y1,
            k,
        }
    }
}

impl Hittable for XY {
    fn hit(&self, r: &Ray, t_min: f32, t_max: f32) -> Option<HitRecord> {
        let t = (self.k - r.origin.z) / r.direction.z;

        if t < t_min || t > t_max {
            return None;
        }

        let x = r.origin.x + t * r.direction.x;
        let y = r.origin.y + t * r.direction.y;

        if x < self.x0 || x > self.x1 || y < self.y0 || y > self.y1 {
            return None;
        }

        let u = (x - self.x0) / (self.x1 - self.x0);
        let v = (y - self.y0) / (self.y1 - self.y0);

        let outward_normal = vec3(0.0, 0.0, 1.0);

        let (normal, front_face) = face_normal_and_is_front(r, outward_normal);

        Some(HitRecord {
            p: r.at(t),
            t,
            normal,
            material: self.material.clone(),
            u,
            v,
            front_face,
        })
    }
}

pub struct XZ {
    material: Rc<dyn Material>,
    x0: f32,
    x1: f32,
    z0: f32,
    z1: f32,
    k: f32,
}

impl XZ {
    pub fn new(x0: f32, x1: f32, z0: f32, z1: f32, k: f32, material: Rc<dyn Material>) -> Self {
        Self {
            material,
            x0,
            x1,
            z0,
            z1,
            k,
        }
    }
}

impl Hittable for XZ {
    fn hit(&self, r: &Ray, t_min: f32, t_max: f32) -> Option<HitRecord> {
        let t = (self.k - r.origin.y) / r.direction.y;

        if t < t_min || t > t_max {
            return None;
        }

        let x = r.origin.x + t * r.direction.x;
        let z = r.origin.z + t * r.direction.z;

        if x < self.x0 || x > self.x1 || z < self.z0 || z > self.z1 {
            return None;
        }

        let u = (x - self.x0) / (self.x1 - self.x0);
        let v = (z - self.z0) / (self.z1 - self.z0);

        let outward_normal = vec3(0.0, 1.0, 0.0);

        let (normal, front_face) = face_normal_and_is_front(r, outward_normal);

        Some(HitRecord {
            p: r.at(t),
            t,
            normal,
            material: self.material.clone(),
            u,
            v,
            front_face,
        })
    }
}

pub struct YZ {
    material: Rc<dyn Material>,
    y0: f32,
    y1: f32,
    z0: f32,
    z1: f32,
    k: f32,
}

impl YZ {
    pub fn new(y0: f32, y1: f32, z0: f32, z1: f32, k: f32, material: Rc<dyn Material>) -> Self {
        Self {
            material,
            y0,
            y1,
            z0,
            z1,
            k,
        }
    }
}

impl Hittable for YZ {
    fn hit(&self, r: &Ray, t_min: f32, t_max: f32) -> Option<HitRecord> {
        let t = (self.k - r.origin.x) / r.direction.x;

        if t < t_min || t > t_max {
            return None;
        }

        let y = r.origin.y + t * r.direction.y;
        let z = r.origin.z + t * r.direction.z;

        if y < self.y0 || y > self.y1 || z < self.z0 || z > self.z1 {
            return None;
        }

        let u = (y - self.y0) / (self.y1 - self.y0);
        let v = (z - self.z0) / (self.z1 - self.z0);

        let outward_normal = vec3(1.0, 0.0, 0.0);
        let (normal, front_face) = face_normal_and_is_front(r, outward_normal);

        Some(HitRecord {
            p: r.at(t),
            t,
            normal,
            material: self.material.clone(),
            u,
            v,
            front_face,
        })
    }
}
