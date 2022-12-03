use std::{
    fmt::Display,
    io, iter,
    ops::{self, Range},
};

use rand::Rng;
use rand_distr::{Distribution, UnitBall, UnitDisc};

#[derive(Debug, Clone, Copy, Default, PartialEq)]
pub struct Vec3([f64; 3]);

impl Vec3 {
    pub fn new(x: f64, y: f64, z: f64) -> Vec3 {
        Vec3([x, y, z])
    }

    pub fn x(self) -> f64 {
        self[0]
    }

    pub fn y(self) -> f64 {
        self[1]
    }

    pub fn z(self) -> f64 {
        self[2]
    }

    pub fn length(self) -> f64 {
        self.length_squared().sqrt()
    }

    pub fn length_squared(self) -> f64 {
        self[0] * self[0] + self[1] * self[1] + self[2] * self[2]
    }

    pub fn dot(self, rhs: Vec3) -> f64 {
        self[0] * rhs[0] + self[1] * rhs[1] + self[2] * rhs[2]
    }

    pub fn cross(self, rhs: Vec3) -> Vec3 {
        Vec3::new(
            self[1] * rhs[2] - self[2] * rhs[1],
            self[2] * rhs[0] - self[0] * rhs[2],
            self[0] * rhs[1] - self[1] * rhs[0],
        )
    }

    pub fn unit(self) -> Vec3 {
        self / self.length()
    }

    pub fn random() -> Vec3 {
        let mut rng = rand::thread_rng();
        let mut v = [0f64; 3];
        rng.fill(&mut v);
        Vec3(v)
    }

    pub fn random_range(range: Range<f64>) -> Vec3 {
        Self::random() * (range.end - range.start)
            + Vec3::new(range.start, range.start, range.start)
    }

    pub fn random_in_unit_sphere() -> Vec3 {
        let mut rng = rand::thread_rng();
        let v: [f64; 3] = UnitBall.sample(&mut rng);
        Vec3(v)
    }

    pub fn random_in_unit_disk() -> Vec3 {
        let mut rng = rand::thread_rng();
        let v: [f64; 2] = UnitDisc.sample(&mut rng);
        Vec3([v[0], v[1], 1.0])
    }

    pub fn is_near_zero(&self) -> bool {
        let s = 1e-8;
        self[0].abs() < s && self[1].abs() < s && self[2].abs() < s
    }
}

impl ops::Neg for Vec3 {
    type Output = Vec3;
    fn neg(self) -> Self::Output {
        Vec3([-self[0], -self[1], -self[2]])
    }
}

impl ops::AddAssign for Vec3 {
    fn add_assign(&mut self, rhs: Self) {
        self[0] += rhs[0];
        self[1] += rhs[1];
        self[2] += rhs[2];
    }
}

impl ops::MulAssign<f64> for Vec3 {
    fn mul_assign(&mut self, rhs: f64) {
        self[0] *= rhs;
        self[1] *= rhs;
        self[2] *= rhs;
    }
}

impl ops::DivAssign<f64> for Vec3 {
    fn div_assign(&mut self, rhs: f64) {
        self[0] /= rhs;
        self[1] /= rhs;
        self[2] /= rhs;
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

impl ops::Add for Vec3 {
    type Output = Vec3;

    fn add(self, rhs: Self) -> Self::Output {
        Vec3::new(self[0] + rhs[0], self[1] + rhs[1], self[2] + rhs[2])
    }
}

impl ops::Sub for Vec3 {
    type Output = Vec3;

    fn sub(self, rhs: Self) -> Self::Output {
        Vec3::new(self[0] - rhs[0], self[1] - rhs[1], self[2] - rhs[2])
    }
}

impl ops::Mul for Vec3 {
    type Output = Vec3;

    fn mul(self, rhs: Self) -> Self::Output {
        Vec3::new(self[0] * rhs[0], self[1] * rhs[1], self[2] * rhs[2])
    }
}

impl ops::Mul<f64> for Vec3 {
    type Output = Vec3;

    fn mul(self, rhs: f64) -> Self::Output {
        Vec3::new(self[0] * rhs, self[1] * rhs, self[2] * rhs)
    }
}

impl ops::Mul<Vec3> for f64 {
    type Output = Vec3;

    fn mul(self, rhs: Vec3) -> Self::Output {
        Vec3::new(rhs[0] * self, rhs[1] * self, rhs[2] * self)
    }
}

impl ops::Div<f64> for Vec3 {
    type Output = Vec3;

    fn div(self, rhs: f64) -> Self::Output {
        Vec3::new(self[0] / rhs, self[1] / rhs, self[2] / rhs)
    }
}

impl Display for Vec3 {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} {} {}", self[0], self[1], self[2])
    }
}

impl iter::Sum for Vec3 {
    fn sum<I: Iterator<Item = Self>>(iter: I) -> Self {
        let mut v = Vec3::default();
        for i in iter {
            v += i;
        }
        v
    }
}

pub type Point3 = Vec3;
pub type Color = Vec3;

impl Color {
    pub fn write(self, w: &mut impl io::Write) -> io::Result<()> {
        let r = self.x().sqrt().clamp(0.0, 0.999);
        let g = self.y().sqrt().clamp(0.0, 0.999);
        let b = self.z().sqrt().clamp(0.0, 0.999);

        // Write the translated [0,255] value of each color component.
        let r = (255.999 * r) as i32;
        let g = (255.999 * g) as i32;
        let b = (255.999 * b) as i32;
        writeln!(w, "{r} {g} {b}")
    }
}

#[cfg(test)]
mod tests {
    use super::{Color, Vec3};

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

    #[test]
    fn test_add() {
        let v1 = Vec3::new(1f64, 2f64, 3f64);
        let v2 = Vec3::new(4f64, 5f64, 6f64);

        let v3 = Vec3::new(5f64, 7f64, 9f64);

        assert_eq!(v3, v1 + v2);
        assert_eq!(v3, v2 + v1);
    }

    #[test]
    fn test_sub() {
        let v1 = Vec3::new(1f64, 2f64, 3f64);
        let v2 = Vec3::new(4f64, 5f64, 6f64);

        let v3 = Vec3::new(-3f64, -3f64, -3f64);

        assert_eq!(v3, v1 - v2);
        assert_eq!(-v3, v2 - v1);
    }

    #[test]
    fn test_mul_vec() {
        let v1 = Vec3::new(1f64, 2f64, 3f64);
        let v2 = Vec3::new(4f64, 5f64, 6f64);

        let v3 = Vec3::new(4f64, 10f64, 18f64);

        assert_eq!(v3, v1 * v2);
    }

    #[test]
    fn test_mul_scalar() {
        let v1 = Vec3::new(1f64, 2f64, 3f64);

        let v3 = Vec3::new(3f64, 6f64, 9f64);

        assert_eq!(v3, v1 * 3f64);
    }

    #[test]
    fn test_div_scalar() {
        let v1 = Vec3::new(1f64, 2f64, 3f64);

        let v3 = Vec3::new(0.5f64, 1f64, 1.5f64);

        assert_eq!(v3, v1 / 2f64);
    }

    #[test]
    fn test_dot_product() {
        let v1 = Vec3::new(1f64, 2f64, 3f64);
        let v2 = Vec3::new(4f64, 5f64, 6f64);

        assert_eq!(32f64, v1.dot(v2));
    }

    #[test]
    fn test_cross_product() {
        let v1 = Vec3::new(1f64, 2f64, 3f64);
        let v2 = Vec3::new(4f64, 5f64, 6f64);

        let v3 = Vec3::new(-3f64, 6f64, -3f64);
        assert_eq!(v3, v1.cross(v2));
    }

    #[test]
    fn test_unit() {
        let v = Vec3::new(1f64, 2f64, 3f64);

        let expected = Vec3::new(
            0.2672612419124244f64,
            0.5345224838248488f64,
            0.8017837257372732f64,
        );
        assert_eq!(expected, v.unit());
    }

    #[test]
    fn test_display() {
        let v = Vec3::new(1f64, 2f64, 3f64);

        assert_eq!("1 2 3", v.to_string())
    }

    #[test]
    fn test_write_color() -> std::io::Result<()> {
        let mut buf: Vec<u8> = vec![];
        let color = Color::new(0.25, 0.25, 0.5);
        color.write(&mut buf)?;

        assert_eq!("127 127 181\n", String::from_utf8_lossy(&buf));
        Ok(())
    }
}
