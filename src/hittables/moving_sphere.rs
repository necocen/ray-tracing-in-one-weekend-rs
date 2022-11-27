use crate::{
    materials::Material,
    ray::Ray,
    vec3::{Point3, Vec3},
};

use super::{Aabb, Hit, Hittable};

#[derive(Debug)]
#[non_exhaustive]
pub struct MovingSphere<M: Material> {
    pub center0: Point3,
    pub center1: Point3,
    pub time0: f64,
    pub time1: f64,
    pub radius: f64,
    pub material: M,
}

impl<M: Material> MovingSphere<M> {
    pub fn new(
        center0: Point3,
        center1: Point3,
        time0: f64,
        time1: f64,
        radius: f64,
        material: M,
    ) -> MovingSphere<M> {
        MovingSphere {
            center0,
            center1,
            time0,
            time1,
            radius,
            material,
        }
    }

    fn center_at(&self, time: f64) -> Point3 {
        self.center0
            + (time - self.time0) / (self.time1 - self.time0) * (self.center1 - self.center0)
    }
}

impl<M: Material> Hittable for MovingSphere<M> {
    fn hit(&self, ray: Ray, t_min: f64, t_max: f64) -> Option<Hit> {
        let oc = ray.origin - self.center_at(ray.time);
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

        let h = Hit::new_with_outward_normal(
            ray,
            root,
            (p - self.center_at(ray.time)) / self.radius,
            &self.material,
        );
        Some(h)
    }

    fn bounding_box(&self, time0: f64, time1: f64) -> Option<Aabb> {
        let box0 = Aabb::new(
            self.center_at(time0) - Vec3::new(self.radius, self.radius, self.radius),
            self.center_at(time0) + Vec3::new(self.radius, self.radius, self.radius),
        );
        let box1 = Aabb::new(
            self.center_at(time1) - Vec3::new(self.radius, self.radius, self.radius),
            self.center_at(time1) + Vec3::new(self.radius, self.radius, self.radius),
        );
        Some(box0.union(&box1))
    }
}
