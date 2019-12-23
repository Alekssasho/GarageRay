use crate::math::*;

pub mod checker_texture;
pub mod constant_texture;

pub use checker_texture::CheckerTexture;
pub use constant_texture::ConstantTexture;

pub trait Texture {
    fn value(&self, u: f32, v: f32, p: &Vec3) -> Vec3;
}
