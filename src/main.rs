use std::f64::consts::PI;

use camera::Camera;
use hittables::{BvhTree, Hittable, HittableVec, MovingSphere, Sphere};
use materials::{Dielectric, Lambertian, Metal};
use rand::Rng;
use ray::Ray;
use rayon::prelude::{IndexedParallelIterator, IntoParallelIterator, ParallelIterator};
use vec3::{Color, Point3, Vec3};

mod camera;
mod hittables;
mod materials;
mod ray;
mod vec3;

fn main() {
    // Image
    let aspect_ratio = 16.0 / 9.0;
    let image_width = 400;
    let image_height = ((image_width as f64) / aspect_ratio) as usize;
    let samples_per_pixel = 100;
    let max_depth = 50;

    // World
    let mut world = scene();
    let world = BvhTree::new(&mut world, 0.0, 1.0);

    // Camera
    let look_from = Point3::new(13.0, 2.0, 3.0);
    let look_at = Point3::new(0.0, 0.0, 0.0);
    let v_up = Vec3::new(0.0, 1.0, 0.0);
    let camera = Camera::new(
        look_from,
        look_at,
        v_up,
        PI * 20.0 / 180.0,
        aspect_ratio,
        0.1,
        10.0,
        0.0,
        1.0,
    );

    // Render
    let image = render(
        image_width,
        image_height,
        &world,
        &camera,
        samples_per_pixel,
        max_depth,
    );

    // Output
    println!("P3\n{image_width} {image_height}\n255");
    for row in image {
        for c in row {
            _ = c.write(&mut std::io::stdout());
        }
    }
    eprint!("\nDone.\n");
}

fn render(
    image_width: usize,
    image_height: usize,
    world: &impl Hittable,
    camera: &Camera,
    samples_per_pixel: i32,
    max_depth: i32,
) -> Vec<Vec<Color>> {
    let mut image = Vec::<Vec<Color>>::with_capacity(image_height);
    for j in (0..image_height).rev() {
        eprint!("\rScanlines remaining: {j:>3}");
        let mut row = Vec::<Color>::with_capacity(image_width);
        (0..image_width)
            .into_par_iter()
            .map(|i| {
                let c: Color = (0..samples_per_pixel)
                    .into_par_iter()
                    .map(|_| {
                        let mut rng = rand::thread_rng();
                        let z: f64 = rng.gen();
                        let w: f64 = rng.gen();
                        let u = (i as f64 + z) / ((image_width - 1) as f64);
                        let v = (j as f64 + w) / ((image_height - 1) as f64);
                        let ray = camera.ray(u, v);
                        ray_color(ray, world, max_depth)
                    })
                    .sum();
                c / samples_per_pixel as f64
            })
            .collect_into_vec(&mut row);
        image.push(row);
    }
    image
}

fn ray_color(ray: Ray, world: &impl Hittable, depth: i32) -> Color {
    if depth <= 0 {
        return Color::default();
    }
    if let Some(hit) = world.hit(ray, 0.001, f64::INFINITY) {
        if let Some(scatter) = hit.material.scatter(&ray, &hit) {
            return scatter.attenuation * ray_color(scatter.ray, world, depth - 1);
        } else {
            return Color::default();
        }
    }
    let t = 0.5 * (ray.direction.unit().y() + 1.0);
    (1.0 - t) * Color::new(1.0, 1.0, 1.0) + t * Color::new(0.5, 0.7, 1.0)
}

fn scene() -> HittableVec {
    let mut world = HittableVec::new();

    let material_ground = Lambertian::new(Color::new(0.5, 0.5, 0.5));
    world.push(Box::new(Sphere::new(
        Point3::new(0.0, -1000.0, 0.0),
        1000.0,
        material_ground,
    )));

    let mut rng = rand::thread_rng();

    for a in -11..11 {
        for b in -11..11 {
            let choose_mat: f64 = rng.gen();
            let center = Point3::new(
                a as f64 + 0.9 * rng.gen::<f64>(),
                0.2,
                b as f64 + 0.9 * rng.gen::<f64>(),
            );

            if (center - Point3::new(4.0, 0.2, 0.0)).length() > 0.9 {
                let sphere: Box<dyn Hittable> = if choose_mat < 0.8 {
                    // diffuse
                    let albedo = Color::random() * Color::random();
                    let center2 = center + Vec3::new(0.0, rng.gen_range(0.0..0.5), 0.0);
                    Box::new(MovingSphere::new(
                        center,
                        center2,
                        0.0,
                        1.0,
                        0.2,
                        Lambertian::new(albedo),
                    ))
                } else if choose_mat < 0.95 {
                    // metal
                    let albedo = Color::random() * 0.5 + Color::new(0.5, 0.5, 0.5);
                    let fuzz = rng.gen::<f64>() * 0.5;
                    Box::new(Sphere::new(center, 0.2, Metal::new(albedo, fuzz)))
                } else {
                    // glass
                    Box::new(Sphere::new(center, 0.2, Dielectric::new(1.5)))
                };
                world.push(sphere);
            }
        }
    }

    let material1 = Dielectric::new(1.5);
    let material2 = Lambertian::new(Color::new(0.4, 0.2, 0.1));
    let material3 = Metal::new(Color::new(0.7, 0.6, 0.5), 0.0);
    world.push(Box::new(Sphere::new(
        Point3::new(0.0, 1.0, 0.0),
        1.0,
        material1,
    )));
    world.push(Box::new(Sphere::new(
        Point3::new(-4.0, 1.0, 0.0),
        1.0,
        material2,
    )));
    world.push(Box::new(Sphere::new(
        Point3::new(4.0, 1.0, 0.0),
        1.0,
        material3,
    )));

    world
}
