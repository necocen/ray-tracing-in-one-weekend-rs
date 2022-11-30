use crate::{ray::Ray, vec3::Vec3};

use super::{Aabb, Hit, Hittable};

#[derive(Debug, Clone)]
pub struct Translate<H: Hittable> {
    hittable: H,
    offset: Vec3,
}

impl<H: Hittable> Translate<H> {
    pub fn new(hittable: H, offset: Vec3) -> Translate<H> {
        Translate { hittable, offset }
    }
}

impl<H: Hittable> Hittable for Translate<H> {
    fn hit(&self, ray: Ray, t_min: f64, t_max: f64) -> Option<Hit> {
        let moved_ray = Ray::new(ray.origin - self.offset, ray.direction, ray.time);
        let Some(mut hit) = self.hittable.hit(moved_ray, t_min, t_max) else {
            return None;
        };
        hit.p += self.offset;
        hit.set_face_normal(&moved_ray, hit.normal);
        Some(hit)
    }

    fn bounding_box(&self, time0: f64, time1: f64) -> Option<Aabb> {
        let Some(mut aabb) = self.hittable.bounding_box(time0, time1) else {
            return None;
        };
        aabb.min += self.offset;
        aabb.max += self.offset;

        Some(aabb)
    }
}
