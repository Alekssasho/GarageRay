use crate::hitable::{HitRecord, Hitable, AABB};
use crate::ray::Ray;

pub struct FlipNormals(pub Box<dyn Hitable>);

impl Hitable for FlipNormals {
    fn hit(&self, ray: &Ray, t_min: f32, t_max: f32) -> Option<HitRecord> {
        match self.0.hit(ray, t_min, t_max) {
            Some(rec) => Some(HitRecord {
                normal: -rec.normal,
                ..rec
            }),
            None => None,
        }
    }
    fn bounding_box(&self, t0: f32, t1: f32) -> Option<AABB> {
        self.0.bounding_box(t0, t1)
    }
}
