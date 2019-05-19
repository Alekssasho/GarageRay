use crate::math::Vec3;

pub trait Primitive {
    fn world_bounds(&self) -> Vec3;
}