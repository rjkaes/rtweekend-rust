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

    pub fn turb(&self, p: &Point3, depth: u32) -> f32 {
        let mut accum: f32 = 0.0;
        let mut tp = *p;
        let mut weight: f32 = 1.0;

        for _ in 0..depth {
            accum += weight * self.noise(&tp);
            weight *= 0.5;
            tp *= 2.0;
        }

        accum.abs()
    }

    pub fn noise(&self, p: &Point3) -> f32 {
        let u = p.x - p.x.floor();
        let v = p.y - p.y.floor();
        let w = p.z - p.z.floor();

        let xint = p.x.floor() as i32;
        let yint = p.y.floor() as i32;
        let zint = p.z.floor() as i32;

        // TODO: It's wasteful to initialize the vec3s when we're going to
        // replace them below.
        let mut c: [Vec3; 8] = [vec3(0.0, 0.0, 0.0); 8];

        #[inline(always)]
        fn idx(i: i32, j: i32, k: i32) -> usize {
            (k * 4 + j * 2 + i) as usize
        }

        for i in 0..2 {
            for j in 0..2 {
                for k in 0..2 {
                    let perm_x_idx = ((xint + i) & 255) as usize;
                    let perm_y_idx = ((yint + j) & 255) as usize;
                    let perm_z_idx = ((zint + k) & 255) as usize;

                    c[idx(i, j, k)] = self.ranvec[(self.perm_x[perm_x_idx]
                        ^ self.perm_y[perm_y_idx]
                        ^ self.perm_z[perm_z_idx])
                        as usize];
                }
            }
        }

        // Hermitian Smoothing
        let uu = u * u * (3.0 - 2.0 * u);
        let vv = v * v * (3.0 - 2.0 * v);
        let ww = w * w * (3.0 - 2.0 * w);

        // Perlin interpolation
        let mut accum: f32 = 0.0;

        for i in 0..2 {
            for j in 0..2 {
                for k in 0..2 {
                    let weight_v = vec3(u - i as f32, v - j as f32, w - k as f32);
                    let rv = c[idx(i, j, k)];

                    accum += (i as f32 * uu + (1.0 - i as f32) * (1.0 - uu))
                        * (j as f32 * vv + (1.0 - j as f32) * (1.0 - vv))
                        * (k as f32 * ww + (1.0 - k as f32) * (1.0 - ww))
                        * rv.dot(&weight_v);
                }
            }
        }

        accum
    }

    fn generate_perm(rng: &mut rand::rngs::SmallRng) -> Vec<i32> {
        let mut indices: Vec<i32> = Vec::with_capacity(POINT_COUNT as usize);
        indices.extend(0..POINT_COUNT);

        for i in (1..=(POINT_COUNT - 1)).rev() {
            let j = rng.gen_range(0..i);
            indices.swap(i as usize, j as usize);
        }

        indices
    }
}
