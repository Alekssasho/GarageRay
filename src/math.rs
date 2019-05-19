pub type Vec2 = cgmath::Vector2<f32>;
pub type Vec2i = cgmath::Vector2<i32>;
pub type Vec3 = cgmath::Vector3<f32>;

pub type Point2 = cgmath::Point2<f32>;
pub type Point2i = cgmath::Point2<i32>;
pub type Point3 = cgmath::Point3<f32>;

pub struct Bounds2Di {
    pub min: Vec2i,
    pub max: Vec2i,
}

impl Bounds2Di {
    pub fn diagonal(&self) -> Vec2i {
        self.max - self.min
    }
}