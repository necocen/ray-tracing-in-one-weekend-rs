use crate::{ray::Ray, vec3::Color};

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
