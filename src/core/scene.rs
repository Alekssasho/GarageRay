use crate::accelerators;
use crate::core::*;
use crate::lights;
use crate::math::Vec3;
use crate::ray::*;

pub struct Scene {
    pub lights: Vec<Box<lights::Light>>,
    aggregate: Box<accelerators::Primitive>,

    world_bound: Vec3,
}

impl Scene {
    fn new(lights: Vec<Box<lights::Light>>, aggregate: Box<accelerators::Primitive>) -> Scene {
        let world_bound = aggregate.world_bounds();

        for light in &lights {
            light.preprocess();
        }

        Scene {
            lights,
            aggregate,
            world_bound,
        }
    }

    pub fn intersect(&self, ray: &Ray) -> (SurfaceInteraction, bool) {
        (SurfaceInteraction::new(), true)
    }
    // IntersectP 24
}
