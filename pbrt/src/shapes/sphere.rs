use super::*;
use crate::math::*;

pub struct Sphere {
    radius: f32,
    z_min: f32,
    z_max: f32,
    theta_min: f32,
    theta_max: f32,
    phi_max: f32,
}

impl ShapeInterface for Sphere {
    fn object_bound(&self) -> Bounds3Df {
        Bounds3Df {
            min: Point3::new(-self.radius, self.radius, self.z_min),
            max: Point3::new(-self.radius, self.radius, self.z_max),
        }
    }

    fn intersect<'a>(
        &self,
        shape: &'a Shape,
        r: &Ray,
        test_alpha_texture: bool,
    ) -> Option<(f32, SurfaceInteraction<'a>)> {
        let ray = shape.world_to_object.transform_ray(r); // TODO: this should be with floating point errors
                                                          //TODO: use EFloat instead of floats (page 135)
        let a = ray.d.x * ray.d.x + ray.d.y * ray.d.y + ray.d.z * ray.d.z;
        let b = 2.0 * (ray.d.x * ray.o.x + ray.d.y * ray.o.y + ray.d.z * ray.o.z);
        let c =
            ray.o.x * ray.o.x + ray.o.y * ray.o.y + ray.o.z * ray.o.z - self.radius * self.radius;

        let (t0, t1) = match quadratic(a, b, c) {
            Some((x, y)) => (x, y),
            None => return None,
        };

        // use lower/upper bound as per page 136
        if t0 > ray.t_max.get() || t1 <= 0.0 {
            return None;
        }

        let mut t_shape_hit = if t0 > 0.0 {
            t0
        } else {
            if t1 > ray.t_max.get() {
                return None;
            }
            t1
        };

        let mut p_hit = ray.at(t_shape_hit);
        if p_hit.x == 0.0 && p_hit.y == 0.0 {
            p_hit.x = 1e-5 * self.radius;
        }
        // Refine sphere intersection point 255
        let mut phi = p_hit.y.atan2(p_hit.x);
        if phi < 0.0 {
            phi += 2.0 * std::f32::consts::PI;
        }
        if (self.z_min > -self.radius && p_hit.z < self.z_min)
            || (self.z_max < self.radius && p_hit.z > self.z_max)
            || phi > self.phi_max
        {
            // TODO: use t1.upperbound
            if t_shape_hit == t1 || t1 > ray.t_max.get() {
                return None;
            }
            t_shape_hit = t1;
            p_hit = ray.at(t_shape_hit);
            if p_hit.x == 0.0 && p_hit.y == 0.0 {
                p_hit.x = 1e-5 * self.radius;
            }
            // Refine sphere intersection point 255
            phi = p_hit.y.atan2(p_hit.x);
            if phi < 0.0 {
                phi += 2.0 * std::f32::consts::PI;
            }
            if (self.z_min > -self.radius && p_hit.z < self.z_min)
                || (self.z_max < self.radius && p_hit.z > self.z_max)
                || phi > self.phi_max
            {
                return None;
            }
        }

        let u = phi / self.phi_max;
        let theta = clamp(p_hit.z / self.radius, -1.0, 1.0).acos();
        let v = (theta - self.theta_min) / (self.theta_max - self.theta_min);

        let z_radius = (p_hit.x * p_hit.x + p_hit.y * p_hit.y).sqrt();
        let inv_z_radius = 1.0 / z_radius;
        let cos_phi = p_hit.x * inv_z_radius;
        let sin_phi = p_hit.y * inv_z_radius;
        let dpdu = Vec3::new(-self.phi_max * p_hit.y, self.phi_max * p_hit.x, 0.0);
        let dpdv = (self.theta_max - self.theta_min)
            * Vec3::new(
                p_hit.z * cos_phi,
                p_hit.z * sin_phi,
                -self.radius * theta.sin(),
            );
        let d2Pduu = -self.phi_max * self.phi_max * Vec3::new(p_hit.x, p_hit.y, 0.0);
        let d2Pduv = (self.theta_max - self.theta_min)
            * p_hit.z
            * self.phi_max
            * Vec3::new(-sin_phi, cos_phi, 0.0);
        let d2Pdvv = -(self.theta_max - self.theta_min)
            * (self.theta_max - self.theta_min)
            * Vec3::new(p_hit.x, p_hit.y, p_hit.z);
        let E = dot(dpdu, dpdu);
        let F = dot(dpdu, dpdv);
        let G = dot(dpdv, dpdv);
        let N = dpdu.cross(dpdv).normalize();
        let e = dot(N, d2Pduu);
        let f = dot(N, d2Pduv);
        let g = dot(N, d2Pdvv);
        let inv_egf2 = 1.0 / (E * G - F * F);
        let dndu =
            Normal3f::from((f * F - e * g) * inv_egf2 * dpdu + (e * F - f * E) * inv_egf2 * dpdv);
        let dndv =
            Normal3f::from((g * F - f * G) * inv_egf2 * dpdu + (f * F - g * E) * inv_egf2 * dpdv);

        // TODO: calculate this
        let p_error = Vec3::zero();
        let isect =
            shape
                .object_to_world
                .transform_surface_interaction(SurfaceInteraction::<'a>::new(
                    p_hit,
                    p_error,
                    Point2::new(u, v),
                    -ray.d,
                    dpdu,
                    dpdv,
                    dndu,
                    dndv,
                    ray.time,
                    shape,
                ));
        Some((t_shape_hit, isect))
    }

    fn area(&self) -> f32 {
        0.0
    }
}

impl Sphere {
    pub fn new(
        object_to_world: Transform,
        world_to_object: Transform,
        reverse_orientation: bool,
        radius: f32,
        z_min: f32,
        z_max: f32,
        phi_max: f32,
    ) -> Shape {
        Shape::new(
            object_to_world,
            world_to_object,
            reverse_orientation,
            Box::new(Sphere {
                radius,
                z_min: clamp(min(z_min, z_max), -radius, radius),
                z_max: clamp(max(z_min, z_max), -radius, radius),
                theta_min: clamp(z_min / radius, -1.0, 1.0).acos(),
                theta_max: clamp(z_max / radius, -1.0, 1.0).acos(),
                phi_max: Rad(clamp(phi_max, 0.0, 360.0)).0,
            }),
        )
    }

    pub fn unit() -> Shape {
        Sphere::new(
            Transform::default(),
            Transform::default(),
            false,
            1.0,
            -1.0,
            1.0,
            360.0,
        )
    }
}
