#![allow(dead_code)]

use std::f64::consts::PI;

use camera::Camera;
use hittables::{BvhTree, Hittable, HittableVec, MovingSphere, Sphere};
use materials::{Dielectric, Lambertian, Metal};
use rand::Rng;
use ray::Ray;
use rayon::prelude::{IndexedParallelIterator, IntoParallelIterator, ParallelIterator};
use textures::{CheckerTexture, ImageTexture, NoiseTexture};
use vec3::{Color, Point3, Vec3};

mod camera;
mod hittables;
mod materials;
mod ray;
mod textures;
mod vec3;

fn main() {
    // Image
    let aspect_ratio = 16.0 / 9.0;
    let image_width = 400;
    let image_height = ((image_width as f64) / aspect_ratio) as usize;
    let samples_per_pixel = 100;
    let max_depth = 50;

    // World
    // let mut world = random_scene();
    // let aperture = 0.1;
    // let theta = PI * 20.0 / 180.0;
    // let look_from = Point3::new(13.0, 2.0, 3.0);
    // let look_at = Point3::new(0.0, 0.0, 0.0);
    // let background = Color::new(0.7, 0.8, 1.0);
    // let mut world = two_spheres();
    // let aperture = 0.0;
    // let theta = PI * 20.0 / 180.0;
    // let look_from = Point3::new(13.0, 2.0, 3.0);
    // let look_at = Point3::new(0.0, 0.0, 0.0);
    // let background = Color::new(0.7, 0.8, 1.0);
    // let mut world = two_perlin_spheres();
    // let aperture = 0.0;
    // let theta = PI * 20.0 / 180.0;
    // let look_from = Point3::new(13.0, 2.0, 3.0);
    // let look_at = Point3::new(0.0, 0.0, 0.0);
    // let background = Color::new(0.7, 0.8, 1.0);
    // let mut world = earth();
    // let aperture = 0.0;
    // let theta = PI * 20.0 / 180.0;
    // let look_from = Point3::new(13.0, 2.0, 3.0);
    // let look_at = Point3::new(0.0, 0.0, 0.0);
    // let background = Color::new(0.7, 0.8, 1.0);
    let mut world = earth();
    let aperture = 0.0;
    let theta = PI * 20.0 / 180.0;
    let look_from = Point3::new(13.0, 2.0, 3.0);
    let look_at = Point3::new(0.0, 0.0, 0.0);
    let background = Color::new(0.0, 0.0, 0.0);

    let world = BvhTree::new(&mut world, 0.0, 1.0);

    // Camera
    let v_up = Vec3::new(0.0, 1.0, 0.0);
    let camera = Camera::new(
        look_from,
        look_at,
        v_up,
        theta,
        aspect_ratio,
        aperture,
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
        background,
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
    background: Color,
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
                        ray_color(ray, background, world, max_depth)
                    })
                    .sum();
                c / samples_per_pixel as f64
            })
            .collect_into_vec(&mut row);
        image.push(row);
    }
    image
}

fn ray_color(ray: Ray, background: Color, world: &impl Hittable, depth: i32) -> Color {
    if depth <= 0 {
        return Color::default();
    }

    let Some(hit) = world.hit(ray, 0.001, f64::INFINITY) else {
        return background;
    };
    let emitted = hit.material.emitted(hit.u, hit.v, &hit.p);
    let Some(scatter) = hit.material.scatter(&ray, &hit) else {
        return emitted;
    };

    emitted + scatter.attenuation * ray_color(scatter.ray, background, world, depth - 1)
}

fn random_scene() -> HittableVec {
    let mut world = HittableVec::new();

    let checker =
        CheckerTexture::new_with_colors(Color::new(0.2, 0.3, 0.1), Color::new(0.9, 0.9, 0.9));
    let material_ground = Lambertian::new(checker);
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
                        Lambertian::new_with_color(albedo),
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
    let material2 = Lambertian::new_with_color(Color::new(0.4, 0.2, 0.1));
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

fn two_spheres() -> HittableVec {
    let mut world = HittableVec::new();

    let checker =
        CheckerTexture::new_with_colors(Color::new(0.2, 0.3, 0.1), Color::new(0.9, 0.9, 0.9));
    world.push(Box::new(Sphere::new(
        Point3::new(0.0, -10.0, 0.0),
        10.0,
        Lambertian::new(checker.clone()),
    )));
    world.push(Box::new(Sphere::new(
        Point3::new(0.0, 10.0, 0.0),
        10.0,
        Lambertian::new(checker),
    )));

    world
}

fn two_perlin_spheres() -> HittableVec {
    let mut world = HittableVec::new();

    let perlin = NoiseTexture::new_with_scale(4.0);
    world.push(Box::new(Sphere::new(
        Point3::new(0.0, -1000.0, 0.0),
        1000.0,
        Lambertian::new(perlin.clone()),
    )));
    world.push(Box::new(Sphere::new(
        Point3::new(0.0, 2.0, 0.0),
        2.0,
        Lambertian::new(perlin),
    )));

    world
}

fn earth() -> HittableVec {
    let mut world = HittableVec::new();

    let earth_texture = ImageTexture::new_with_filename("./earthmap.jpg").unwrap();
    let earth_surface = Lambertian::new(earth_texture);
    let globe = Sphere::new(Point3::default(), 2.0, earth_surface);

    world.push(Box::new(globe));

    world
}
