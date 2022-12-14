use crate::{
    materials::Material,
    ray::Ray,
    vec3::{Point3, Vec3},
};

#[non_exhaustive]
pub struct Hit<'a> {
    pub p: Point3,
    /// The normal always point against the hitting ray
    pub normal: Vec3,
    pub t: f64,
    pub u: f64,
    pub v: f64,
    /// True if hitting ray is outgoing to surface
    pub front_face: bool,
    pub material: &'a dyn Material,
}

impl<'a> Hit<'a> {
    #[allow(dead_code)]
    pub fn new(
        p: Point3,
        normal: Vec3,
        t: f64,
        u: f64,
        v: f64,
        front_face: bool,
        material: &'a dyn Material,
    ) -> Hit<'a> {
        Hit {
            p,
            normal,
            t,
            u,
            v,
            front_face,
            material,
        }
    }

    pub fn new_with_outward_normal(
        ray: &Ray,
        t: f64,
        u: f64,
        v: f64,
        outward_normal: Vec3,
        material: &'a dyn Material,
    ) -> Hit<'a> {
        let front_face = ray.direction.dot(outward_normal) < 0.0;
        let normal = if front_face {
            outward_normal
        } else {
            -outward_normal
        };
        Hit {
            p: ray.at(t),
            normal,
            t,
            u,
            v,
            front_face,
            material,
        }
    }

    pub fn set_face_normal(&mut self, ray: &Ray, outward_normal: Vec3) {
        self.front_face = ray.direction.dot(outward_normal) < 0.0;
        if self.front_face {
            self.normal = outward_normal;
        } else {
            self.normal = -outward_normal;
        }
    }
}
