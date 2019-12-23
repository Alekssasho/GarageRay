use crate::math::Vec3;
use crate::texture::Texture;

pub struct ConstantTexture(pub Vec3);

impl Texture for ConstantTexture {
    fn value(&self, _u: f32, _v: f32, _p: &Vec3) -> Vec3 {
        self.0
    }
}
