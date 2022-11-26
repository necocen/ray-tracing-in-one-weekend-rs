use crate::{
    hittables::Hit,
    ray::Ray,
    vec3::{Color, Vec3},
};

use super::{Material, Scatter};

#[derive(Debug, Clone, Copy)]
pub struct Lambertian {
    albedo: Color,
}

impl Lambertian {
    pub fn new(albedo: Color) -> Lambertian {
        Lambertian { albedo }
    }
}

impl Material for Lambertian {
    fn scatter(&self, ray: &Ray, hit: &Hit) -> Option<Scatter> {
        let mut scatter_direction = hit.normal + Vec3::random_in_unit_sphere().unit();
        if scatter_direction.is_near_zero() {
            scatter_direction = hit.normal;
        }
        Some(Scatter {
            attenuation: self.albedo,
            ray: Ray::new_with_time(hit.p, scatter_direction, ray.time),
        })
    }
}
