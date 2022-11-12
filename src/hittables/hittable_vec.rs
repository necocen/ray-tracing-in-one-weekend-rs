use crate::ray::Ray;

use super::{hit::Hit, Hittable};

pub type HittableVec = Vec<Box<dyn Hittable>>;

impl Hittable for HittableVec {
    fn hit(&self, ray: Ray, t_min: f64, t_max: f64) -> Option<Hit> {
        let mut t_max = t_max;
        let mut record: Option<Hit> = None;
        for hittable in self.iter() {
            if let Some(r) = hittable.hit(ray, t_min, t_max) {
                t_max = r.t;
                record = Some(r);
            }
        }
        record
    }
}