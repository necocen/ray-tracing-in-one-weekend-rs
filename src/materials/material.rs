use crate::{
    hittables::Hit,
    ray::Ray,
    vec3::{Color, Point3},
};

use super::Scatter;

pub trait Material: Send + Sync {
    fn scatter(&self, ray: &Ray, hit: &Hit) -> Option<Scatter>;

    #[allow(unused_variables)]
    fn emitted(&self, u: f64, v: f64, p: &Point3) -> Color {
        Color::default()
    }
}
