use vec3::Color;

mod ray;
mod vec3;

fn main() {
    let image_width = 256;
    let image_height = 256;

    println!("P3\n{image_width} {image_height}\n255");

    for j in (0..image_height).rev() {
        eprint!("\rScanlines remaining: {j}");
        for i in 0..image_width {
            let c = Color::new(
                (i as f64) / ((image_width - 1) as f64),
                (j as f64) / ((image_width - 1) as f64),
                0.25f64,
            );
            _ = c.write(&mut std::io::stdout());
        }
    }
    eprint!("\nDone.\n");
}
