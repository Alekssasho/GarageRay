use crate::material::*;
use crate::random::random_in_unit_sphere;
use crate::ray::Ray;
use crate::texture::*;

#[derive(Clone)]
pub struct Isotropic(pub Box<dyn Texture>);

impl Material for Isotropic {
    fn scatter(&self, ray: &Ray, rec: &HitRecord) -> Option<(Vec3, Ray)> {
        Some((
            self.0.value(rec.u, rec.v, &rec.p),
            Ray {
                origin: rec.p,
                direction: random_in_unit_sphere(),
                ..*ray
            },
        ))
    }
}
