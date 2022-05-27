use crate::perlin::*;
use crate::vec3::*;
use jpeg_decoder::Decoder;
use std::fs::File;
use std::io::BufReader;
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
    scale: f32,
}

impl NoiseTexture {
    pub fn new(scale: f32) -> Self {
        Self {
            noise: Perlin::new(),
            scale,
        }
    }
}

impl Texture for NoiseTexture {
    #[inline]
    fn value(&self, _u: f32, _v: f32, p: &Point3) -> Color {
        color(1.0, 1.0, 1.0) * 0.5 * (1.0 + (self.scale * p.z + 10.0 * self.noise.turb(p, 7)).sin())
    }
}

pub struct ImageTexture {
    data: Vec<u8>,
    width: u16,
    height: u16,
    bytes_per_pixel: usize,
    bytes_per_scanline: usize,
}

impl ImageTexture {
    pub fn new(filename: &str) -> Self {
        let file = File::open(filename).expect("failed to open file");
        let mut decoder = Decoder::new(BufReader::new(file));
        let pixels = decoder.decode().expect("failed to decode image");
        let metadata = decoder.info().unwrap();

        let bytes_per_pixel = metadata.pixel_format.pixel_bytes();
        let bytes_per_scanline = bytes_per_pixel * metadata.width as usize;

        Self {
            data: pixels,
            width: metadata.width,
            height: metadata.height,
            bytes_per_pixel,
            bytes_per_scanline,
        }
    }
}

impl Texture for ImageTexture {
    fn value(&self, u: f32, v: f32, _p: &Point3) -> Color {
        // Clamp input texture coordinates to [0,1] x [1,0]
        let cu = u.clamp(0.0, 1.0);
        let cv = 1.0 - v.clamp(0.0, 1.0); // Flip V to image coordinates

        let mut i = (cu * self.width as f32) as usize;
        let mut j = (cv * self.height as f32) as usize;

        // Clamp integer mapping, since actual coordinates should be less than 1.0
        if i >= self.width as usize {
            i = (self.width - 1) as usize;
        }
        if j >= self.height as usize {
            j = (self.height - 1) as usize;
        }

        const COLOR_SCALE: f32 = 1.0 / 255.0;
        let offset = (j * self.bytes_per_scanline + i * self.bytes_per_pixel) as usize;

        color(
            COLOR_SCALE * self.data[offset] as f32,
            COLOR_SCALE * self.data[offset + 1] as f32,
            COLOR_SCALE * self.data[offset + 2] as f32,
        )
    }
}
