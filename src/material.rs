use crate::color::Color;
use crate::hittable::HitRecord;
use crate::ray::Ray;
use crate::vec3::Vec3;
use rand::random;

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

pub struct Dielectric {
    pub refraction_index: f64,
}

impl Dielectric {
    fn reflectance(cosine: f64, ref_idx: f64) -> f64 {
        // Use Schlick's approximation for reflectance.

        let r0 = (1.0 - ref_idx) / (1.0 + ref_idx);
        let r0 = r0 * r0;

        r0 + (1.0 - r0) * (1.0 - cosine).powi(5)
    }
}

impl Material for Dielectric {
    fn scatter(&self, ray: &Ray, hit_record: &HitRecord) -> Option<(Ray, Color)> {
        let attenuation = Color::new(1.0, 1.0, 1.0);
        let refraction_ratio = match hit_record.front_face {
            true => 1.0 / self.refraction_index,
            false => self.refraction_index,
        };

        let cos_theta = f64::min(hit_record.normal().dot(&-ray.direction.unit_vector()), 1.0);
        let sin_theta = (1.0 - cos_theta * cos_theta).sqrt();

        let cannot_refract = refraction_ratio * sin_theta > 1.0;
        let must_reflect = Self::reflectance(cos_theta, refraction_ratio) > random();

        let new_direction = if cannot_refract || must_reflect {
            // Can't refract
            ray.direction.unit_vector().reflect(&hit_record.normal())
        } else {
            // Can refract
            ray.direction
                .unit_vector()
                .refract(&hit_record.normal(), refraction_ratio)
        };

        let refracted = Ray::new(hit_record.point, new_direction);

        Some((refracted, attenuation))
    }
}
