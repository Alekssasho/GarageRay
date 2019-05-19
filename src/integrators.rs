use crate::core::Scene;

mod sample_integrator;

pub use sample_integrator::SampleIntegrator;

pub trait Integrator {
    fn render(&self, scene: &Scene) -> ();
}