use crate::math::*;

pub fn parallel_for_2d(func: &Fn(Point2i) -> (), num_tiles: Point2i) {
    // TODO: actually implement me parallely
    for y in 0..num_tiles.y {
        for x in 0..num_tiles.x {
            func(Point2i::new(x, y));
        }
    }
}
