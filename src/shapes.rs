mod sphere;

pub use sphere::Sphere;

use crate::core::SurfaceInteraction;
use crate::math::{Bounds3Df, Transform};
use crate::ray::Ray;

pub trait ShapeInterface {
    fn object_bound(&self) -> Bounds3Df;
    fn world_bound(&self, shape: &Shape) -> Bounds3Df {
        shape.object_to_world.transform_bounds(&self.object_bound())
    }
    fn intersect(
        &self,
        shape: &Shape,
        ray: &Ray,
        test_alpha_texture: bool,
    ) -> (bool, f32, SurfaceInteraction);

    fn intersect_p(&self, shape: &Shape, ray: &Ray, test_alpha_texture: bool) -> bool {
        self.intersect(shape, ray, test_alpha_texture).0
    }

    fn area(&self) -> f32;
}

pub struct Shape {
    pub reverse_orientation: bool,
    pub transform_swaps_handedness: bool,
    pub object_to_world: Transform, // TODO: this should be references
    pub world_to_object: Transform,
    shape_impl: Box<dyn ShapeInterface>,
}

impl Shape {
    pub fn new(
        object_to_world: Transform,
        world_to_object: Transform,
        reverse_orientation: bool,
        shape_impl: Box<dyn ShapeInterface>,
    ) -> Shape {
        Shape {
            reverse_orientation,
            transform_swaps_handedness: object_to_world.swaps_handedness(),
            object_to_world,
            world_to_object,
            shape_impl,
        }
    }

    pub fn object_bound(&self) -> Bounds3Df {
        self.shape_impl.object_bound()
    }

    pub fn world_bound(&self) -> Bounds3Df {
        self.shape_impl.world_bound(self)
    }

    fn intersect(&self, ray: &Ray, test_alpha_texture: bool) -> (bool, f32, SurfaceInteraction) {
        self.shape_impl.intersect(self, ray, test_alpha_texture)
    }

    fn intersect_p(&self, ray: &Ray, test_alpha_texture: bool) -> bool {
        self.shape_impl.intersect_p(self, ray, test_alpha_texture)
    }

    fn area(&self) -> f32 {
        self.shape_impl.area()
    }
}
