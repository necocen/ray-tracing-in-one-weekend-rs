use std::ops;

#[derive(Debug, Clone, Copy, Default, PartialEq)]
pub struct Vec3([f64; 3]);

impl Vec3 {
    pub fn new(x: f64, y: f64, z: f64) -> Vec3 {
        Vec3([x, y, z])
    }

    pub fn x(&self) -> f64 {
        self.0[0]
    }

    pub fn y(&self) -> f64 {
        self.0[1]
    }

    pub fn z(&self) -> f64 {
        self.0[2]
    }

    pub fn length(&self) -> f64 {
        self.length_squared().sqrt()
    }

    pub fn length_squared(&self) -> f64 {
        self.0[0] * self.0[0] + self.0[1] * self.0[1] + self.0[2] * self.0[2]
    }
}

impl ops::Neg for Vec3 {
    type Output = Vec3;
    fn neg(self) -> Self::Output {
        Vec3([-self.0[0], -self.0[1], -self.0[2]])
    }
}

impl ops::AddAssign for Vec3 {
    fn add_assign(&mut self, rhs: Self) {
        self.0[0] += rhs.0[0];
        self.0[1] += rhs.0[1];
        self.0[2] += rhs.0[2];
    }
}

impl ops::MulAssign<f64> for Vec3 {
    fn mul_assign(&mut self, rhs: f64) {
        self.0[0] *= rhs;
        self.0[1] *= rhs;
        self.0[2] *= rhs;
    }
}

impl ops::DivAssign<f64> for Vec3 {
    fn div_assign(&mut self, rhs: f64) {
        self.0[0] /= rhs;
        self.0[1] /= rhs;
        self.0[2] /= rhs;
    }
}

impl ops::Index<usize> for Vec3 {
    type Output = f64;

    fn index(&self, index: usize) -> &Self::Output {
        &self.0[index]
    }
}

impl ops::IndexMut<usize> for Vec3 {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        &mut self.0[index]
    }
}

type Point3 = Vec3;
type Color = Vec3;

#[cfg(test)]
mod test {
    use super::Vec3;

    #[test]
    fn test_default() {
        let v = Vec3::default();
        assert_eq!(0f64, v.x());
        assert_eq!(0f64, v.y());
        assert_eq!(0f64, v.z());
    }

    #[test]
    fn test_new() {
        let v = Vec3::new(1f64, 2f64, 3f64);
        assert_eq!(1f64, v.x());
        assert_eq!(2f64, v.y());
        assert_eq!(3f64, v.z());
    }

    #[test]
    fn test_length() {
        let v = Vec3::new(1f64, 2f64, 3f64);
        assert_eq!(14f64, v.length_squared());
        assert_eq!(3.741_657_386_773_941_3_f64, v.length());
    }

    #[test]
    fn test_neg() {
        let v = Vec3::new(1f64, 2f64, 3f64);
        assert_eq!(-1f64, (-v).x());
        assert_eq!(-2f64, (-v).y());
        assert_eq!(-3f64, (-v).z());
    }

    #[test]
    fn test_add_assign() {
        let mut v1 = Vec3::new(1f64, 2f64, 3f64);
        let v2 = Vec3::new(4f64, 5f64, 6f64);
        v1 += v2;
        assert_eq!(5f64, v1.x());
        assert_eq!(7f64, v1.y());
        assert_eq!(9f64, v1.z());
    }

    #[test]
    fn test_mul_assign() {
        let mut v1 = Vec3::new(1f64, 2f64, 3f64);
        let t = 3f64;
        v1 *= t;
        assert_eq!(3f64, v1.x());
        assert_eq!(6f64, v1.y());
        assert_eq!(9f64, v1.z());
    }

    #[test]
    fn test_div_assign() {
        let mut v1 = Vec3::new(1f64, 2f64, 3f64);
        let t = 2f64;
        v1 /= t;
        assert_eq!(0.5f64, v1.x());
        assert_eq!(1f64, v1.y());
        assert_eq!(1.5f64, v1.z());
    }

    #[test]
    fn test_index() {
        let v = Vec3::new(1f64, 2f64, 3f64);
        assert_eq!(1f64, v[0]);
        assert_eq!(2f64, v[1]);
        assert_eq!(3f64, v[2]);
    }

    #[test]
    fn test_index_mut() {
        let mut v = Vec3::new(1f64, 2f64, 3f64);
        v[0] = 4f64;
        v[1] = 5f64;
        v[2] = 6f64;
        assert_eq!(4f64, v.x());
        assert_eq!(5f64, v.y());
        assert_eq!(6f64, v.z());
    }

    #[test]
    fn test_partial_eq() {
        let v1 = Vec3::new(1f64, 2f64, 3f64);
        let v2 = Vec3::new(1f64, 2f64, 3f64);
        assert_eq!(v1, v2);

        let v3 = Vec3::new(1f64, 2f64, 0f64);
        assert_ne!(v1, v3);
    }
}
