use crate::core::Scene;

mod sample_integrator;
mod whitted_integrator;

pub use sample_integrator::SampleIntegrator;
pub use whitted_integrator::WhittedIntegrator;

pub trait Integrator {
    fn render(&self, scene: &Scene) -> ();
}
