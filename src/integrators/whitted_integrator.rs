use super::sample_integrator::*;
use crate::cameras::Camera;
use crate::core::*;
use crate::integrators::Integrator;
use crate::math::{Bounds2Di, Point2i, Vec2i};
use crate::ray::*;
use crate::samplers::Sampler;
use crate::spectrum::Spectrum;

pub struct WhittedIntegrator {
    max_depth: i32,
}

impl SampleIntegratorInterface for WhittedIntegrator {
    fn preprocess(&self, scene: &Scene, sampler: &Sampler) {}

    fn light_incoming(
        &self,
        sample_integrator: &SampleIntegrator,
        ray: &RayDifferential,
        scene: &Scene,
        sampler: &mut Sampler,
        // Memory Arena
        depth: i32,
    ) -> Spectrum {
        let mut light = Spectrum::new();
        let (isect, has_intersected) = scene.intersect(&ray.ray);
        if !has_intersected {
            // Page 34

            return light;
        }
        light
    }
}
