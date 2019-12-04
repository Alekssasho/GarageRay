// Matrix type is column major in cgmath, but row major in book
// Most likely cgmath is implemented with right hand rule, but book is left handed

//pub type Vec2 = cgmath::Vector2<f32>;
pub type Vec2i = cgmath::Vector2<i32>;
pub type Vec3 = cgmath::Vector3<f32>;

pub type Point2 = cgmath::Point2<f32>;
pub type Point2i = cgmath::Point2<i32>;
pub type Point3 = cgmath::Point3<f32>;

pub type Normal3f = cgmath::Vector3<f32>;

pub use cgmath::dot;
pub use cgmath::vec3;
pub use cgmath::EuclideanSpace;

use cgmath::*;

pub fn min<T: BaseNum>(lhs: T, rhs: T) -> T {
    match lhs.partial_cmp(&rhs) {
        Some(std::cmp::Ordering::Less) => lhs,
        Some(std::cmp::Ordering::Equal) => lhs,
        Some(std::cmp::Ordering::Greater) => rhs,
        None => panic!(),
    }
}

pub fn max<T: BaseNum>(lhs: T, rhs: T) -> T {
    match lhs.partial_cmp(&rhs) {
        Some(std::cmp::Ordering::Less) => rhs,
        Some(std::cmp::Ordering::Equal) => rhs,
        Some(std::cmp::Ordering::Greater) => lhs,
        None => panic!(),
    }
}

pub fn clamp<T: BaseNum>(val: T, low: T, high: T) -> T {
    if val < low {
        low
    } else if val > high {
        high
    } else {
        val
    }
}

pub fn lerp<T: BaseNum>(t: T, lhs: T, rhs: T) -> T {
    (T::from(1.0).unwrap() - t) * lhs + t * rhs
}