use crate::color::{write_color, Color};
use crate::hittable::{Hittable, HittableList};
use crate::ray::Ray;
use crate::sphere::Sphere;
use crate::vec3::{Point, Vec3};
use std::io::Write;

use crate::camera::Camera;
use crate::material::{Lambertian, Metal};
use rand::{random, Rng};
use std::f64::INFINITY;
use std::rc::Rc;
use std::time::Instant;

mod camera;
mod color;
mod hittable;
mod material;
mod ray;
mod sphere;
mod vec3;

fn ray_color(ray: &Ray, world: &HittableList, depth: i32) -> Color {
    if depth <= 0 {
        return Color::zero();
    }

    if let Some(hit_record) = world.hit(ray, 0.001, INFINITY) {
        return if let Some((scattered_ray, attenuation)) =
            hit_record.material.scatter(ray, &hit_record)
        {
            ray_color(&scattered_ray, world, depth - 1) * attenuation
        } else {
            Color::zero()
        };
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
    let max_depth = 50;

    let max_color = 255;

    // World

    let blue_metal = Rc::new(Metal::new(Color::new(0.1, 0.5, 0.9), 0.0));

    let material_ground = Rc::new(Lambertian {
        albedo: Color::new(0.8, 0.8, 0.0),
    });
    let material_center = Rc::new(Lambertian {
        albedo: Color::new(0.7, 0.3, 0.3),
    });
    let material_left = Rc::new(Metal::new(Color::new(0.8, 0.8, 0.8), 0.3));
    let material_right = Rc::new(Metal::new(Color::new(0.8, 0.6, 0.2), 1.0));

    let world: HittableList = vec![
        Box::new(Sphere::new(
            Point::new(0.0, 0.0, -1.0),
            0.5,
            material_center.clone(),
        )),
        Box::new(Sphere::new(
            Point::new(-1.0, 0.0, -1.0),
            0.5,
            material_left.clone(),
        )),
        Box::new(Sphere::new(
            Point::new(1.0, 0.0, -1.0),
            0.5,
            material_right.clone(),
        )),
        Box::new(Sphere::new(
            Point::new(0.0, -100.5, -1.0),
            100.0,
            material_ground.clone(),
        )),
    ];

    let camera = Camera::new();

    // Render
    let start = Instant::now();
    println!("P3\n{} {}\n{}", image_width, image_height, max_color);

    let mut rng = rand::thread_rng();

    for y in (0..image_height).rev() {
        eprintln!("\rScanlines remaining: {}", y);
        std::io::stdout().flush().expect("failed to flush output");

        for x in 0..image_width {
            let pixel_color = (0..samples_per_pixel)
                .into_iter()
                .map(|_| {
                    let u = (x as f64 + rng.gen::<f64>()) / (image_width - 1) as f64;
                    let v = (y as f64 + rng.gen::<f64>()) / (image_height - 1) as f64;

                    let ray = camera.get_ray(u, v);

                    ray_color(&ray, &world, max_depth)
                })
                .sum();

            color::write_color(&mut std::io::stdout(), &pixel_color, samples_per_pixel)
                .expect("failed to write color");
        }
    }
    let duration = start.elapsed();
    eprintln!("Elapsed time: {:?}", duration)
}
