use hittable::{HitRecord, Hittable, HittableList};
use ray::Ray;
use sphere::Sphere;
use vec3::{Color, Point3, Vec3};

mod hittable;
mod ray;
mod sphere;
mod vec3;

fn main() {
    // Image
    let aspect_ratio = 16.0 / 9.0;
    let image_width = 400;
    let image_height = ((image_width as f64) / aspect_ratio) as i32;

    // World
    let world: HittableList = vec![
        Box::new(Sphere::new(Point3::new(0.0, 0.0, -1.0), 0.5)),
        Box::new(Sphere::new(Point3::new(0.0, -100.5, -1.0), 100.0)),
    ];

    // Camera
    let viewport_height = 2.0;
    let viewport_width = aspect_ratio * viewport_height;
    let focal_length = 1.0;

    let origin = Point3::new(0.0, 0.0, 0.0);
    let horizontal = Vec3::new(viewport_width, 0.0, 0.0);
    let vertical = Vec3::new(0.0, viewport_height, 0.0);
    let lower_left_corner =
        origin - horizontal / 2.0 - vertical / 2.0 - Vec3::new(0.0, 0.0, focal_length);

    // Render

    println!("P3\n{image_width} {image_height}\n255");

    for j in (0..image_height).rev() {
        eprint!("\rScanlines remaining: {j}");
        for i in 0..image_width {
            let u = (i as f64) / ((image_width - 1) as f64);
            let v = (j as f64) / ((image_height - 1) as f64);
            let ray = Ray::new(
                origin,
                lower_left_corner + u * horizontal + v * vertical - origin,
            );
            let c = ray_color(ray, &world);
            _ = c.write(&mut std::io::stdout());
        }
    }
    eprint!("\nDone.\n");
}

fn ray_color(ray: Ray, world: &impl Hittable) -> Color {
    if let Some(HitRecord { normal, .. }) = world.hit(ray, 0.0, f64::INFINITY) {
        return 0.5 * Color::new(normal.x() + 1.0, normal.y() + 1.0, normal.z() + 1.0);
    }
    let t = 0.5 * (ray.direction.unit().y() + 1.0);
    (1.0 - t) * Color::new(1.0, 1.0, 1.0) + t * Color::new(0.5, 0.7, 1.0)
}
