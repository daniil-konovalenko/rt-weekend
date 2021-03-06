use crate::color::Color;
use crate::ray::Ray;
use crate::vec3::{Point, Vec3};
use std::io::Write;

mod color;
mod ray;
mod vec3;

fn ray_color(ray: &Ray) -> Color {
    let t = hits_sphere(&Point::new(0.0, 0.0, -1.0), 0.5, *ray);

    if t > 0.0 {
        let normal = (ray.at(t) - Vec3::new(0.0, 0.0, -1.0)).unit_vector();
        return Color::new(normal.x + 1.0, normal.y + 1.0, normal.z + 1.0) * 0.5;
    }

    let unit_direction = ray.direction.unit_vector();
    let t = 0.5 * (unit_direction.y + 1.0);

    Color::new(1.0, 1.0, 1.0) * (1.0 - t) + Color::new(0.5, 0.7, 1.0) * t
}

fn hits_sphere(center: &Point, radius: f64, ray: Ray) -> f64 {
    let oc = ray.origin - *center;

    let a = ray.direction.length_squared();
    let half_b = oc.dot(&ray.direction);
    let c = oc.length_squared() - radius * radius;

    let discriminant = half_b * half_b - a * c;

    if discriminant < 0.0 {
        -1.0
    } else {
        (-half_b - discriminant.sqrt()) / a
    }
}

fn main() {
    // Image
    let aspect_ratio = 16.0 / 9.0;
    let image_width = 400;
    let image_height = (image_width as f64 / aspect_ratio) as i64;

    let max_color = 255;

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

            let pixel_color = ray_color(&ray);

            color::write_color(&mut std::io::stdout(), &pixel_color)
                .expect("failed to write color");
        }
    }
}
