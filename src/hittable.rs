use crate::{
    ray::Ray,
    vec3::{Point3, Vec3},
};

#[derive(Debug, Clone, Copy, PartialEq)]
#[non_exhaustive]
pub struct HitRecord {
    pub p: Point3,
    pub normal: Vec3,
    pub t: f64,
}

pub trait Hittable {
    fn hit(&self, ray: Ray, t_min: f64, f_max: f64) -> Option<HitRecord>;
}
