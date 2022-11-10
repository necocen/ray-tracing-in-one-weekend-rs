use crate::{
    hittable::{HitRecord, Hittable},
    material::Material,
    vec3::Point3,
};

#[derive(Debug)]
#[non_exhaustive]
pub struct Sphere<'a> {
    pub center: Point3,
    pub radius: f64,
    pub material: &'a dyn Material,
}

impl<'a> Sphere<'a> {
    pub fn new(center: Point3, radius: f64, material: &'a impl Material) -> Sphere<'a> {
        Sphere {
            center,
            radius,
            material,
        }
    }
}

impl<'a> Hittable for Sphere<'a> {
    fn hit(&self, ray: crate::ray::Ray, t_min: f64, t_max: f64) -> Option<HitRecord> {
        let oc = ray.origin - self.center;
        let a = ray.direction.length_squared();
        let half_b = oc.dot(ray.direction);
        let c = oc.dot(oc) - self.radius * self.radius;
        let discriminant = half_b * half_b - a * c;

        if discriminant < 0.0 {
            return None;
        }

        let sqrtd = discriminant.sqrt();
        // Find the nearest root that lies in the acceptable range
        let mut root = (-half_b - sqrtd) / a;
        if root < t_min || t_max < root {
            root = (-half_b + sqrtd) / a;
            if root < t_min || t_max < root {
                return None;
            }
        }

        let p = ray.at(root);

        let h = HitRecord::new_with_outward_normal(
            ray,
            root,
            (p - self.center) / self.radius,
            self.material,
        );
        Some(h)
    }
}
