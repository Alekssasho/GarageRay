use crate::math::*;
use crate::texture::Texture;

use image::*;

#[derive(Clone)]
pub struct ImageTexture {
    image: image::DynamicImage,
    width: i32,
    height: i32,
}

impl Texture for ImageTexture {
    fn value(&self, u: f32, v: f32, _p: &Vec3) -> Vec3 {
        let mut i = (u * self.width as f32) as i32;
        let mut j = ((1.0 - v) * self.height as f32 - 0.001) as i32;
        if i < 0 {
            i = 0;
        }
        if j < 0 {
            j = 0;
        }
        if i > self.width - 1 {
            i = self.width - 1;
        }
        if j > self.height - 1 {
            j = self.height - 1;
        }
        let image::Rgba(data) = self.image.get_pixel(i as u32, j as u32);
        let r = data[0] as f32 / 255.0;
        let g = data[1] as f32 / 255.0;
        let b = data[2] as f32 / 255.0;
        vec3(r, g, b)
    }
}

impl ImageTexture {
    pub fn new(img: image::DynamicImage) -> ImageTexture {
        ImageTexture {
            width: img.dimensions().0 as i32,
            height: img.dimensions().1 as i32,
            image: img,
        }
    }
}
