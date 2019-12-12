use crate::hitable::HitRecord;
use crate::material::Material;
use crate::math::*;
use crate::ray::Ray;
use crate::random::random_in_unit_sphere;

pub struct Metal {
    pub albedo: Vec3,
    pub fuzz: f32,
}

fn reflect(v: &Vec3, n: &Vec3) -> Vec3 {
    v - 2.0 * dot(*v, *n) * n
}

impl Material for Metal {
    fn scatter(&self, ray: &Ray, rec: &HitRecord) -> Option<(Vec3, Ray)> {
        let reflected = reflect(&ray.direction.normalize(), &rec.normal);
        let scattered = Ray {
            origin: rec.p,
            direction: reflected + self.fuzz * random_in_unit_sphere(),
        };
        if dot(scattered.direction, rec.normal) > 0.0 {
            Some((self.albedo, scattered))
        } else {
            None
        }
    }
}
