use crate::math::*;

pub mod checker_texture;
pub mod constant_texture;
pub mod image_texture;
pub mod noise_texture;

pub use checker_texture::CheckerTexture;
pub use constant_texture::ConstantTexture;
pub use image_texture::ImageTexture;
pub use noise_texture::NoiseTexture;

pub trait Texture: TextureClone {
    fn value(&self, u: f32, v: f32, p: &Vec3) -> Vec3;
}

// Box cloning implementation
pub trait TextureClone {
    fn box_clone(&self) -> Box<dyn Texture>;
}

impl<T> TextureClone for T
where
    T: 'static + Texture + Clone,
{
    fn box_clone(&self) -> Box<dyn Texture> {
        Box::new(self.clone())
    }
}

impl Clone for Box<dyn Texture> {
    fn clone(&self) -> Box<dyn Texture> {
        self.box_clone()
    }
}
