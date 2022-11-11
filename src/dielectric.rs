use crate::{
    hittable::HitRecord,
    material::{Material, Scatter},
    ray::Ray,
    vec3::{Color, Vec3},
};

#[derive(Debug, Clone)]
pub struct Dielectric {
    eta: f64,
}

impl Dielectric {
    pub fn new(eta: f64) -> Dielectric {
        Dielectric { eta }
    }

    fn refract(uv: Vec3, n: Vec3, refraction_ratio: f64) -> Vec3 {
        let cos_theta = (-uv.dot(n)).min(1.0);
        let perpendicular = refraction_ratio * (uv + cos_theta * n);
        let parallel = -(1.0 - perpendicular.length_squared()).abs().sqrt() * n;
        perpendicular + parallel
    }

    fn reflect(v: Vec3, n: Vec3) -> Vec3 {
        v - 2.0 * v.dot(n) * n
    }
}

impl Material for Dielectric {
    fn scatter(&self, ray: &Ray, hit: &HitRecord) -> Option<Scatter> {
        let refraction_ratio = if hit.front_face {
            1.0 / self.eta
        } else {
            self.eta
        };
        let unit_direction = ray.direction.unit();
        let cos_theta = (-unit_direction.dot(hit.normal)).min(1.0);
        let sin_theta = (1.0 - cos_theta * cos_theta).sqrt();
        let cannot_refract = refraction_ratio * sin_theta > 1.0;
        let direction = if cannot_refract {
            Dielectric::reflect(unit_direction, hit.normal)
        } else {
            Dielectric::refract(unit_direction, hit.normal, refraction_ratio)
        };
        Some(Scatter::new(
            Color::new(1.0, 1.0, 1.0),
            Ray::new(hit.p, direction),
        ))
    }
}
