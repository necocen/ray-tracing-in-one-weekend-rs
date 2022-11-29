use crate::{
    materials::Material,
    ray::Ray,
    vec3::{Point3, Vec3},
};

use super::{Aabb, Hit, Hittable};

#[derive(Debug, Clone)]
pub struct XyRect<M: Material> {
    x0: f64,
    x1: f64,
    y0: f64,
    y1: f64,
    z: f64,
    material: M,
}

impl<M: Material> XyRect<M> {
    pub fn new(x0: f64, x1: f64, y0: f64, y1: f64, z: f64, material: M) -> XyRect<M> {
        assert!(x0 < x1);
        assert!(y0 < y1);
        XyRect {
            x0,
            x1,
            y0,
            y1,
            z,
            material,
        }
    }
}

impl<M: Material> Hittable for XyRect<M> {
    fn hit(&self, ray: Ray, t_min: f64, t_max: f64) -> Option<Hit> {
        let t = (self.z - ray.origin.z()) / ray.direction.z();
        if t < t_min || t > t_max {
            return None;
        }
        let x = ray.origin.x() + t * ray.direction.x();
        let y = ray.origin.y() + t * ray.direction.y();
        if x < self.x0 || x > self.x1 || y < self.y0 || y > self.y1 {
            return None;
        }
        let u = (x - self.x0) / (self.x1 - self.x0);
        let v = (y - self.y0) / (self.y1 - self.y0);

        Some(Hit::new_with_outward_normal(
            ray,
            t,
            u,
            v,
            Vec3::new(0.0, 0.0, 1.0),
            &self.material,
        ))
    }

    fn bounding_box(&self, _time0: f64, _time1: f64) -> Option<Aabb> {
        let min = Point3::new(self.x0, self.y0, self.z - 0.0001);
        let max = Point3::new(self.x1, self.y1, self.z + 0.0001);
        Some(Aabb::new(min, max))
    }
}
