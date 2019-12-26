use crate::hitable::{HitRecord, Hitable, AABB};
use crate::material::Material;
use crate::math::*;
use crate::ray::Ray;

pub struct XYRect {
    pub x0: f32,
    pub x1: f32,
    pub y0: f32,
    pub y1: f32,
    pub k: f32,
    pub material: Box<dyn Material>,
}

impl Hitable for XYRect {
    fn hit(&self, ray: &Ray, t_min: f32, t_max: f32) -> Option<HitRecord> {
        let t = (self.k - ray.origin.z) / ray.direction.z;
        if t < t_min || t > t_max {
            return None;
        }
        let x = ray.origin.x + t * ray.direction.x;
        let y = ray.origin.y + t * ray.direction.y;
        if x < self.x0 || x > self.x1 || y < self.y0 || y > self.y1 {
            return None;
        }
        Some(HitRecord {
            t,
            p: ray.point_at_parameter(t),
            u: (x - self.x0) / (self.x1 - self.x0),
            v: (y - self.y0) / (self.y1 - self.y0),
            normal: vec3(0.0, 0.0, 1.0),
            material: Some(&*self.material),
        })
    }

    fn bounding_box(&self, _t0: f32, _t1: f32) -> Option<AABB> {
        Some(AABB {
            min: vec3(self.x0, self.y0, self.k - 0.0001),
            max: vec3(self.x1, self.y1, self.k + 0.0001),
        })
    }
}
