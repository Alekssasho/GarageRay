mod dielectric;
mod diffuse_light;
mod isotropic;
mod lambertian;
mod metal;

pub use dielectric::Dielectric;
pub use diffuse_light::DiffuseLight;
pub use isotropic::Isotropic;
pub use lambertian::Lambertian;
pub use metal::Metal;

use crate::hitable::HitRecord;
use crate::math::*;
use crate::ray::Ray;

pub struct ScatterResult {
    pub albedo: Vec3,
    pub scattered_ray: Ray,
    pub pdf: f32,
}

pub trait Material: MaterialClone {
    fn scatter(&self, ray: &Ray, rec: &HitRecord) -> Option<ScatterResult>;
    fn scattering_pdf(&self, _ray: &Ray, _rec: &HitRecord, _scattered: &Ray) -> f32 {
        0.0
    }
    fn emitted(&self, _u: f32, _v: f32, _p: &Vec3) -> Vec3 {
        Vec3::zero()
    }
}

// Box cloning implementation
pub trait MaterialClone: Sync {
    fn box_clone(&self) -> Box<dyn Material>;
}

impl<T> MaterialClone for T
where
    T: 'static + Material + Clone,
{
    fn box_clone(&self) -> Box<dyn Material> {
        Box::new(self.clone())
    }
}

impl Clone for Box<dyn Material> {
    fn clone(&self) -> Box<dyn Material> {
        self.box_clone()
    }
}
