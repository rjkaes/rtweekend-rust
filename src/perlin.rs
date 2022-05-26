use crate::vec3::*;
use rand::rngs::SmallRng;
use rand::{Rng, SeedableRng};

const POINT_COUNT: i32 = 256;

// NOTE: i32 is REQUIRED!  Without it, negative values are do not xor or
// multiply correctly.
#[derive(Default)]
pub struct Perlin {
    ranfloat: Vec<f32>,
    perm_x: Vec<i32>,
    perm_y: Vec<i32>,
    perm_z: Vec<i32>,
}

impl Perlin {
    pub fn new() -> Self {
        let mut rng = SmallRng::from_entropy();

        Self {
            ranfloat: (0..POINT_COUNT).map(|_| rng.gen::<f32>()).collect(),
            perm_x: Self::generate_perm(&mut rng),
            perm_y: Self::generate_perm(&mut rng),
            perm_z: Self::generate_perm(&mut rng),
        }
    }

    pub fn noise(&self, p: &Point3) -> f32 {
        let mut u = p.x - p.x.floor();
        let mut v = p.y - p.y.floor();
        let mut w = p.z - p.z.floor();

        // Hermitian Smoothing
        u = u * u * (3.0 - 2.0 * u);
        v = v * v * (3.0 - 2.0 * v);
        w = w * w * (3.0 - 2.0 * w);

        let i = p.x.floor() as i32;
        let j = p.y.floor() as i32;
        let k = p.z.floor() as i32;

        // trilinear_interp
        let mut accum: f32 = 0.0;

        for di in 0..2 {
            for dj in 0..2 {
                for dk in 0..2 {
                    let x_idx = ((i + di) & 255) as usize;
                    let y_idx = ((j + dj) & 255) as usize;
                    let z_idx = ((k + dk) & 255) as usize;

                    let rf = self.ranfloat
                        [(self.perm_x[x_idx] ^ self.perm_y[y_idx] ^ self.perm_z[z_idx]) as usize];

                    accum += (di as f32 * u + (1.0 - di as f32) * (1.0 - u))
                        * (dj as f32 * v + (1.0 - dj as f32) * (1.0 - v))
                        * (dk as f32 * w + (1.0 - dk as f32) * (1.0 - w))
                        * rf;
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
