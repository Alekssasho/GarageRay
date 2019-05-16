// TODO: Remove from here
trait Camera {
}
trait Sampler {
}

use crate::core::Scene;
use crate::integrators::Integrator;

pub struct SampleIntegrator {
    sampler: Box<Sampler>,
    camera: Box<Camera>,
}

impl SampleIntegrator {
    fn new(sampler: Box<Sampler>, camera: Box<Camera>) -> SampleIntegrator {
        SampleIntegrator{ sampler, camera }
    }
}

pub trait SampleIntegratorInterface {
    fn preprocess(&self, scene: &Scene, sampler: &Sampler) -> ();
}

impl SampleIntegratorInterface for SampleIntegrator {
    fn preprocess(&self, scene: &Scene, sampler: &Sampler) -> () {}
}

impl Integrator for SampleIntegrator {
    fn render(&self, scene: &Scene) -> () {
        self.preprocess(scene, &*self.sampler);
        // 26 Page
    }
}