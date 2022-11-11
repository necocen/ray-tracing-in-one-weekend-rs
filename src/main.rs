use camera::Camera;
use hittable::{Hittable, HittableList};
use lambertian::Lambertian;
use metal::Metal;
use rand::Rng;
use ray::Ray;
use sphere::Sphere;
use vec3::{Color, Point3};

mod camera;
mod hittable;
mod lambertian;
mod material;
mod metal;
mod ray;
mod sphere;
mod vec3;

fn main() {
    // Image
    let aspect_ratio = 16.0 / 9.0;
    let image_width = 400;
    let image_height = ((image_width as f64) / aspect_ratio) as i32;
    let samples_per_pixel = 100;
    let max_depth = 50;

    // World
    let material_ground = Lambertian::new(Color::new(0.8, 0.8, 0.0));
    let material_center = Lambertian::new(Color::new(0.7, 0.3, 0.3));
    let material_left = Metal::new(Color::new(0.8, 0.8, 0.8), 0.3);
    let material_right = Metal::new(Color::new(0.8, 0.6, 0.2), 1.0);

    let world: HittableList = vec![
        Box::new(Sphere::new(
            Point3::new(0.0, -100.5, -1.0),
            100.0,
            &material_ground,
        )),
        Box::new(Sphere::new(
            Point3::new(0.0, 0.0, -1.0),
            0.5,
            &material_center,
        )),
        Box::new(Sphere::new(
            Point3::new(-1.0, 0.0, -1.0),
            0.5,
            &material_left,
        )),
        Box::new(Sphere::new(
            Point3::new(1.0, 0.0, -1.0),
            0.5,
            &material_right,
        )),
    ];

    // Camera
    let camera = Camera::new();

    // Render

    println!("P3\n{image_width} {image_height}\n255");

    let mut rng = rand::thread_rng();
    for j in (0..image_height).rev() {
        eprint!("\rScanlines remaining: {j}");
        for i in 0..image_width {
            let mut c = (0..samples_per_pixel)
                .map(|_| {
                    let z: f64 = rng.gen();
                    let w: f64 = rng.gen();
                    let u = (i as f64 + z) / ((image_width - 1) as f64);
                    let v = (j as f64 + w) / ((image_height - 1) as f64);
                    let ray = camera.ray(u, v);
                    ray_color(ray, &world, max_depth)
                })
                .fold(Color::default(), |c, d| c + d);
            c /= samples_per_pixel as f64;
            _ = c.write(&mut std::io::stdout());
        }
    }
    eprint!("\nDone.\n");
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
