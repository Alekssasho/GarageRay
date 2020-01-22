use crate::material::*;
use crate::random::random_in_unit_sphere;
use crate::ray::Ray;
use crate::texture::*;

#[derive(Clone)]
pub struct Isotropic(pub Box<dyn Texture>);

impl Material for Isotropic {
    fn scatter(&self, ray: &Ray, rec: &HitRecord) -> Option<ScatterResult> {
        Some(ScatterResult {
            albedo: self.0.value(rec.u, rec.v, &rec.p),
            scattered_ray: Ray {
                origin: rec.p,
                direction: random_in_unit_sphere(),
                ..*ray
            },
            pdf: 0.0,
        })
    }
}
