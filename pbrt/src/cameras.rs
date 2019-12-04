use crate::core::Film;
use crate::ray::RayDifferential;

pub trait Camera {
    fn film(&self) -> Film;
    fn generate_ray_differential(&self, sample: CameraSample) -> (RayDifferential, f32);
}

pub struct CameraSample;
