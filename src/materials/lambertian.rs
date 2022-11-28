use crate::{
    hittables::Hit,
    ray::Ray,
    textures::{SolidColor, Texture},
    vec3::{Color, Vec3},
};

use super::{Material, Scatter};

#[derive(Debug, Clone, Copy)]
pub struct Lambertian<T: Texture> {
    albedo: T,
}

impl Lambertian<SolidColor> {
    pub fn new_with_color(color: Color) -> Lambertian<SolidColor> {
        let albedo = SolidColor::new(color);
        Lambertian { albedo }
    }
}

impl<T: Texture> Lambertian<T> {
    pub fn new(albedo: T) -> Lambertian<T> {
        Lambertian { albedo }
    }
}

impl<T: Texture> Material for Lambertian<T> {
    fn scatter(&self, ray: &Ray, hit: &Hit) -> Option<Scatter> {
        let mut scatter_direction = hit.normal + Vec3::random_in_unit_sphere().unit();
        if scatter_direction.is_near_zero() {
            scatter_direction = hit.normal;
        }
        Some(Scatter {
            attenuation: self.albedo.value(hit.u, hit.v, &hit.p),
            ray: Ray::new(hit.p, scatter_direction, ray.time),
        })
    }
}
