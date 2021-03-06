use crate::color::Color;
use crate::hittable::{Hittable, HittableList};
use crate::ray::Ray;
use crate::sphere::Sphere;
use crate::vec3::Point;
use std::io::Write;

use crate::camera::Camera;
use rand::random;
use std::f64::INFINITY;
use std::time::Instant;

mod camera;
mod color;
mod hittable;
mod ray;
mod sphere;
mod vec3;

fn ray_color(ray: &Ray, world: &HittableList) -> Color {
    if let Some(hit_record) = world.hit(ray, 0.0, INFINITY) {
        return (hit_record.normal() + Color::new(1.0, 1.0, 1.0)) * 0.5;
    }

    let unit_direction = ray.direction.unit_vector();
    let t = 0.5 * (unit_direction.y + 1.0);

    Color::new(1.0, 1.0, 1.0) * (1.0 - t) + Color::new(0.5, 0.7, 1.0) * t
}

fn main() {
    // Image
    let aspect_ratio = 16.0 / 9.0;
    let image_width = 400;
    let image_height = (image_width as f64 / aspect_ratio) as i64;
    let samples_per_pixel = 100;

    let max_color = 255;

    // World
    let world: HittableList = vec![
        Box::new(Sphere::new(Point::new(0.0, 0.0, -1.0), 0.5)),
        Box::new(Sphere::new(Point::new(0.0, -100.5, -1.0), 100.0)),
    ];

    let camera = Camera::new();

    // Render
    let start = Instant::now();
    println!("P3\n{} {}\n{}", image_width, image_height, max_color);

    for y in (0..image_height).rev() {
        eprintln!("\rScanlines remaining: {}", y);
        std::io::stdout().flush().expect("failed to flush output");

        for x in 0..image_width {
            let pixel_color = (0..samples_per_pixel)
                .into_iter()
                .map(|_| {
                    let u = (x as f64 + random::<f64>()) / (image_width - 1) as f64;
                    let v = (y as f64 + random::<f64>()) / (image_height - 1) as f64;

                    let ray = camera.get_ray(u, v);

                    ray_color(&ray, &world)
                })
                .sum();

            color::write_color(&mut std::io::stdout(), &pixel_color, samples_per_pixel)
                .expect("failed to write color");
        }
    }
    let duration = start.elapsed();
    eprintln!("Elapsed time: {:?}", duration)
}
