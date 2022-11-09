use crate::{
    ray::Ray,
    vec3::{Point3, Vec3},
};

#[derive(Debug, Clone, Copy, PartialEq)]
#[non_exhaustive]
pub struct HitRecord {
    pub p: Point3,
    /// The normal always point against the hitting ray
    pub normal: Vec3,
    pub t: f64,
    /// True if hitting ray is outgoing to surface
    pub front_face: bool,
}

impl HitRecord {
    pub fn new(p: Point3, normal: Vec3, t: f64, front_face: bool) -> HitRecord {
        HitRecord {
            p,
            normal,
            t,
            front_face,
        }
    }

    pub fn new_with_outward_normal(ray: Ray, t: f64, outward_normal: Vec3) -> HitRecord {
        let front_face = ray.direction.dot(outward_normal) < 0.0;
        let normal = if front_face {
            outward_normal
        } else {
            -outward_normal
        };
        HitRecord {
            p: ray.at(t),
            normal,
            t,
            front_face,
        }
    }
}

pub trait Hittable {
    fn hit(&self, ray: Ray, t_min: f64, t_max: f64) -> Option<HitRecord>;
}

pub type HittableList = Vec<Box<dyn Hittable>>;

impl Hittable for HittableList {
    fn hit(&self, ray: Ray, t_min: f64, t_max: f64) -> Option<HitRecord> {
        let mut t_max = t_max;
        let mut record: Option<HitRecord> = None;
        for hittable in self.iter() {
            if let Some(r) = hittable.hit(ray, t_min, t_max) {
                t_max = r.t;
                record = Some(r);
            }
        }
        record
    }
}
