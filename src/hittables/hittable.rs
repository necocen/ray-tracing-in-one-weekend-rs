use crate::ray::Ray;

use super::{hit::Hit, Aabb};

pub trait Hittable: Send + Sync {
    fn hit(&self, ray: &Ray, t_min: f64, t_max: f64) -> Option<Hit>;
    fn bounding_box(&self, time0: f64, time1: f64) -> Option<Aabb>;
}
