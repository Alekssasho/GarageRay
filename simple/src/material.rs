mod lambertian;
mod metal;
pub use lambertian::Lambertian;
pub use metal::Metal;

use crate::hitable::HitRecord;
use crate::math::Vec3;
use crate::ray::Ray;

pub trait Material {
    fn scatter(&self, ray: &Ray, rec: &HitRecord) -> Option<(Vec3, Ray)>;
}
