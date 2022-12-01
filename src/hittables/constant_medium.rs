use std::f64;

use rand::Rng;

use crate::{
    materials::{Material, Scatter},
    ray::Ray,
    textures::{SolidColor, Texture},
    vec3::{Color, Vec3},
};

use super::{Aabb, Hit, Hittable};

#[derive(Debug, Clone)]
pub struct ConstantMedium<H: Hittable, T: Texture> {
    boundary: H,
    phase_function: Isotropic<T>,
    neg_inv_density: f64,
}

impl<H: Hittable> ConstantMedium<H, SolidColor> {
    pub fn new_with_color(
        boundary: H,
        color: Color,
        density: f64,
    ) -> ConstantMedium<H, SolidColor> {
        ConstantMedium {
            boundary,
            phase_function: Isotropic::new_with_color(color),
            neg_inv_density: -1.0 / density,
        }
    }
}

impl<H: Hittable, T: Texture> ConstantMedium<H, T> {
    pub fn new(boundary: H, texture: T, density: f64) -> ConstantMedium<H, T> {
        ConstantMedium {
            boundary,
            phase_function: Isotropic::new(texture),
            neg_inv_density: -1.0 / density,
        }
    }
}

impl<H: Hittable, T: Texture> Hittable for ConstantMedium<H, T> {
    fn hit(&self, ray: &Ray, t_min: f64, t_max: f64) -> Option<Hit> {
        // 入射
        let Some(mut hit_in) = self.boundary.hit(ray, -f64::INFINITY, f64::INFINITY) else {
            return None;
        };
        // 射出
        let Some(mut hit_out) = self.boundary.hit(ray, hit_in.t + 0.0001, f64::INFINITY) else {
            return None;
        };

        if hit_in.t < t_min {
            hit_in.t = t_min;
        }
        if hit_out.t > t_max {
            hit_out.t = t_max;
        }

        if hit_in.t >= hit_out.t {
            // t_minからt_outの間には媒質がない
            return None;
        }

        if hit_in.t < 0.0 {
            // ?
            hit_in.t = 0.0;
        }

        let ray_length = ray.direction.length();
        let distance_inside_boundary = (hit_out.t - hit_in.t) * ray_length;
        let mut rng = rand::thread_rng();
        let hit_distance = self.neg_inv_density * rng.gen_range::<f64, _>(0.0..1.0).ln();

        if hit_distance > distance_inside_boundary {
            // 散乱せずに脱出した
            return None;
        }

        let t = hit_in.t + hit_distance / ray_length;
        let p = ray.at(t);
        Some(Hit::new(
            p,
            Vec3::new(1.0, 0.0, 0.0),
            t,
            0.0,
            0.0,
            true,
            &self.phase_function,
        ))
    }

    fn bounding_box(&self, time0: f64, time1: f64) -> Option<Aabb> {
        self.boundary.bounding_box(time0, time1)
    }
}

#[derive(Debug, Clone)]
struct Isotropic<T: Texture> {
    albedo: T,
}

impl Isotropic<SolidColor> {
    pub fn new_with_color(color: Color) -> Isotropic<SolidColor> {
        Isotropic {
            albedo: SolidColor::new(color),
        }
    }
}

impl<T: Texture> Isotropic<T> {
    pub fn new(albedo: T) -> Isotropic<T> {
        Isotropic { albedo }
    }
}

impl<T: Texture> Material for Isotropic<T> {
    fn scatter(&self, ray: &Ray, hit: &Hit) -> Option<Scatter> {
        let ray = Ray::new(hit.p, Vec3::random_in_unit_sphere(), ray.time);
        let attenuation = self.albedo.value(hit.u, hit.v, &hit.p);
        Some(Scatter::new(attenuation, ray))
    }
}
