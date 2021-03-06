use crate::ray::Ray;
use crate::vec3::{Point, Vec3};

pub struct HitRecord {
    pub point: Point,
    normal: Vec3,
    pub t: f64,
    pub front_face: bool,
}

impl HitRecord {
    pub fn normal(&self) -> Vec3 {
        self.normal
    }
}

impl HitRecord {
    pub fn new(point: Point, t: f64, ray: &Ray, outward_normal: &Vec3) -> Self {
        let front_face = ray.direction.dot(outward_normal) < 0.0;
        let normal = if front_face {
            *outward_normal
        } else {
            -*outward_normal
        };

        Self {
            point,
            normal,
            t,
            front_face,
        }
    }
}

pub trait Hittable {
    fn hit(&self, ray: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord>;
}

pub type HittableList = Vec<Box<dyn Hittable>>;

impl Hittable for HittableList {
    fn hit(&self, ray: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord> {
        let mut closest = t_max;
        let mut last_hit = None;

        for object in self {
            if let Some(hit_record) = object.hit(ray, t_min, closest) {
                closest = hit_record.t;
                last_hit = Some(hit_record);
            }
        }

        last_hit
    }
}
