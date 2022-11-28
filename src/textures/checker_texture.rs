use std::fmt::Debug;

use crate::vec3::{Color, Point3};

use super::{SolidColor, Texture};

#[derive(Debug)]
pub struct CheckerTexture<O: Texture + Debug, E: Texture + Debug> {
    odd: O,
    even: E,
}

impl CheckerTexture<SolidColor, SolidColor> {
    pub fn new_with_colors(odd: Color, even: Color) -> CheckerTexture<SolidColor, SolidColor> {
        CheckerTexture {
            odd: SolidColor::new(odd),
            even: SolidColor::new(even),
        }
    }
}

impl<O: Texture + Debug, E: Texture + Debug> CheckerTexture<O, E> {
    pub fn new(odd: O, even: E) -> CheckerTexture<O, E> {
        CheckerTexture { odd, even }
    }
}

impl<O: Texture + Debug, E: Texture + Debug> Texture for CheckerTexture<O, E> {
    fn value(&self, u: f64, v: f64, p: &Point3) -> Color {
        let sines = ((10.0 * p.x()).sin()) * ((10.0 * p.y()).sin()) * ((10.0 * p.z()).sin());
        if sines < 0.0 {
            self.odd.value(u, v, p)
        } else {
            self.even.value(u, v, p)
        }
    }
}
