use crate::hitable::HitRecord;
use crate::material::{Material, ScatterResult};
use crate::math::*;
use crate::random::random_in_unit_sphere;
use crate::ray::Ray;

#[derive(Clone)]
pub struct Metal {
    pub albedo: Vec3,
    pub fuzz: f32,
}

pub fn reflect(v: &Vec3, n: &Vec3) -> Vec3 {
    v - 2.0 * dot(*v, *n) * n
}

impl Material for Metal {
    fn scatter(&self, ray: &Ray, rec: &HitRecord) -> Option<ScatterResult> {
        let reflected = reflect(&ray.direction.normalize(), &rec.normal);
        Some(ScatterResult {
            attenuation: self.albedo,
            pdf: None,
            specular_ray: Some(Ray {
                origin: rec.p,
                direction: reflected + self.fuzz * random_in_unit_sphere(),
                ..*ray
            }),
        })
    }
}
