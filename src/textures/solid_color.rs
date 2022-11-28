use crate::vec3::{Color, Point3};

use super::Texture;

#[derive(Debug, Clone)]
pub struct SolidColor {
    color: Color,
}

impl SolidColor {
    pub fn new(color: Color) -> SolidColor {
        SolidColor { color }
    }

    pub fn new_with_rgb(r: f64, g: f64, b: f64) -> SolidColor {
        SolidColor {
            color: Color::new(r, g, b),
        }
    }
}

impl Texture for SolidColor {
    fn value(&self, _u: f64, _v: f64, _p: &Point3) -> Color {
        self.color
    }
}

#[cfg(test)]
mod tests {
    use crate::vec3::Vec3;

    use super::*;

    #[test]
    fn test_value_does_not_depend_on_u_v_p() {
        let solid_color = SolidColor::new_with_rgb(0.5, 0.5, 0.5);
        assert_eq!(
            Color::new(0.5, 0.5, 0.5),
            solid_color.value(0.0, 0.0, &Vec3::default())
        );
        assert_eq!(
            Color::new(0.5, 0.5, 0.5),
            solid_color.value(1.0, -1.0, &Vec3::default())
        );
        assert_eq!(
            Color::new(0.5, 0.5, 0.5),
            solid_color.value(0.0, 0.0, &Vec3::new(0.0, 1.0, 2.0))
        );
    }
}
