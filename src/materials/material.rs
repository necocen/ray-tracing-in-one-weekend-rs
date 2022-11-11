use crate::{hittables::Hit, ray::Ray};

use super::Scatter;

pub trait Material: std::fmt::Debug + Send + Sync {
    fn scatter(&self, ray: &Ray, hit: &Hit) -> Option<Scatter>;
}
