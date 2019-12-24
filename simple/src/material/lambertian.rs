use crate::hitable::HitRecord;
use crate::material::Material;
use crate::math::*;
use crate::random::random_in_unit_sphere;
use crate::ray::Ray;
use crate::texture::Texture;

pub struct Lambertian {
    pub albedo: Box<dyn Texture>,
}

impl Material for Lambertian {
    fn scatter(&self, ray: &Ray, rec: &HitRecord) -> Option<(Vec3, Ray)> {
        let target = rec.p + rec.normal + random_in_unit_sphere();
        Some((
            self.albedo.value(rec.u, rec.v, &rec.p),
            Ray {
                origin: rec.p,
                direction: target - rec.p,
                ..*ray
            },
        ))
    }
}
