use crate::hitable::HitRecord;
use crate::material::{Material, ScatterResult};
use crate::math::*;
use crate::random::random_in_unit_sphere;
use crate::ray::Ray;
use crate::texture::Texture;

#[derive(Clone)]
pub struct Lambertian {
    pub albedo: Box<dyn Texture>,
}

impl Material for Lambertian {
    fn scatter(&self, ray: &Ray, rec: &HitRecord) -> Option<ScatterResult> {
        // let target = rec.p + rec.normal + random_in_unit_sphere();
        // let scattered_ray = Ray {
        //     origin: rec.p,
        //     direction: (target - rec.p).normalize(),
        //     ..*ray
        // };
        // Some(ScatterResult{
        //     albedo: self.albedo.value(rec.u, rec.v, &rec.p),
        //     pdf: dot(rec.normal, scattered_ray.direction) / std::f32::consts::PI,
        //     scattered_ray,
        // })

        let direction = loop {
            let direction = random_in_unit_sphere();
            if dot(direction, rec.normal) >= 0.0 {
                break direction;
            }
        };
        let scattered_ray = Ray {
            origin: rec.p,
            direction: direction.normalize(),
            ..*ray
        };
        Some(ScatterResult{
            albedo: self.albedo.value(rec.u, rec.v, &rec.p),
            pdf: 0.5 / std::f32::consts::PI,
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
