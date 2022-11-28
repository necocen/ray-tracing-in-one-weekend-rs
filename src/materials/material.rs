use crate::{hittables::Hit, ray::Ray};

use super::Scatter;

pub trait Material: Send + Sync {
    fn scatter(&self, ray: &Ray, hit: &Hit) -> Option<Scatter>;
}
