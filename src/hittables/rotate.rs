use crate::{ray::Ray, vec3::Point3};

use super::{Aabb, Hit, Hittable};

#[derive(Debug, Clone)]
pub struct RotateY<H: Hittable> {
    hittable: H,
    sin_theta: f64,
    cos_theta: f64,
    bbox: Option<Aabb>,
}

impl<H: Hittable> RotateY<H> {
    pub fn new(hittable: H, theta: f64) -> RotateY<H> {
        let sin_theta = theta.sin();
        let cos_theta = theta.cos();
        let Some(bbox) = hittable.bounding_box(0.0, 1.0) else {
            return RotateY {
                hittable,
                sin_theta,
                cos_theta,
                bbox: None,
            }
        };

        let mut min = Point3::new(f64::INFINITY, f64::INFINITY, f64::INFINITY);
        let mut max = Point3::new(-f64::INFINITY, -f64::INFINITY, -f64::INFINITY);

        for j in 0..2 {
            let y = j as f64 * bbox.max.y() + (1 - j) as f64 * bbox.min.y();
            min[1] = min[1].min(y);
            max[1] = max[1].max(y);
            for i in 0..2 {
                for k in 0..2 {
                    let x = i as f64 * bbox.max.x() + (1 - i) as f64 * bbox.min.x();
                    let z = k as f64 * bbox.max.z() + (1 - k) as f64 * bbox.min.z();
                    let new_x = cos_theta * x + sin_theta * z;
                    let new_z = -sin_theta * x + cos_theta * z;
                    min[0] = min[0].min(new_x);
                    max[0] = max[0].max(new_x);
                    min[2] = min[2].min(new_z);
                    max[2] = max[2].max(new_z);
                }
            }
        }

        RotateY {
            hittable,
            sin_theta,
            cos_theta,
            bbox: Some(Aabb::new(min, max)),
        }
    }
}

impl<H: Hittable> Hittable for RotateY<H> {
    fn hit(&self, ray: &Ray, t_min: f64, t_max: f64) -> Option<Hit> {
        let mut origin = ray.origin;
        let mut direction = ray.direction;

        origin[0] = self.cos_theta * ray.origin.x() - self.sin_theta * ray.origin.z();
        origin[2] = self.sin_theta * ray.origin.x() + self.cos_theta * ray.origin.z();

        direction[0] = self.cos_theta * ray.direction.x() - self.sin_theta * ray.direction.z();
        direction[2] = self.sin_theta * ray.direction.x() + self.cos_theta * ray.direction.z();

        let rotated_ray = Ray::new(origin, direction, ray.time);
        let Some(mut hit) = self.hittable.hit(&rotated_ray, t_min, t_max) else {
            return None;
        };

        let mut p = hit.p;
        let mut normal = hit.normal;

        p[0] = self.cos_theta * hit.p.x() + self.sin_theta * hit.p.z();
        p[2] = -self.sin_theta * hit.p.x() + self.cos_theta * hit.p.z();

        normal[0] = self.cos_theta * hit.normal.x() + self.sin_theta * hit.normal.z();
        normal[2] = -self.sin_theta * hit.normal.x() + self.cos_theta * hit.normal.z();

        hit.p = p;
        hit.set_face_normal(&rotated_ray, normal);

        Some(hit)
    }

    fn bounding_box(&self, _time0: f64, _time1: f64) -> Option<Aabb> {
        // FIXME: timeを考慮してない
        self.bbox
    }
}
