use crate::hitable::HitRecord;
use crate::material::{Material, ScatterResult};
use crate::math::*;
use crate::random::random_cosine_direction;
use crate::ray::Ray;
use crate::texture::Texture;
use crate::onb::*;

#[derive(Clone)]
pub struct Lambertian {
    pub albedo: Box<dyn Texture>,
}

impl Material for Lambertian {
    fn scatter(&self, ray: &Ray, rec: &HitRecord) -> Option<ScatterResult> {
        let uvw = ONB::build_from_w(&rec.normal);
        let direction = uvw.local_vec(&random_cosine_direction());
        let scattered_ray = Ray {
            origin: rec.p,
            direction: (direction).normalize(),
            ..*ray
        };
        Some(ScatterResult{
            albedo: self.albedo.value(rec.u, rec.v, &rec.p),
            pdf: dot(uvw.w, scattered_ray.direction) / std::f32::consts::PI,
            scattered_ray,
        })
    }

    fn scattering_pdf(&self, _ray: &Ray, rec: &HitRecord, scattered: &Ray) -> f32{
        let mut cosine = dot(rec.normal, scattered.direction.normalize());
        if cosine < 0.0 {
            cosine = 0.0;
        }
        cosine / std::f32::consts::PI
    }
}
