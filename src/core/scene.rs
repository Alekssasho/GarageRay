use crate::lights;
use crate::accelerators;
use crate::math::Vec3;

pub struct Scene {
    lights: Vec<Box<lights::Light>>,
    aggregate: Box<accelerators::Primitive>,

    world_bound: Vec3,
}

impl Scene {
    fn new(lights: Vec<Box<lights::Light>>, aggregate: Box<accelerators::Primitive>) -> Scene{
        let world_bound = aggregate.world_bounds();

        for light in &lights {
            light.preprocess();
        }

        Scene { lights, aggregate, world_bound }
    }

    // Intersect 24
    // IntersectP 24
}