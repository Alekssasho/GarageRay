use crate::cameras::CameraSample;
use crate::math::*;

pub trait Sampler {
    fn clone(&self) -> Box<Sampler>;

    fn start_pixel(&self, pixel: Point2i) -> SamplerPixel;
    fn get_camera_sample(&self, pixel: Point2i) -> CameraSample;
    fn get_samples_per_pixel(&self) -> i32;
    fn get_2d(&self) -> Point2;
}

pub struct SamplerPixel();

impl Iterator for SamplerPixel {
    type Item = ();

    fn next(&mut self) -> Option<()> {
        None
    }
}
