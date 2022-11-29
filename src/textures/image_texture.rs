use std::path::Path;

use anyhow::Result;
use image::{io::Reader, RgbImage};

use crate::vec3::{Color, Point3};

use super::Texture;

#[derive(Debug, Clone)]
pub struct ImageTexture {
    image: RgbImage,
}

impl ImageTexture {
    pub fn new_with_filename(filename: impl AsRef<Path>) -> Result<ImageTexture> {
        let image = Reader::open(filename)?.decode()?.into_rgb8();
        Ok(ImageTexture { image })
    }
}

impl Texture for ImageTexture {
    fn value(&self, mut u: f64, mut v: f64, p: &Point3) -> Color {
        // Clamp input texture coordinates to [0,1] x [1,0]
        u = u.clamp(0.0, 1.0);
        v = 1.0 - v.clamp(0.0, 1.0);
        let i = ((u * self.image.width() as f64) as u32).min(self.image.width() - 1);
        let j = ((v * self.image.height() as f64) as u32).min(self.image.height() - 1);

        let color_scale = 1.0 / 255.0;
        let pixel = self.image.get_pixel(i, j);
        Color::new(
            pixel[0] as f64 * color_scale,
            pixel[1] as f64 * color_scale,
            pixel[2] as f64 * color_scale,
        )
    }
}
