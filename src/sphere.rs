use crate::hittable::{HitRecord, Hittable};
use crate::material::Material;
use crate::ray::Ray;
use crate::vec3::{Point, Vec3};
use std::rc::Rc;

pub struct Sphere {
    center: Point,
    radius: f64,
    material: Rc<dyn Material>,
}

impl Sphere {
    pub fn new(center: Vec3, radius: f64, material: Rc<dyn Material>) -> Self {
        Sphere {
            center,
            radius,
            material,
        }
    }
}

impl Hittable for Sphere {
    fn hit(&self, ray: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord> {
        let oc = ray.origin - self.center;

        let a = ray.direction.length_squared();
        let half_b = oc.dot(&ray.direction);
        let c = oc.length_squared() - self.radius * self.radius;

        let discriminant = half_b * half_b - a * c;
        let sqrtd = discriminant.sqrt();

        let root = (-half_b - sqrtd) / a;

        let root_fits = |root| root >= t_min && root <= t_max;

        // Find the nearest root that lies in the acceptable range
        if !root_fits(root) {
            let root = (-half_b + sqrtd) / a;
            if !root_fits(root) {
                return None;
            }
        }

        let hit_point = ray.at(root);
        let outward_normal = (hit_point - self.center) / self.radius;

        Some(HitRecord::new(
            hit_point,
            root,
            ray,
            &outward_normal,
            self.material.clone(),
        ))
    }
}
