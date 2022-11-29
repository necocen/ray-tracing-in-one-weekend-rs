use rand::{seq::SliceRandom, Rng};

use crate::vec3::Point3;

const POINT_COUNT: usize = 256;

#[derive(Debug, Clone)]
pub struct Perlin {
    rand_float: [f64; POINT_COUNT],
    perm_x: [usize; POINT_COUNT],
    perm_y: [usize; POINT_COUNT],
    perm_z: [usize; POINT_COUNT],
}

impl Perlin {
    pub fn new() -> Perlin {
        let mut rng = rand::thread_rng();
        let mut rand_float = [0.0; POINT_COUNT];
        for r in rand_float.iter_mut() {
            *r = rng.gen_range(0.0..1.0);
        }
        let mut perm_x = [0usize; POINT_COUNT];
        let mut perm_y = [0usize; POINT_COUNT];
        let mut perm_z = [0usize; POINT_COUNT];
        for i in 0..POINT_COUNT {
            perm_x[i] = i;
            perm_y[i] = i;
            perm_z[i] = i;
        }
        perm_x.shuffle(&mut rng);
        perm_y.shuffle(&mut rng);
        perm_z.shuffle(&mut rng);

        Perlin {
            rand_float,
            perm_x,
            perm_y,
            perm_z,
        }
    }

    pub fn noise_at(&self, p: &Point3) -> f64 {
        let i = (4.0 * p.x().abs()) as usize & 0xff;
        let j = (4.0 * p.y().abs()) as usize & 0xff;
        let k = (4.0 * p.z().abs()) as usize & 0xff;

        self.rand_float[self.perm_x[i] ^ self.perm_y[j] ^ self.perm_z[k]]
    }
}
