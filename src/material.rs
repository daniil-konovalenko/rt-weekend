use crate::color::Color;
use crate::hittable::HitRecord;
use crate::ray::Ray;
use crate::vec3::Vec3;

pub trait Material {
    fn scatter(&self, ray: &Ray, hit_record: &HitRecord) -> Option<(Ray, Color)>;
}

#[derive(Copy, Clone)]
pub struct Lambertian {
    pub albedo: Color,
}

impl Material for Lambertian {
    fn scatter(&self, ray: &Ray, hit_record: &HitRecord) -> Option<(Ray, Color)> {
        let scatter_direction = hit_record.normal() + Vec3::random_unit();

        // Catch degenerate scatter direction
        let scatter_direction = if scatter_direction.is_near_zero() {
            hit_record.normal()
        } else {
            scatter_direction
        };

        return Some((Ray::new(hit_record.point, scatter_direction), self.albedo));
    }
}
