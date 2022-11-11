use crate::{hittable::HitRecord, ray::Ray, vec3::Color};

pub trait Material: std::fmt::Debug + Send + Sync {
    fn scatter(&self, ray: &Ray, hit: &HitRecord) -> Option<Scatter>;
}

#[derive(Debug, Clone, Copy)]
#[non_exhaustive]
pub struct Scatter {
    pub attenuation: Color,
    pub ray: Ray,
}

impl Scatter {
    pub fn new(attenuation: Color, ray: Ray) -> Scatter {
        Scatter { attenuation, ray }
    }
}
