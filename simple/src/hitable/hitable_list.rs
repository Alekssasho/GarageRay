use crate::hitable::{surrounding_box, HitRecord, Hitable, AABB};
use crate::math::*;
use crate::random::*;
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
        let mut result = self.first().unwrap().bounding_box(t0, t1)?;
        for hitable in &self[1..] {
            let aabb = hitable.bounding_box(t0, t1)?;
            result = surrounding_box(aabb, result);
        }
        Some(result)
    }

    fn pdf_value(&self, o: &Vec3, v: &Vec3) -> f32 {
        let weight = 1.0 / self.len() as f32;
        self.into_iter()
            .map(|hitable| weight * hitable.pdf_value(o, v))
            .sum()
    }

    fn random(&self, o: &Vec3) -> Vec3 {
        let index = (random_float() * self.len() as f32) as usize;
        self[index].random(o)
    }
}
