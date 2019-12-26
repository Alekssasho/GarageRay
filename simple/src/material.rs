mod dielectric;
mod diffuse_light;
mod lambertian;
mod metal;

pub use dielectric::Dielectric;
pub use diffuse_light::DiffuseLight;
pub use lambertian::Lambertian;
pub use metal::Metal;

use crate::hitable::HitRecord;
use crate::math::*;
use crate::ray::Ray;

pub trait Material {
    fn scatter(&self, ray: &Ray, rec: &HitRecord) -> Option<(Vec3, Ray)>;
    fn emitted(&self, _u: f32, _v: f32, _p: &Vec3) -> Vec3 {
        Vec3::zero()
    }
}
