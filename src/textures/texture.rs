use crate::vec3::{Color, Point3};

pub trait Texture {
    fn value(&self, u: f64, v: f64, p: &Point3) -> Color;
}
