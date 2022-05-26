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
        let i = ((4.0 * p.x) as i32) & 255;
        let j = ((4.0 * p.y) as i32) & 255;
        let k = ((4.0 * p.z) as i32) & 255;

        let idx = self.perm_x[i as usize] ^ self.perm_y[j as usize] ^ self.perm_z[k as usize];

        self.ranfloat[idx as usize]
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
