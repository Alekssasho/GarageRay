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
        Bounds3Df::default()
    }

    fn intersect(
        &self,
        shape: &Shape,
        ray: &Ray,
        test_alpha_texture: bool,
    ) -> (bool, f32, SurfaceInteraction) {
        (
            false,
            ray.t_max.get(),
            SurfaceInteraction::delete_me_default(),
        )
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
