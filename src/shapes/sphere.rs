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
        Bounds3Df{
            min: Point3::new(-self.radius, self.radius, self.z_min),
            max: Point3::new(-self.radius, self.radius, self.z_max)
        }
    }

    fn intersect(
        &self,
        shape: &Shape,
        r: &Ray,
        test_alpha_texture: bool,
    ) -> (bool, f32, SurfaceInteraction) {
        let phi: f32;
        let p_hit: Point3;
        let ray = shape.world_to_object.transform_ray(r); // TODO: this should be with floating point errors
        //TODO: use EFloat instead of floats (page 135)
        let a = ray.d.x * ray.d.x + ray.d.y * ray.d.y + ray.d.z * ray.d.z;
        let b = 2.0 * (ray.d.x * ray.o.x + ray.d.y * ray.o.y + ray.d.z * ray.o.z);
        let c = ray.o.x * ray.o.x + ray.o.y * ray.o.y + ray.o.z * ray.o.z - self.radius * self.radius;

        let (t0, t1) = match quadratic(a, b, c) {
            None => return (false, 0.0, SurfaceInteraction::delete_me_default()),
            Some((x, y)) => (x, y)
        };


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
        Shape::new(object_to_world, world_to_object, reverse_orientation, Box::new(Sphere{
            radius,
            z_min: clamp(min(z_min, z_max), -radius, radius),
            z_max: clamp(max(z_min, z_max), -radius, radius),
            theta_min: clamp(z_min / radius, -1.0, 1.0).acos(),
            theta_max: clamp(z_max / radius, -1.0, 1.0).acos(),
            phi_max: Rad(clamp(phi_max, 0.0, 360.0)).0,
        }))
    }

    pub fn unit() -> Shape {
        Sphere::new(Transform::default(), Transform::default(), false, 1.0, -1.0, 1.0, 360.0)
    }
}
