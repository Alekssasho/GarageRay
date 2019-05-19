use crate::math::Point2i;

pub trait Sampler {
    fn clone(&self) -> Box<Sampler>;

    fn start_pixel(&self, pixel: Point2i) -> SamplerPixel;
}

pub struct SamplerPixel();

impl Iterator for SamplerPixel {
    type Item = ();

    fn next(&mut self) -> Option<()> {
        None
    }
}