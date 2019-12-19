use crate::hitable::{HitRecord, Hitable};
use crate::material::Material;
use crate::math::*;
use crate::ray::Ray;

pub struct Sphere {
    pub center: Vec3,
    pub radius: f32,
    pub material: Box<dyn Material>,
}

impl Hitable for Sphere {
    fn hit(&self, ray: &Ray, t_min: f32, t_max: f32) -> Option<HitRecord> {
        let oc = ray.origin - self.center;
        let a = dot(ray.direction, ray.direction);
        let b = dot(oc, ray.direction);
        let c = dot(oc, oc) - self.radius * self.radius;
        let discriminant = b * b - a * c;
        if discriminant > 0.0 {
            let temp = (-b - discriminant.sqrt()) / a;
            if temp < t_max && temp > t_min {
                let p = ray.point_at_parameter(temp);
                return Some(HitRecord {
                    t: temp,
                    p: p,
                    normal: (p - self.center) / self.radius,
                    material: Some(&*self.material),
                });
            }
            let temp = (-b + discriminant.sqrt()) / a;
            if temp < t_max && temp > t_min {
                let p = ray.point_at_parameter(temp);
                return Some(HitRecord {
                    t: temp,
                    p: p,
                    normal: (p - self.center) / self.radius,
                    material: Some(&*self.material),
                });
            }
        }
        None
    }
}

pub struct MovingSphere {
    pub center0: Vec3,
    pub center1: Vec3,
    pub radius: f32,
    pub material: Box<dyn Material>,
    pub time0: f32,
    pub time1: f32,
}

impl MovingSphere {
    fn center(&self, time: f32) -> Vec3 {
        self.center0 + ((time - self.time0) / (self.time1 - self.time0)) * (self.center1 - self.center0)
    }
}

impl Hitable for MovingSphere {
    fn hit(&self, ray: &Ray, t_min: f32, t_max: f32) -> Option<HitRecord> {
        let oc = ray.origin - self.center(ray.time);
        let a = dot(ray.direction, ray.direction);
        let b = dot(oc, ray.direction);
        let c = dot(oc, oc) - self.radius * self.radius;
        let discriminant = b * b - a * c;
        if discriminant > 0.0 {
            let temp = (-b - discriminant.sqrt()) / a;
            if temp < t_max && temp > t_min {
                let p = ray.point_at_parameter(temp);
                return Some(HitRecord {
                    t: temp,
                    p: p,
                    normal: (p - self.center(ray.time)) / self.radius,
                    material: Some(&*self.material),
                });
            }
            let temp = (-b + discriminant.sqrt()) / a;
            if temp < t_max && temp > t_min {
                let p = ray.point_at_parameter(temp);
                return Some(HitRecord {
                    t: temp,
                    p: p,
                    normal: (p - self.center(ray.time)) / self.radius,
                    material: Some(&*self.material),
                });
            }
        }
        None
    }
}