use rand::Rng;

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
    u: Vec3,
    v: Vec3,
    #[allow(dead_code)]
    w: Vec3,
    lens_radius: f64,
    time0: f64,
    time1: f64,
}

impl Camera {
    /// - `look_from` - point that camera is looking from
    /// - `look_at` - point that camera is looking at
    /// - `v_up` - 'up' direction of camera
    /// - `theta` - vertical field-of-view in radians
    /// - `aspect_Ratio` - aspect ratio of viewport
    /// - `aperture` - diameter of aperture
    /// - `focus_dist` - ???
    /// - `time0` - shutter open time
    /// - `time1` - shutter close time
    #[allow(clippy::too_many_arguments)]
    pub fn new(
        look_from: Point3,
        look_at: Point3,
        v_up: Vec3,
        theta: f64,
        aspect_ratio: f64,
        aperture: f64,
        focus_dist: f64,
        time0: f64,
        time1: f64,
    ) -> Camera {
        let h = (theta / 2.0).tan();
        let viewport_height = h * 2.0;
        let viewport_width = aspect_ratio * viewport_height;

        let w = (look_from - look_at).unit();
        let u = v_up.cross(w).unit();
        let v = w.cross(u);

        let origin = look_from;
        let horizontal = focus_dist * viewport_width * u;
        let vertical = focus_dist * viewport_height * v;
        let lower_left_corner = origin - horizontal / 2.0 - vertical / 2.0 - focus_dist * w;

        Camera {
            origin,
            lower_left_corner,
            horizontal,
            vertical,
            u,
            v,
            w,
            lens_radius: aperture / 2.0,
            time0,
            time1,
        }
    }

    pub fn ray(&self, u: f64, v: f64) -> Ray {
        let rd = self.lens_radius * Vec3::random_in_unit_disk();
        let offset = self.u * rd.x() + self.v * rd.y();
        let mut rng = rand::thread_rng();
        let time = if self.time0 == self.time1 {
            self.time0
        } else {
            rng.gen_range(self.time0..self.time1)
        };
        Ray::new(
            self.origin + offset,
            self.lower_left_corner + u * self.horizontal + v * self.vertical - self.origin - offset,
            time,
        )
    }
}
