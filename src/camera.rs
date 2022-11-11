use crate::{
    ray::Ray,
    vec3::{Point3, Vec3},
};

#[derive(Debug, Clone)]
pub struct Camera {
    origin: Point3,
    lower_left_corner: Point3,
    horizontal: Vec3,
    vertical: Vec3,
}

impl Camera {
    /// - `look_from` - point that camera is looking from
    /// - `look_at` - point that camera is looking at
    /// - `v_up` - 'up' direction of camera
    /// - `theta` - vertical field-of-view in radians
    /// - `aspect_Ratio` - aspect ratio of viewport
    pub fn new(
        look_from: Point3,
        look_at: Point3,
        v_up: Vec3,
        theta: f64,
        aspect_ratio: f64,
    ) -> Camera {
        let h = (theta / 2.0).tan();
        let viewport_height = h * 2.0;
        let viewport_width = aspect_ratio * viewport_height;

        let w = (look_from - look_at).unit();
        let u = v_up.cross(w).unit();
        let v = w.cross(u);

        let origin = look_from;
        let horizontal = viewport_width * u;
        let vertical = viewport_height * v;
        let lower_left_corner = origin - horizontal / 2.0 - vertical / 2.0 - w;

        Camera {
            origin,
            lower_left_corner,
            horizontal,
            vertical,
        }
    }

    pub fn ray(&self, u: f64, v: f64) -> Ray {
        Ray::new(
            self.origin,
            self.lower_left_corner + u * self.horizontal + v * self.vertical - self.origin,
        )
    }
}
