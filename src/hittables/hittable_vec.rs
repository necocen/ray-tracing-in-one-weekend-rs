use crate::ray::Ray;

use super::{hit::Hit, Aabb, Hittable};

pub type HittableVec = Vec<Box<dyn Hittable>>;

impl Hittable for HittableVec {
    fn hit(&self, ray: &Ray, t_min: f64, t_max: f64) -> Option<Hit> {
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

    fn bounding_box(&self, time0: f64, time1: f64) -> Option<Aabb> {
        if self.is_empty() {
            return None;
        }

        let mut result: Option<Aabb> = None;
        for object in self.iter() {
            let Some(b) = object.bounding_box(time0, time1) else { continue; };
            if let Some(r) = result {
                result = Some(r.union(&b))
            } else {
                result = Some(b)
            }
        }

        result
    }
}
