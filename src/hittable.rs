use crate::aabb::*;
use crate::material::*;
use crate::ray::*;
use crate::vec3::*;
use std::rc::Rc;

pub type HittableInstance = Rc<dyn Hittable>;

pub trait Hittable {
    fn hit(&self, r: &Ray, t_min: f32, t_max: f32) -> Option<HitRecord>;
    fn bounding_box(&self, time0: f32, time1: f32) -> Option<AABB>;
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
    instance: HittableInstance,
}

impl Translate {
    pub fn new(instance: HittableInstance, offset: Vec3) -> Self {
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

    fn bounding_box(&self, time0: f32, time1: f32) -> Option<AABB> {
        self.instance
            .bounding_box(time0, time1)
            .map(|output_box| aabb(output_box.min + self.offset, output_box.max + self.offset))
    }
}

pub struct RotateY {
    sin_theta: f32,
    cos_theta: f32,
    instance: HittableInstance,
    bbox: Option<AABB>,
}

impl RotateY {
    pub fn new(instance: HittableInstance, angle: f32) -> Self {
        let radians = angle.to_radians();
        let sin_theta = radians.sin();
        let cos_theta = radians.cos();

        let bbox = if let Some(bbox) = instance.bounding_box(0.0, 1.0) {
            let mut min = point3(f32::INFINITY, f32::INFINITY, f32::INFINITY);
            let mut max = point3(-f32::INFINITY, -f32::INFINITY, -f32::INFINITY);

            for i in 0..2 {
                for j in 0..2 {
                    for k in 0..2 {
                        let fi = i as f32;
                        let fj = j as f32;
                        let fk = k as f32;

                        let x = fi * bbox.max.x + (1.0 - fi) * bbox.min.x;
                        let y = fj * bbox.max.y + (1.0 - fj) * bbox.min.y;
                        let z = fk * bbox.max.z + (1.0 - fk) * bbox.min.z;

                        let newx = cos_theta * x + sin_theta * z;
                        let newz = -sin_theta * x + cos_theta * z;

                        let tester = vec3(newx, y, newz);

                        for c in 0..3 {
                            min[c] = min[c].min(tester[c]);
                            max[c] = max[c].max(tester[c]);
                        }
                    }
                }
            }

            Some(aabb(min, max))
        } else {
            None
        };

        Self {
            instance,
            bbox,
            sin_theta,
            cos_theta,
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

    fn bounding_box(&self, _time0: f32, _time1: f32) -> Option<AABB> {
        self.bbox
    }
}
