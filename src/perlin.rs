use crate::vec3::*;
use rand::rngs::SmallRng;
use rand::{Rng, SeedableRng};

const POINT_COUNT: i32 = 256;

// NOTE: i32 is REQUIRED!  Without it, negative values are do not xor or
// multiply correctly.
#[derive(Default)]
pub struct Perlin {
    ranvec: Vec<Vec3>,
    perm_x: Vec<i32>,
    perm_y: Vec<i32>,
    perm_z: Vec<i32>,
}

impl Perlin {
    pub fn new() -> Self {
        let mut rng = SmallRng::from_entropy();

        Self {
            ranvec: (0..POINT_COUNT)
                .map(|_| Vec3::random_range(-1.0, 1.0).unit())
                .collect(),
            perm_x: Self::generate_perm(&mut rng),
            perm_y: Self::generate_perm(&mut rng),
            perm_z: Self::generate_perm(&mut rng),
        }
    }

    pub fn noise(&self, p: &Point3) -> f32 {
        let u = p.x - p.x.floor();
        let v = p.y - p.y.floor();
        let w = p.z - p.z.floor();

        let xint = p.x.floor() as i32;
        let yint = p.y.floor() as i32;
        let zint = p.z.floor() as i32;

        // Hermitian Smoothing
        let uu = u * u * (3.0 - 2.0 * u);
        let vv = v * v * (3.0 - 2.0 * v);
        let ww = w * w * (3.0 - 2.0 * w);

        // Perlin interpolation
        let mut accum: f32 = 0.0;

        for i in 0..2 {
            for j in 0..2 {
                for k in 0..2 {
                    let x_idx = ((xint + i) & 255) as usize;
                    let y_idx = ((yint + j) & 255) as usize;
                    let z_idx = ((zint + k) & 255) as usize;

                    let rv = self.ranvec
                        [(self.perm_x[x_idx] ^ self.perm_y[y_idx] ^ self.perm_z[z_idx]) as usize];

                    let weight_v = vec3(u - i as f32, v - j as f32, w - k as f32);

                    accum += (i as f32 * uu + (1.0 - i as f32) * (1.0 - uu))
                        * (j as f32 * vv + (1.0 - j as f32) * (1.0 - vv))
                        * (k as f32 * ww + (1.0 - k as f32) * (1.0 - ww))
                        * weight_v.dot(&rv);
                }
            }
        }

        accum
    }

    fn generate_perm(rng: &mut rand::rngs::SmallRng) -> Vec<i32> {
        let mut indices: Vec<i32> = Vec::with_capacity(POINT_COUNT as usize);
        indices.extend(0..POINT_COUNT);

        for i in 0..POINT_COUNT {
            let j = rng.gen_range(i..POINT_COUNT);
            indices.swap(i as usize, j as usize);
        }

        indices
    }
}
