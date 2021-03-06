use crate::color::Color;
use crate::hittable::{Hittable, HittableList};
use crate::ray::Ray;
use crate::sphere::Sphere;
use crate::vec3::{Point, Vec3};
use std::io::Write;

use std::f64::INFINITY;

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

    let max_color = 255;

    // World
    let world: HittableList = vec![
        Box::new(Sphere::new(Point::new(0.0, 0.0, -1.0), 0.5)),
        Box::new(Sphere::new(Point::new(0.0, -100.5, -1.0), 100.0)),
    ];

    // Camera
    let viewport_height = 2.0;
    let viewport_width = aspect_ratio * viewport_height;
    let focal_length = 1.0;

    let origin = Point::zero();
    let horizontal = Vec3::new(viewport_width, 0.0, 0.0);
    let vertical = Vec3::new(0.0, viewport_height, 0.0);
    let lower_left_corner =
        origin - horizontal / 2.0 - vertical / 2.0 - Vec3::new(0.0, 0.0, focal_length);

    // Render

    println!("P3\n{} {}\n{}", image_width, image_height, max_color);

    for y in (0..image_height).rev() {
        eprintln!("\rScanlines remaining: {}", y);
        std::io::stdout().flush().expect("failed to flush output");

        for x in 0..image_width {
            let u = x as f64 / (image_width - 1) as f64;
            let v = y as f64 / (image_height - 1) as f64;

            let ray = Ray::new(
                origin,
                lower_left_corner + horizontal * u + vertical * v - origin,
            );

            let pixel_color = ray_color(&ray, &world);

            color::write_color(&mut std::io::stdout(), &pixel_color)
                .expect("failed to write color");
        }
    }
}
