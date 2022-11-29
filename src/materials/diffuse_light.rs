use crate::{
    hittables::Hit,
    ray::Ray,
    textures::{SolidColor, Texture},
    vec3::{Color, Point3},
};

use super::{Material, Scatter};

#[derive(Debug, Clone)]
pub struct DiffuseLight<T: Texture> {
    emit: T,
}

impl<T: Texture> DiffuseLight<T> {
    pub fn new(emit: T) -> DiffuseLight<T> {
        DiffuseLight { emit }
    }
}

impl DiffuseLight<SolidColor> {
    pub fn new_with_color(color: Color) -> DiffuseLight<SolidColor> {
        DiffuseLight {
            emit: SolidColor::new(color),
        }
    }
}

impl<T: Texture> Material for DiffuseLight<T> {
    fn scatter(&self, _ray: &Ray, _hit: &Hit) -> Option<Scatter> {
        None
    }

    fn emitted(&self, u: f64, v: f64, p: &Point3) -> Color {
        self.emit.value(u, v, p)
    }
}
