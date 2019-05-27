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

pub struct Bounds2D<T> {
    pub min: Vector2<T>,
    pub max: Vector2<T>,
}

impl<T: BaseNum> Bounds2D<T> {
    pub fn diagonal(&self) -> Vector2<T> {
        self.max - self.min
    }

    fn new(p: cgmath::Point2<T>) -> Bounds2D<T> {
        Bounds2D {
            min: p.to_vec(),
            max: p.to_vec(),
        }
    }
}

impl<T: Bounded> Default for Bounds2D<T> {
    fn default() -> Bounds2D<T> {
        Bounds2D {
            min: vec2(T::max_value(), T::max_value()),
            max: vec2(T::min_value(), T::min_value()),
        }
    }
}

pub type Bounds2Di = Bounds2D<i32>;

pub fn coordinate_system(v1: Vec3) -> (Vec3, Vec3, Vec3) {
    let v2;
    if v1.x.abs() > v1.y.abs() {
        v2 = vec3(-v1.z, 0.0, v1.x).normalize();
    } else {
        v2 = vec3(0.0, v1.z, -v1.y).normalize();
    }

    (v1, v2, v1.cross(v2))
}

pub fn face_forward(n: Normal3f, v: Vec3) -> Normal3f {
    if dot(n, v) < 0.0 {
        -n
    } else {
        n
    }
}
