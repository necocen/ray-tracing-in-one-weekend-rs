use crate::{
    material::Material,
    ray::Ray,
    vec3::{Point3, Vec3},
};

#[derive(Debug)]
#[non_exhaustive]
pub struct HitRecord<'a> {
    pub p: Point3,
    /// The normal always point against the hitting ray
    pub normal: Vec3,
    pub t: f64,
    /// True if hitting ray is outgoing to surface
    pub front_face: bool,
    pub material: &'a dyn Material,
}

impl<'a> HitRecord<'a> {
    #[allow(dead_code)]
    pub fn new(
        p: Point3,
        normal: Vec3,
        t: f64,
        front_face: bool,
        material: &'a dyn Material,
    ) -> HitRecord<'a> {
        HitRecord {
            p,
            normal,
            t,
            front_face,
            material,
        }
    }

    pub fn new_with_outward_normal(
        ray: Ray,
        t: f64,
        outward_normal: Vec3,
        material: &'a dyn Material,
    ) -> HitRecord<'a> {
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
            material,
        }
    }
}

pub trait Hittable: Send + Sync {
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
