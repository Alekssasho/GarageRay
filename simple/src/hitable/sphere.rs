use crate::hitable::{surrounding_box, HitRecord, Hitable, AABB};
use crate::material::Material;
use crate::math::*;
use crate::onb::*;
use crate::random::*;
use crate::ray::Ray;

#[derive(Clone)]
pub struct Sphere {
    pub center: Vec3,
    pub radius: f32,
    pub material: Box<dyn Material>,
}

fn get_sphere_uv(p: Vec3) -> (f32, f32) {
    let phi = p.z.atan2(p.x);
    let theta = p.y.asin();
    (
        1.0 - (phi + std::f32::consts::PI) / (2.0 * std::f32::consts::PI),
        (theta + std::f32::consts::PI / 2.0) / std::f32::consts::PI,
    )
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
                let (u, v) = get_sphere_uv((p - self.center) / self.radius);
                return Some(HitRecord {
                    t: temp,
                    p,
                    u,
                    v,
                    normal: (p - self.center) / self.radius,
                    material: Some(&*self.material),
                });
            }
            let temp = (-b + discriminant.sqrt()) / a;
            if temp < t_max && temp > t_min {
                let p = ray.point_at_parameter(temp);
                let (u, v) = get_sphere_uv((p - self.center) / self.radius);
                return Some(HitRecord {
                    t: temp,
                    p,
                    u,
                    v,
                    normal: (p - self.center) / self.radius,
                    material: Some(&*self.material),
                });
            }
        }
        None
    }

    fn bounding_box(&self, _t0: f32, _t1: f32) -> Option<AABB> {
        Some(AABB {
            min: self.center - vec3(self.radius, self.radius, self.radius),
            max: self.center + vec3(self.radius, self.radius, self.radius),
        })
    }

    fn pdf_value(&self, o: &Vec3, v: &Vec3) -> f32 {
        if let Some(_) = self.hit(
            &Ray {
                origin: *o,
                direction: *v,
                time: 0.0,
            },
            0.001,
            std::f32::MAX,
        ) {
            let cos_theta_max =
                (1.0 - self.radius * self.radius / (self.center - o).magnitude2()).sqrt();
            let solid_angle = 2.0 * std::f32::consts::PI * (1.0 - cos_theta_max);
            1.0 / solid_angle
        } else {
            0.0
        }
    }
    fn random(&self, o: &Vec3) -> Vec3 {
        let direction = self.center - o;
        let distance_squared = direction.magnitude2();
        let uvw = ONB::build_from_w(&direction);
        uvw.local_vec(&random_to_sphere(self.radius, distance_squared))
    }
}

fn random_to_sphere(radius: f32, distance_squared: f32) -> Vec3 {
    let r1 = random_float();
    let r2 = random_float();
    let z = 1.0 + r2 * ((1.0 - radius * radius / distance_squared).sqrt() - 1.0);
    let phi = 2.0 * std::f32::consts::PI * r1;
    let x = phi.cos() * (1.0 - z * z).sqrt();
    let y = phi.sin() * (1.0 - z * z).sqrt();
    vec3(x, y, z)
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
        self.center0
            + ((time - self.time0) / (self.time1 - self.time0)) * (self.center1 - self.center0)
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
                let (u, v) = get_sphere_uv((p - self.center(ray.time)) / self.radius);
                return Some(HitRecord {
                    t: temp,
                    p,
                    u,
                    v,
                    normal: (p - self.center(ray.time)) / self.radius,
                    material: Some(&*self.material),
                });
            }
            let temp = (-b + discriminant.sqrt()) / a;
            if temp < t_max && temp > t_min {
                let p = ray.point_at_parameter(temp);
                let (u, v) = get_sphere_uv((p - self.center(ray.time)) / self.radius);
                return Some(HitRecord {
                    t: temp,
                    p,
                    u,
                    v,
                    normal: (p - self.center(ray.time)) / self.radius,
                    material: Some(&*self.material),
                });
            }
        }
        None
    }

    fn bounding_box(&self, t0: f32, t1: f32) -> Option<AABB> {
        let box0 = AABB {
            min: self.center(t0) - vec3(self.radius, self.radius, self.radius),
            max: self.center(t0) + vec3(self.radius, self.radius, self.radius),
        };

        let box1 = AABB {
            min: self.center(t1) - vec3(self.radius, self.radius, self.radius),
            max: self.center(t1) + vec3(self.radius, self.radius, self.radius),
        };

        Some(surrounding_box(box0, box1))
    }
}
