use crate::hitable::HitRecord;
use crate::material::Material;
use crate::math::*;
use crate::random::random_in_unit_sphere;
use crate::ray::Ray;

pub struct Lambertian {
    pub albedo: Vec3,
}

impl Material for Lambertian {
    fn scatter(&self, _ray: &Ray, rec: &HitRecord) -> Option<(Vec3, Ray)> {
        let target = rec.p + rec.normal + random_in_unit_sphere();
        Some((
            self.albedo,
            Ray {
                origin: rec.p,
                direction: target - rec.p,
            },
        ))
    }
}
