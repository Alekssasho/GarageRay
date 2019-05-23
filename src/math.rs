//pub type Vec2 = cgmath::Vector2<f32>;
pub type Vec2i = cgmath::Vector2<i32>;
pub type Vec3 = cgmath::Vector3<f32>;

pub type Point2 = cgmath::Point2<f32>;
pub type Point2i = cgmath::Point2<i32>;
pub type Point3 = cgmath::Point3<f32>;

pub type Normal3f = cgmath::Vector3<f32>;

pub use cgmath::dot;
pub use cgmath::vec3;

use cgmath::*;

pub struct Bounds2Di {
    pub min: Vec2i,
    pub max: Vec2i,
}

impl Bounds2Di {
    pub fn diagonal(&self) -> Vec2i {
        self.max - self.min
    }
}

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
