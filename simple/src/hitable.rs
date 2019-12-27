mod box_hitable;
mod bvh;
mod constant_medium;
mod flip_normals;
pub mod hitable_list;
mod rect;
mod sphere;
mod transformations;

pub use box_hitable::BoxHitable;
pub use constant_medium::ConstantMedium;
pub use flip_normals::FlipNormals;
pub use rect::XYRect;
pub use rect::XZRect;
pub use rect::YZRect;
pub use sphere::MovingSphere;
pub use sphere::Sphere;
pub use transformations::{RotateY, Translate};

pub use bvh::BVHNode;

use crate::material::Material;
use crate::math::{vec3, Vec3};
use crate::ray::Ray;

#[derive(Clone, Copy)]
pub struct HitRecord<'a> {
    pub t: f32,
    pub p: Vec3,
    pub normal: Vec3,
    pub u: f32,
    pub v: f32,
    pub material: Option<&'a dyn Material>,
}

impl Default for HitRecord<'_> {
    fn default() -> Self {
        HitRecord {
            t: 0.0,
            p: vec3(0.0, 0.0, 0.0),
            normal: vec3(0.0, 0.0, 0.0),
            u: 0.0,
            v: 0.0,
            material: None,
        }
    }
}

pub trait Hitable {
    fn hit(&self, ray: &Ray, t_min: f32, t_max: f32) -> Option<HitRecord>;
    fn bounding_box(&self, t0: f32, t1: f32) -> Option<AABB>;
}

#[derive(Clone, Copy)]
pub struct AABB {
    pub min: Vec3,
    pub max: Vec3,
}

impl AABB {
    fn hit(&self, ray: &Ray, t_min: f32, t_max: f32) -> bool {
        let mut tmin = t_min;
        let mut tmax = t_max;
        for i in 0..3 {
            let t0 = ((self.min[i] - ray.origin[i]) / ray.direction[i])
                .min((self.max[i] - ray.origin[i]) / ray.direction[i]);
            let t1 = ((self.min[i] - ray.origin[i]) / ray.direction[i])
                .max((self.max[i] - ray.origin[i]) / ray.direction[i]);
            tmin = t0.max(tmin);
            tmax = t1.min(tmax);
            if tmax <= tmin {
                return false;
            }
        }
        true
    }
}

pub fn surrounding_box(box0: AABB, box1: AABB) -> AABB {
    let small = vec3(
        box0.min.x.min(box1.min.x),
        box0.min.y.min(box1.min.y),
        box0.min.z.min(box1.min.z),
    );
    let big = vec3(
        box0.max.x.max(box1.max.x),
        box0.max.y.max(box1.max.y),
        box0.max.z.max(box1.max.z),
    );
    AABB {
        min: small,
        max: big,
    }
}
