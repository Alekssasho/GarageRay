use crate::hitable::{surrounding_box, HitRecord, Hitable, AABB};
use crate::ray::Ray;

impl Hitable for Vec<Box<dyn Hitable>> {
    fn hit(&self, ray: &Ray, t_min: f32, t_max: f32) -> Option<HitRecord> {
        let mut hit: Option<HitRecord> = None;
        let mut closest_so_far = t_max;
        for hitable in self {
            if let Some(temp_rec) = hitable.hit(ray, t_min, closest_so_far) {
                closest_so_far = temp_rec.t;
                hit = Some(temp_rec);
            }
        }

        hit
    }

    fn bounding_box(&self, t0: f32, t1: f32) -> Option<AABB> {
        if self.len() < 1 {
            return None;
        }
        let mut result = if let Some(aabb) = self.first().unwrap().bounding_box(t0, t1) {
            aabb
        } else {
            return None;
        };
        for hitable in &self[1..] {
            if let Some(aabb) = hitable.bounding_box(t0, t1) {
                result = surrounding_box(aabb, result);
            } else {
                return None;
            }
        }
        Some(result)
    }
}
