#![allow(dead_code)]

use std::f64::consts::PI;

use camera::Camera;
use hittables::{
    Box as HittableBox, BvhTree, ConstantMedium, Hittable, HittableVec, MovingSphere, RotateY,
    Sphere, Translate, XyRect, XzRect, YzRect,
};
use materials::{Dielectric, DiffuseLight, Lambertian, Metal};
use rand::Rng;
use ray::Ray;
#[cfg(feature = "parallel")]
use rayon::prelude::{IndexedParallelIterator, IntoParallelIterator, ParallelIterator};
use textures::{CheckerTexture, ImageTexture, NoiseTexture};
use vec3::{Color, Point3, Vec3};

mod camera;
mod hittables;
mod materials;
mod math;
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
    // let mut world = simple_light();
    // let samples_per_pixel = 400;
    // let aperture = 0.0;
    // let theta = PI * 20.0 / 180.0;
    // let look_from = Point3::new(26.0, 3.0, 6.0);
    // let look_at = Point3::new(0.0, 2.0, 0.0);
    // let background = Color::new(0.0, 0.0, 0.0);
    // let mut world = cornell_box();
    // let aspect_ratio = 1.0;
    // let image_width = 600;
    // let image_height = ((image_width as f64) / aspect_ratio) as usize;
    // let samples_per_pixel = 200;
    // let aperture = 0.0;
    // let theta = PI * 40.0 / 180.0;
    // let look_from = Point3::new(278.0, 278.0, -800.0);
    // let look_at = Point3::new(278.0, 278.0, 0.0);
    // let background = Color::new(0.0, 0.0, 0.0);
    // let world = cornell_smoke();
    // let aspect_ratio = 1.0;
    // let image_width = 600;
    // let image_height = ((image_width as f64) / aspect_ratio) as usize;
    // let samples_per_pixel = 200;
    // let aperture = 0.0;
    // let theta = PI * 40.0 / 180.0;
    // let look_from = Point3::new(278.0, 278.0, -800.0);
    // let look_at = Point3::new(278.0, 278.0, 0.0);
    // let background = Color::new(0.0, 0.0, 0.0);
    let world = final_scene();
    let aspect_ratio = 1.0;
    let image_width = 800;
    let image_height = ((image_width as f64) / aspect_ratio) as usize;
    let samples_per_pixel = 10000;
    let aperture = 0.0;
    let theta = PI * 40.0 / 180.0;
    let look_from = Point3::new(478.0, 278.0, -600.0);
    let look_at = Point3::new(278.0, 278.0, 0.0);
    let background = Color::new(0.0, 0.0, 0.0);

    let world = BvhTree::new(world, 0.0, 1.0);

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
    #[allow(clippy::too_many_arguments)]
    fn sample(
        image_width: usize,
        image_height: usize,
        camera: &Camera,
        background: Color,
        world: &impl Hittable,
        max_depth: i32,
        i: usize,
        j: usize,
    ) -> Color {
        let mut rng = rand::thread_rng();
        let z: f64 = rng.gen();
        let w: f64 = rng.gen();
        let u = (i as f64 + z) / ((image_width - 1) as f64);
        let v = (j as f64 + w) / ((image_height - 1) as f64);
        let ray = camera.ray(u, v);
        ray_color(&ray, background, world, max_depth)
    }

    for j in (0..image_height).rev() {
        eprint!("\rScanlines remaining: {j:>3}");
        cfg_if::cfg_if! {
            if #[cfg(feature = "parallel")] {
                let mut row = Vec::<Color>::with_capacity(image_width);
                (0..image_width)
                    .into_par_iter()
                    .map(|i| {
                        let c: Color = (0..samples_per_pixel)
                            .into_par_iter()
                            .map(|_| {
                                sample(
                                    image_width,
                                    image_height,
                                    camera,
                                    background,
                                    world,
                                    max_depth,
                                    i,
                                    j,
                                )
                            })
                            .sum();
                        c / samples_per_pixel as f64
                    })
                    .collect_into_vec(&mut row);
            } else {
                let row = (0..image_width)
                    .into_iter()
                    .map(|i| {
                        let c: Color = (0..samples_per_pixel)
                            .into_iter()
                            .map(|_| {
                                sample(
                                    image_width,
                                    image_height,
                                    camera,
                                    background,
                                    world,
                                    max_depth,
                                    i,
                                    j,
                                )
                            })
                            .sum();
                        c / samples_per_pixel as f64
                    })
                    .collect::<Vec<_>>();
            }
        }

        image.push(row);
    }
    image
}

fn ray_color(ray: &Ray, background: Color, world: &impl Hittable, depth: i32) -> Color {
    if depth <= 0 {
        return Color::default();
    }

    let Some(hit) = world.hit(ray, 0.001, f64::INFINITY) else {
        return background;
    };
    let emitted = hit.material.emitted(hit.u, hit.v, &hit.p);
    let Some(scatter) = hit.material.scatter(ray, &hit) else {
        return emitted;
    };

    emitted + scatter.attenuation * ray_color(&scatter.ray, background, world, depth - 1)
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

fn simple_light() -> HittableVec {
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

    let diff_light = DiffuseLight::new_with_color(Color::new(4.0, 4.0, 4.0));
    world.push(Box::new(XyRect::new(3.0, 5.0, 1.0, 3.0, -2.0, diff_light)));

    world
}

fn cornell_box() -> HittableVec {
    let mut world = HittableVec::new();

    let red = Lambertian::new_with_color(Color::new(0.65, 0.05, 0.05));
    let white = Lambertian::new_with_color(Color::new(0.73, 0.73, 0.73));
    let green = Lambertian::new_with_color(Color::new(0.12, 0.45, 0.15));
    let light = DiffuseLight::new_with_color(Color::new(15.0, 15.0, 15.0));

    world.push(Box::new(YzRect::new(0.0, 555.0, 0.0, 555.0, 555.0, green)));
    world.push(Box::new(YzRect::new(0.0, 555.0, 0.0, 555.0, 0.0, red)));
    world.push(Box::new(XzRect::new(
        213.0, 343.0, 227.0, 332.0, 554.0, light,
    )));
    world.push(Box::new(XzRect::new(
        0.0,
        555.0,
        0.0,
        555.0,
        0.0,
        white.clone(),
    )));
    world.push(Box::new(XzRect::new(
        0.0,
        555.0,
        0.0,
        555.0,
        555.0,
        white.clone(),
    )));
    world.push(Box::new(XyRect::new(
        0.0,
        555.0,
        0.0,
        555.0,
        555.0,
        white.clone(),
    )));

    world.push(Box::new(Translate::new(
        RotateY::new(
            HittableBox::new(
                Point3::new(0.0, 0.0, 0.0),
                Point3::new(165.0, 330.0, 165.0),
                white.clone(),
            ),
            PI * 15.0 / 180.0,
        ),
        Vec3::new(265.0, 0.0, 295.0),
    )));
    world.push(Box::new(Translate::new(
        RotateY::new(
            HittableBox::new(
                Point3::new(0.0, 0.0, 0.0),
                Point3::new(165.0, 165.0, 165.0),
                white,
            ),
            PI * -18.0 / 180.0,
        ),
        Vec3::new(130.0, 0.0, 65.0),
    )));

    world
}

fn cornell_smoke() -> HittableVec {
    let mut world = HittableVec::new();

    let red = Lambertian::new_with_color(Color::new(0.65, 0.05, 0.05));
    let white = Lambertian::new_with_color(Color::new(0.73, 0.73, 0.73));
    let green = Lambertian::new_with_color(Color::new(0.12, 0.45, 0.15));
    let light = DiffuseLight::new_with_color(Color::new(7.0, 7.0, 7.0));

    world.push(Box::new(YzRect::new(0.0, 555.0, 0.0, 555.0, 555.0, green)));
    world.push(Box::new(YzRect::new(0.0, 555.0, 0.0, 555.0, 0.0, red)));
    world.push(Box::new(XzRect::new(
        113.0, 443.0, 127.0, 432.0, 554.0, light,
    )));
    world.push(Box::new(XzRect::new(
        0.0,
        555.0,
        0.0,
        555.0,
        0.0,
        white.clone(),
    )));
    world.push(Box::new(XzRect::new(
        0.0,
        555.0,
        0.0,
        555.0,
        555.0,
        white.clone(),
    )));
    world.push(Box::new(XyRect::new(
        0.0,
        555.0,
        0.0,
        555.0,
        555.0,
        white.clone(),
    )));

    world.push(Box::new(ConstantMedium::new_with_color(
        Translate::new(
            RotateY::new(
                HittableBox::new(
                    Point3::new(0.0, 0.0, 0.0),
                    Point3::new(165.0, 330.0, 165.0),
                    white.clone(),
                ),
                PI * 15.0 / 180.0,
            ),
            Vec3::new(265.0, 0.0, 295.0),
        ),
        Color::new(0.0, 0.0, 0.0),
        0.01,
    )));
    world.push(Box::new(ConstantMedium::new_with_color(
        Translate::new(
            RotateY::new(
                HittableBox::new(
                    Point3::new(0.0, 0.0, 0.0),
                    Point3::new(165.0, 165.0, 165.0),
                    white,
                ),
                PI * -18.0 / 180.0,
            ),
            Vec3::new(130.0, 0.0, 65.0),
        ),
        Color::new(1.0, 1.0, 1.0),
        0.01,
    )));

    world
}

fn final_scene() -> HittableVec {
    let mut world = HittableVec::new();

    let light = DiffuseLight::new_with_color(Color::new(7.0, 7.0, 7.0));
    world.push(Box::new(XzRect::new(
        123.0, 423.0, 147.0, 412.0, 554.0, light,
    )));

    let boxes1 = (0..20)
        .flat_map(|i| {
            (0..20).map(move |j| {
                let ground = Lambertian::new_with_color(Color::new(0.48, 0.83, 0.53));
                let mut rng = rand::thread_rng();
                let w = 100.0;
                let x0 = -1000.0 + i as f64 * w;
                let z0 = -1000.0 + j as f64 * w;
                let y0 = 0.0;
                let x1 = x0 + w;
                let y1 = rng.gen_range(1.0..101.0);
                let z1 = z0 + w;
                Box::new(HittableBox::new(
                    Point3::new(x0, y0, z0),
                    Point3::new(x1, y1, z1),
                    ground,
                )) as Box<dyn Hittable>
            })
        })
        .collect::<Vec<_>>();
    world.push(Box::new(BvhTree::new(boxes1, 0.0, 1.0)));

    let moving_sphere_material = Lambertian::new_with_color(Color::new(0.7, 0.3, 0.1));
    world.push(Box::new(MovingSphere::new(
        Point3::new(400.0, 400.0, 200.0),
        Point3::new(430.0, 400.0, 200.0),
        0.0,
        1.0,
        50.0,
        moving_sphere_material,
    )));

    let dielectric_material = Dielectric::new(1.5);
    world.push(Box::new(Sphere::new(
        Point3::new(260.0, 150.0, 45.0),
        50.0,
        dielectric_material,
    )));

    let metal_material = Metal::new(Color::new(0.8, 0.8, 0.9), 1.0);
    world.push(Box::new(Sphere::new(
        Point3::new(0.0, 150.0, 145.0),
        50.0,
        metal_material,
    )));

    let boundary = Sphere::new(Point3::new(360.0, 150.0, 145.0), 70.0, Dielectric::new(1.5));
    world.push(Box::new(boundary.clone()));
    world.push(Box::new(ConstantMedium::new_with_color(
        boundary,
        Color::new(0.2, 0.4, 0.9),
        0.2,
    )));

    let boundary = Sphere::new(Point3::new(0.0, 0.0, 5.0), 5000.0, Dielectric::new(1.5));
    world.push(Box::new(ConstantMedium::new_with_color(
        boundary,
        Color::new(1.0, 1.0, 1.0),
        0.0001,
    )));

    let earth_material =
        Lambertian::new(ImageTexture::new_with_filename("./earthmap.jpg").unwrap());
    world.push(Box::new(Sphere::new(
        Point3::new(400.0, 200.0, 400.0),
        100.0,
        earth_material,
    )));

    let perlin_material = Lambertian::new(NoiseTexture::new_with_scale(0.1));
    world.push(Box::new(Sphere::new(
        Point3::new(220.0, 280.0, 300.0),
        80.0,
        perlin_material,
    )));

    let white = Lambertian::new_with_color(Color::new(0.73, 0.73, 0.73));
    let boxes2 = (0..1000)
        .map(|_| {
            Box::new(Sphere::new(
                Point3::random_range(0.0..165.0),
                10.0,
                white.clone(),
            )) as Box<dyn Hittable>
        })
        .collect::<Vec<_>>();
    world.push(Box::new(Translate::new(
        RotateY::new(BvhTree::new(boxes2, 0.0, 1.0), PI * 15.0 / 180.0),
        Vec3::new(-100.0, 270.0, 395.0),
    )));

    world
}
