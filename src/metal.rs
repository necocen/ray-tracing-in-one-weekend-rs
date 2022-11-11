use crate::{
    hittable::HitRecord,
    material::{Material, Scatter},
    ray::Ray,
    vec3::{Color, Vec3},
};

#[derive(Debug, Clone, Copy)]
pub struct Metal {
    albedo: Color,
    fuzziness: f64,
}

impl Metal {
    pub fn new(albedo: Color, fuzziness: f64) -> Metal {
        Metal {
            albedo,
            fuzziness: fuzziness.min(1.0),
        }
    }
}

impl Material for Metal {
    fn scatter(&self, ray: &Ray, hit: &HitRecord) -> Option<Scatter> {
        fn reflect(v: Vec3, n: Vec3) -> Vec3 {
            v - 2.0 * v.dot(n) * n
        }
        let reflected = reflect(ray.direction.unit(), hit.normal);
        let scattered = Ray::new(
            hit.p,
            reflected + self.fuzziness * Vec3::random_in_unit_sphere(),
        );
        if scattered.direction.dot(hit.normal) > 0.0 {
            Some(Scatter::new(self.albedo, scattered))
        } else {
            None
        }
    }
}
