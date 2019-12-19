pub mod hitable_list;
pub mod sphere;

pub use sphere::MovingSphere;
pub use sphere::Sphere;

use crate::material::Material;
use crate::math::{vec3, Vec3};
use crate::ray::Ray;

#[derive(Clone, Copy)]
pub struct HitRecord<'a> {
    pub t: f32,
    pub p: Vec3,
    pub normal: Vec3,
    pub material: Option<&'a dyn Material>,
}

impl Default for HitRecord<'_> {
    fn default() -> Self {
        HitRecord {
            t: 0.0,
            p: vec3(0.0, 0.0, 0.0),
            normal: vec3(0.0, 0.0, 0.0),
            material: None,
        }
    }
}

pub trait Hitable {
    fn hit(&self, ray: &Ray, t_min: f32, t_max: f32) -> Option<HitRecord>;
}
