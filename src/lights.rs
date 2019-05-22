use crate::core::*;
use crate::math::*;
use crate::ray::RayDifferential;
use crate::spectrum::Spectrum;

pub trait Light {
    fn preprocess(&self) -> ();
    fn light_emission(&self, ray: &RayDifferential) -> Spectrum;
    // This should be Interaction and not surfaceinteraction
    fn sample_light_incoming(
        &self,
        isect: &SurfaceInteraction,
        u: &Point2,
    ) -> (Spectrum, Vec3, f32, VisibiliyTester);
}

pub struct VisibiliyTester {}

impl VisibiliyTester {
    pub fn unoccluded(&self, scene: &Scene) -> bool {
        true
    }
}
