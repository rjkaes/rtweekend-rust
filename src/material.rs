use crate::hittable::HitRecord;
use crate::ray::Ray;
use crate::texture::*;
use crate::vec3::*;
use std::rc::Rc;

pub type Scattered = (Color, Ray);
pub trait Material {
    fn scatter(&self, r_in: &Ray, rec: &HitRecord) -> Option<Scattered>;
}

pub struct Lambertian {
    pub albedo: Rc<dyn Texture>,
}

impl Lambertian {
    pub fn new(albedo: Color) -> Self {
        Lambertian {
            albedo: Rc::new(SolidColor::new(albedo)),
        }
    }

    pub fn new_from_texture(albedo: Rc<dyn Texture>) -> Self {
        Lambertian { albedo }
    }
}

impl Material for Lambertian {
    fn scatter(&self, r_in: &Ray, rec: &HitRecord) -> Option<Scattered> {
        let scattered_direction = rec.normal + random_unit_vector();

        // Catch degnerate scatter direction
        let choosen_scattered_direction = if scattered_direction.near_zero() {
            rec.normal
        } else {
            scattered_direction
        };

        Some((
            self.albedo.value(rec.u, rec.v, &rec.p),
            Ray::new(rec.p, choosen_scattered_direction, r_in.time),
        ))
    }
}

pub struct Metal {
    pub albedo: Color,
    pub fuzz: f32,
}

impl Metal {
    pub fn new(albedo: Color, fuzz: f32) -> Metal {
        Metal {
            albedo,
            fuzz: if fuzz < 1.0 { fuzz } else { 1.0 },
        }
    }
}

impl Material for Metal {
    fn scatter(&self, r_in: &Ray, rec: &HitRecord) -> Option<Scattered> {
        let reflected = r_in.direction.unit().reflect(&rec.normal);

        let scattered = Ray::new(
            rec.p,
            reflected + self.fuzz * random_in_unit_sphere(),
            r_in.time,
        );
        let attenuation = self.albedo;

        if scattered.direction.dot(&rec.normal) > 0.0 {
            Some((attenuation, scattered))
        } else {
            None
        }
    }
}

pub struct Dielectric {
    pub ir: f32,
}

impl Dielectric {
    pub fn new(ir: f32) -> Dielectric {
        Dielectric { ir }
    }
}

impl Material for Dielectric {
    fn scatter(&self, r_in: &Ray, rec: &HitRecord) -> Option<Scattered> {
        let refraction_ratio = if rec.front_face {
            1.0 / self.ir
        } else {
            self.ir
        };

        let unit_direction = r_in.direction.unit();

        let cos_theta = f32::min(-unit_direction.dot(&rec.normal), 1.0);
        let sin_theta = (1.0 - cos_theta * cos_theta).sqrt();

        let cannot_refract = refraction_ratio * sin_theta > 1.0;

        // Use Schlick's approximation for reflectance
        let mut r0 = (1.0 - refraction_ratio) / (1.0 + refraction_ratio);
        r0 = r0 * r0;
        let reflectance = r0 + (1.0 - r0) * (1.0 - cos_theta).powi(5);

        let direction = if cannot_refract || reflectance > super::random() {
            unit_direction.reflect(&rec.normal)
        } else {
            unit_direction.refract(&rec.normal, refraction_ratio)
        };

        Some((color(1.0, 1.0, 1.0), Ray::new(rec.p, direction, r_in.time)))
    }
}

fn random_in_unit_sphere() -> Vec3 {
    loop {
        let p = Vec3::random_range(-1.0, 1.0);
        if p.length_squared() >= 1.0 {
            continue;
        }
        return p;
    }
}

fn random_unit_vector() -> Vec3 {
    random_in_unit_sphere().unit()
}
