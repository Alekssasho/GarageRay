// Matrix type is column major in cgmath, but row major in book
// Most likely cgmath is implemented with right hand rule, but book is left handed

pub type Vec3 = cgmath::Vector3<f32>;

pub use cgmath::dot;
pub use cgmath::vec3;
pub use cgmath::EuclideanSpace;
pub use cgmath::InnerSpace;
pub use cgmath::VectorSpace;
