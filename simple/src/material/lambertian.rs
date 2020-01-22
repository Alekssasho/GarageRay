use crate::hitable::HitRecord;
use crate::material::{Material, ScatterResult};
use crate::math::*;
use crate::pdf::*;
use crate::ray::Ray;
use crate::texture::Texture;

#[derive(Clone)]
pub struct Lambertian {
    pub albedo: Box<dyn Texture>,
}

impl Material for Lambertian {
    fn scatter(&self, _ray: &Ray, rec: &HitRecord) -> Option<ScatterResult> {
        Some(ScatterResult {
            attenuation: self.albedo.value(rec.u, rec.v, &rec.p),
            pdf: Some(Box::new(Cosine::new(&rec.normal))),
            specular_ray: None,
        })
    }

    fn scattering_pdf(&self, _ray: &Ray, rec: &HitRecord, scattered: &Ray) -> f32 {
        let mut cosine = dot(rec.normal, scattered.direction.normalize());
        if cosine < 0.0 {
            cosine = 0.0;
        }
        cosine / std::f32::consts::PI
    }
}
