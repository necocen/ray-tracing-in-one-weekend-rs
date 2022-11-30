use crate::{materials::Material, ray::Ray, vec3::Point3};

use super::{Aabb, Hit, Hittable, HittableVec, XyRect, XzRect, YzRect};

pub struct Box<M: Material + Clone> {
    box_min: Point3,
    box_max: Point3,
    sides: HittableVec,
    material: M,
}

impl<M: Material + Clone + 'static> Box<M> {
    pub fn new(box_min: Point3, box_max: Point3, material: M) -> Box<M> {
        let sides: HittableVec = vec![
            std::boxed::Box::new(XyRect::new(
                box_min.x(),
                box_max.x(),
                box_min.y(),
                box_max.y(),
                box_max.z(),
                material.clone(),
            )),
            std::boxed::Box::new(XyRect::new(
                box_min.x(),
                box_max.x(),
                box_min.y(),
                box_max.y(),
                box_min.z(),
                material.clone(),
            )),
            std::boxed::Box::new(XzRect::new(
                box_min.x(),
                box_max.x(),
                box_min.z(),
                box_max.z(),
                box_max.y(),
                material.clone(),
            )),
            std::boxed::Box::new(XzRect::new(
                box_min.x(),
                box_max.x(),
                box_min.z(),
                box_max.z(),
                box_min.y(),
                material.clone(),
            )),
            std::boxed::Box::new(YzRect::new(
                box_min.y(),
                box_max.y(),
                box_min.z(),
                box_max.z(),
                box_max.x(),
                material.clone(),
            )),
            std::boxed::Box::new(YzRect::new(
                box_min.y(),
                box_max.y(),
                box_min.z(),
                box_max.z(),
                box_min.x(),
                material.clone(),
            )),
        ];

        Box {
            box_min,
            box_max,
            sides,
            material,
        }
    }
}

impl<M: Material + Clone> Hittable for Box<M> {
    fn hit(&self, ray: &Ray, t_min: f64, t_max: f64) -> Option<Hit> {
        self.sides.hit(ray, t_min, t_max)
    }

    fn bounding_box(&self, _time0: f64, _time1: f64) -> Option<Aabb> {
        Some(Aabb::new(self.box_min, self.box_max))
    }
}
