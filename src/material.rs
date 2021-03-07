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

pub struct Metal {
    pub albedo: Color,
    fuzz: f64,
}

impl Metal {
    pub fn new(albedo: Color, fuzz: f64) -> Self {
        let fuzz = if fuzz < 1.0 { fuzz } else { 1.0 };
        Metal { albedo, fuzz }
    }
}

impl Material for Metal {
    fn scatter(&self, ray: &Ray, hit_record: &HitRecord) -> Option<(Ray, Color)> {
        let reflect_direction = ray.direction.reflect(&hit_record.normal().unit_vector());
        let reflected = Ray::new(
            hit_record.point,
            reflect_direction + Vec3::random_in_unit_sphere() * self.fuzz,
        );

        if reflected.direction.dot(&hit_record.normal()) > 0.0 {
            Some((reflected, self.albedo))
        } else {
            None
        }
    }
}
