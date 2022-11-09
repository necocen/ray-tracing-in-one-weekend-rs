use camera::Camera;
use hittable::{HitRecord, Hittable, HittableList};
use rand::Rng;
use ray::Ray;
use sphere::Sphere;
use vec3::{Color, Point3, Vec3};

mod camera;
mod hittable;
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
    let world: HittableList = vec![
        Box::new(Sphere::new(Point3::new(0.0, 0.0, -1.0), 0.5)),
        Box::new(Sphere::new(Point3::new(0.0, -100.5, -1.0), 100.0)),
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
    if let Some(HitRecord { normal, p, .. }) = world.hit(ray, 0.0, f64::INFINITY) {
        let target = p + normal + Vec3::random_in_unit_sphere();
        return 0.5 * ray_color(Ray::new(p, target - p), world, depth - 1);
    }
    let t = 0.5 * (ray.direction.unit().y() + 1.0);
    (1.0 - t) * Color::new(1.0, 1.0, 1.0) + t * Color::new(0.5, 0.7, 1.0)
}
