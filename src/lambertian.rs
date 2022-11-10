use crate::{
    hittable::HitRecord,
    material::{Material, Scatter},
    ray::Ray,
    vec3::{Color, Vec3},
};

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
    fn scatter(&self, _ray: &Ray, hit: &HitRecord) -> Option<Scatter> {
        let mut scatter_direction = hit.normal + Vec3::random_in_unit_sphere().unit();
        if scatter_direction.is_near_zero() {
            scatter_direction = hit.normal;
        }
        Some(Scatter {
            attenuation: self.albedo,
            ray: Ray::new(hit.p, scatter_direction),
        })
    }
}
