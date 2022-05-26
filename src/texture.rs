use crate::perlin::*;
use crate::vec3::*;
use std::rc::Rc;

pub trait Texture {
    fn value(&self, u: f32, v: f32, p: &Point3) -> Color;
}

pub struct SolidColor {
    color_value: Color,
}

impl SolidColor {
    #[inline]
    pub fn rgb(red: f32, green: f32, blue: f32) -> Self {
        Self::new(color(red, green, blue))
    }

    #[inline]
    pub fn new(color_value: Color) -> Self {
        Self { color_value }
    }
}

impl Texture for SolidColor {
    #[inline]
    fn value(&self, _u: f32, _v: f32, _p: &Point3) -> Color {
        self.color_value
    }
}

pub struct CheckerTexture {
    odd: Rc<dyn Texture>,
    even: Rc<dyn Texture>,
}

impl CheckerTexture {
    pub fn from_color(c1: Color, c2: Color) -> Self {
        Self {
            odd: Rc::new(SolidColor::new(c1)),
            even: Rc::new(SolidColor::new(c2)),
        }
    }

    pub fn new(even: Rc<dyn Texture>, odd: Rc<dyn Texture>) -> Self {
        Self { odd, even }
    }
}

impl Texture for CheckerTexture {
    #[inline]
    fn value(&self, u: f32, v: f32, p: &Point3) -> Color {
        let sines = (10.0 * p.x).sin() * (10.0 * p.y).sin() * (10.0 * p.z).sin();
        if sines < 0.0 {
            self.odd.value(u, v, p)
        } else {
            self.even.value(u, v, p)
        }
    }
}

pub struct NoiseTexture {
    noise: Perlin,
}

impl NoiseTexture {
    pub fn new() -> Self {
        Self {
            noise: Perlin::new(),
        }
    }
}

impl Texture for NoiseTexture {
    #[inline]
    fn value(&self, _u: f32, _v: f32, p: &Point3) -> Color {
        color(1.0, 1.0, 1.0) * self.noise.noise(p)
    }
}
