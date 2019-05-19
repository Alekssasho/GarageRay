use crate::math::*;

pub fn parallel_for_2d(lambda: &Fn(Point2i) -> (), num_tiles: Point2i) {
    lambda(num_tiles);
    // TODO: actually implement me
}