pub mod hitable_list;
pub mod sphere;

pub use hitable_list::HitableList;
pub use sphere::Sphere;

use crate::math::{vec3, Vec3};
use crate::ray::Ray;

#[derive(Clone, Copy)]
pub struct HitRecord {
    pub t: f32,
    pub p: Vec3,
    pub normal: Vec3,
}

impl Default for HitRecord {
    fn default() -> Self {
        HitRecord {
            t: 0.0,
            p: vec3(0.0, 0.0, 0.0),
            normal: vec3(0.0, 0.0, 0.0),
        }
    }
}

pub trait Hitable {
    fn hit(&self, ray: &Ray, t_min: f32, t_max: f32, rec: &mut HitRecord) -> bool;
}
