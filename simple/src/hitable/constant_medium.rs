use crate::hitable::{HitRecord, Hitable, AABB};
use crate::material::*;
use crate::math::*;
use crate::random::*;
use crate::ray::Ray;
use crate::texture::*;

pub struct ConstantMedium {
    boundary: Box<dyn Hitable>,
    density: f32,
    phase_function: Box<dyn Material>,
}

impl Hitable for ConstantMedium {
    fn hit(&self, ray: &Ray, t_min: f32, t_max: f32) -> Option<HitRecord> {
        //let mut db = random_float() < 0.00001;
        if let Some(mut rec1) = self.boundary.hit(ray, -std::f32::MAX, std::f32::MAX) {
            if let Some(mut rec2) = self.boundary.hit(ray, rec1.t + 0.0001, std::f32::MAX) {
                if rec1.t < t_min {
                    rec1.t = t_min;
                }
                if rec2.t > t_max {
                    rec2.t = t_max;
                }
                if rec1.t >= rec2.t {
                    return None;
                }
                if rec1.t < 0.0 {
                    rec1.t = 0.0;
                }
                let distance_inside_boundary = (rec2.t - rec1.t) * ray.direction.magnitude();
                let hit_distance = -(1.0 / self.density) * random_float().ln();
                if hit_distance < distance_inside_boundary {
                    let t = rec1.t + hit_distance / ray.direction.magnitude();
                    return Some(HitRecord {
                        t,
                        p: ray.point_at_parameter(t),
                        normal: vec3(1.0, 0.0, 0.0), //arbitrary
                        material: Some(&*self.phase_function),
                        u: 0.0,
                        v: 0.0,
                    });
                }
            }
        }
        None
    }

    fn bounding_box(&self, t0: f32, t1: f32) -> Option<AABB> {
        self.boundary.bounding_box(t0, t1)
    }
}

impl ConstantMedium {
    pub fn new(boundary: Box<dyn Hitable>, density: f32, texture: Box<dyn Texture>) -> Self {
        ConstantMedium {
            boundary,
            density,
            phase_function: Box::new(Isotropic(texture)),
        }
    }
}
