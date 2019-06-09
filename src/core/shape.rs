use crate::math::{Transform, Bounds3Df};

trait ShapeInterface {
    fn object_bound(&self) -> Bounds3Df;
    fn world_bound(&self, shape: &Shape) -> Bounds3Df {
        shape.object_to_world.transform_bounds(&self.object_bound())
    }
}

// Delete me !!!
struct ShapeImpl {

}

impl ShapeInterface for ShapeImpl {
    fn object_bound(&self) -> Bounds3Df {
        Bounds3Df::default()
    }
}

pub struct Shape{
    pub reverse_orientation: bool,
    pub transform_swaps_handedness: bool,
    pub object_to_world: Transform, // TODO: this should be references
    pub world_to_object: Transform,
    shape_impl: Box<ShapeInterface>,
}

impl Shape {
    pub fn new(object_to_world: Transform, world_to_object: Transform, reverse_orientation: bool) -> Shape {
        Shape {
            reverse_orientation,
            transform_swaps_handedness: object_to_world.swaps_handedness(),
            object_to_world,
            world_to_object,
            shape_impl: Box::new(ShapeImpl{})
        }
    }

    pub fn object_bound(&self) -> Bounds3Df {
        self.shape_impl.object_bound()
    }

    pub fn world_bound(&self) -> Bounds3Df {
        self.shape_impl.world_bound(self)
    }
}

// TODO: Delete me
impl Default for Shape {
    fn default() -> Shape {
        Shape::new(Transform::default(), Transform::default(), false)
    }
}