use rand::{seq::SliceRandom, Rng};

use crate::vec3::{Point3, Vec3};

const POINT_COUNT: usize = 256;

#[derive(Debug, Clone)]
pub struct Perlin {
    rand_vec: [Vec3; POINT_COUNT],
    perm_x: [usize; POINT_COUNT],
    perm_y: [usize; POINT_COUNT],
    perm_z: [usize; POINT_COUNT],
}

impl Perlin {
    pub fn new() -> Perlin {
        let mut rng = rand::thread_rng();
        let mut rand_vec = [Vec3::default(); POINT_COUNT];
        for r in rand_vec.iter_mut() {
            *r = Vec3::random()
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
            rand_vec,
            perm_x,
            perm_y,
            perm_z,
        }
    }

    pub fn noise_at(&self, p: &Point3) -> f64 {
        let u = p.x() - p.x().floor();
        let v = p.y() - p.y().floor();
        let w = p.z() - p.z().floor();
        let i = p.x().floor() as isize;
        let j = p.y().floor() as isize;
        let k = p.z().floor() as isize;
        let mut c = [[[Vec3::default(); 2]; 2]; 2];
        (0..2).for_each(|di| {
            (0..2).for_each(|dj| {
                (0..2).for_each(|dk| {
                    c[di][dj][dk] = self.rand_vec[self.perm_x[(i as usize + di) & 0xff]
                        ^ self.perm_y[(j as usize + dj) & 0xff]
                        ^ self.perm_z[(k as usize + dk) & 0xff]]
                });
            });
        });

        (Self::perlin_interpolation(c, u, v, w) + 1.0) * 0.5
    }

    fn perlin_interpolation(c: [[[Vec3; 2]; 2]; 2], u: f64, v: f64, w: f64) -> f64 {
        let uu = u * u * (3.0 - 2.0 * u);
        let vv = v * v * (3.0 - 2.0 * v);
        let ww = w * w * (3.0 - 2.0 * w);
        (0..2).fold(0.0, |acc, i| {
            acc + (0..2).fold(0.0, move |acc, j| {
                acc + (0..2).fold(0.0, move |acc, k| {
                    let weight = Vec3::new(u - i as f64, v - j as f64, w - k as f64);
                    acc + (i as f64 * uu + ((1 - i) as f64) * (1.0 - uu))
                        * (j as f64 * vv + ((1 - j) as f64) * (1.0 - vv))
                        * (k as f64 * ww + ((1 - k) as f64) * (1.0 - ww))
                        * c[i][j][k].dot(weight)
                })
            })
        })
    }
}
