use std::mem::swap;

use crate::{ray::Ray, vec3::Point3};

#[derive(Debug, Clone, Copy, PartialEq)]
#[non_exhaustive]
pub struct Aabb {
    pub min: Point3,
    pub max: Point3,
}

impl Aabb {
    pub fn new(min: Point3, max: Point3) -> Aabb {
        Aabb { min, max }
    }

    pub fn hit(&self, ray: &Ray, mut t_min: f64, mut t_max: f64) -> bool {
        for i in 0..3 {
            let inv = 1.0 / ray.direction[i];
            let mut t0 = (self.min[i] - ray.origin[i]) * inv;
            let mut t1 = (self.max[i] - ray.origin[i]) * inv;
            if inv < 0.0 {
                swap(&mut t0, &mut t1);
            }
            t_min = f64::max(t_min, t0);
            t_max = f64::min(t_max, t1);
            if t_max <= t_min {
                return false;
            }
        }
        true
    }

    pub fn union(&self, other: &Aabb) -> Aabb {
        let min = Point3::new(
            f64::min(self.min.x(), other.min.x()),
            f64::min(self.min.y(), other.min.y()),
            f64::min(self.min.z(), other.min.z()),
        );
        let max = Point3::new(
            f64::max(self.max.x(), other.max.x()),
            f64::max(self.max.y(), other.max.y()),
            f64::max(self.max.z(), other.max.z()),
        );
        Aabb::new(min, max)
    }
}

#[cfg(test)]
mod tests {
    use crate::vec3::Vec3;

    use super::*;

    #[test]
    fn test_hit_simple() {
        let aabb = Aabb::new(Point3::new(-0.5, -0.5, 0.0), Point3::new(0.5, 0.5, 1.0));
        let ray = Ray::new(Point3::new(0.0, 0.0, 0.0), Vec3::new(0.0, 0.0, 1.0), 0.0);

        assert!(aabb.hit(&ray, 0.0, 1.0));
        assert!(!aabb.hit(&ray, -1.0, 0.0));
        assert!(aabb.hit(&ray, -1.0, 0.5));
        assert!(!aabb.hit(&ray, 1.0, 2.0));
        assert!(aabb.hit(&ray, 0.5, 2.0));
    }

    #[test]
    fn test_hit_complex() {
        let aabb = Aabb::new(Point3::new(-1.0, -1.0, 0.0), Point3::new(1.0, 1.0, 1.0));
        let ray = Ray::new(
            Point3::new(0.0, 0.0, -0.5),
            Vec3::new(1.0, 1.0, 1.0) / 3.0f64.sqrt(),
            0.0,
        );

        assert!(!aabb.hit(&ray, 0.0, 0.5 * 3.0f64.sqrt() - f64::EPSILON));
        assert!(!aabb.hit(&ray, 3.0f64.sqrt(), 2.0));
        assert!(aabb.hit(&ray, 0.0, 1.0));
        assert!(aabb.hit(&ray, 1.0, 2.0));
    }

    #[test]
    fn test_union() {
        let aabb1 = Aabb::new(Point3::new(0.0, 0.0, 0.0), Point3::new(1.0, 1.0, 1.0));
        let aabb2 = Aabb::new(Point3::new(-1.0, -1.0, -1.0), Point3::new(0.0, 0.0, 0.0));

        assert_eq!(
            Aabb::new(Point3::new(-1.0, -1.0, -1.0), Point3::new(1.0, 1.0, 1.0)),
            aabb1.union(&aabb2)
        )
    }
}
