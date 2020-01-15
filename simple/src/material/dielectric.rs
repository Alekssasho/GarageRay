use crate::hitable::HitRecord;
use crate::material::metal::reflect;
use crate::material::{Material, ScatterResult};
use crate::math::*;
use crate::random::random_float;
use crate::ray::Ray;

#[derive(Clone)]
pub struct Dielectric {
    pub ref_index: f32,
}

fn refract(v: &Vec3, n: &Vec3, ni_over_nt: f32) -> Option<Vec3> {
    let uv = v.normalize();
    let dt = dot(uv, *n);
    let discriminant = 1.0 - ni_over_nt * ni_over_nt * (1.0 - dt * dt);
    if discriminant > 0.0 {
        Some(ni_over_nt * (uv - n * dt) - n * discriminant.sqrt())
    } else {
        None
    }
}

fn schlick(cosine: f32, ref_index: f32) -> f32 {
    let r0 = (1.0 - ref_index) / (1.0 + ref_index);
    let r0 = r0 * r0;
    r0 + (1.0 - r0) * (1.0 - cosine).powf(5.0)
}

impl Material for Dielectric {
    fn scatter(&self, ray: &Ray, rec: &HitRecord) -> Option<ScatterResult> {
        let reflected = reflect(&ray.direction, &rec.normal);
        let (outward_normal, ni_over_nt, cosine) = if dot(ray.direction, rec.normal) > 0.0 {
            (
                -rec.normal,
                self.ref_index,
                self.ref_index * dot(ray.direction, rec.normal) / ray.direction.magnitude(),
            )
        } else {
            (
                rec.normal,
                1.0 / self.ref_index,
                -dot(ray.direction, rec.normal) / ray.direction.magnitude(),
            )
        };

        let refracted = refract(&ray.direction, &outward_normal, ni_over_nt);
        let reflect_probability = if refracted.is_some() {
            schlick(cosine, self.ref_index)
        } else {
            1.0
        };

        if random_float() < reflect_probability {
            Some(ScatterResult{
                albedo: vec3(1.0, 1.0, 1.0),
                scattered_ray: Ray {
                    origin: rec.p,
                    direction: reflected,
                    ..*ray
                },
                pdf: 0.0,
            })
        } else {
            Some(ScatterResult{
                albedo: vec3(1.0, 1.0, 1.0),
                scattered_ray: Ray {
                    origin: rec.p,
                    direction: refracted.unwrap(),
                    ..*ray
                },
                pdf: 0.0
            })
        }
    }
}
