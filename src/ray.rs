use crate::vec3::Vec3;

#[derive(Debug, Clone, Copy, PartialEq, Default)]
#[non_exhaustive]
pub struct Ray {
    pub origin: Vec3,
    pub direction: Vec3,
}

impl Ray {
    pub fn new(origin: Vec3, direction: Vec3) -> Ray {
        Ray { origin, direction }
    }

    pub fn at(&self, t: f64) -> Vec3 {
        self.origin + t * self.direction
    }
}

#[cfg(test)]
mod tests {
    use crate::vec3::Vec3;

    use super::Ray;

    #[test]
    fn test_at() {
        let ray = Ray::new(Vec3::new(1f64, 2f64, 3f64), Vec3::new(4f64, 5f64, 6f64));

        let v0 = Vec3::new(1f64, 2f64, 3f64);
        let v1 = Vec3::new(5f64, 7f64, 9f64);
        let vm1 = Vec3::new(-3f64, -3f64, -3f64);
        let v15 = Vec3::new(7f64, 9.5f64, 12f64);

        assert_eq!(v0, ray.at(0f64));
        assert_eq!(v1, ray.at(1f64));
        assert_eq!(vm1, ray.at(-1f64));
        assert_eq!(v15, ray.at(1.5f64));
    }
}
