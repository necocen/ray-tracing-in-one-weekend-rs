use crate::ray::Ray;

use super::hit::Hit;

pub trait Hittable: Send + Sync {
    fn hit(&self, ray: Ray, t_min: f64, t_max: f64) -> Option<Hit>;
}
