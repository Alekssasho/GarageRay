use super::sample_integrator::*;
use crate::cameras::Camera;
use crate::core::reflection::BxDFType;
use crate::core::*;
use crate::integrators::Integrator;
use crate::math::*;
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
        let mut L = Spectrum::new();
        let maybe_isect = scene.intersect(&ray.ray);
        if maybe_isect.is_none() {
            for light in scene.lights.iter() {
                L += light.light_emission(ray);
            }
            return L;
        }
        let isect = maybe_isect.unwrap();
        let n = isect.shading.n;
        let wo = isect.interaction.wo;
        isect.compute_scattering_functions(ray);
        L += isect.light_emission(&wo);

        for light in scene.lights.iter() {
            let (Li, wi, pdf, visibiliy) = light.sample_light_incoming(&isect, &sampler.get_2d());
            if Li.is_black() || pdf == 0.0 {
                continue;
            }

            let f = isect.bsdf.f(&wo, &wi, BxDFType::all());
            if !f.is_black() && visibiliy.unoccluded(scene) {
                L += f * Li * dot(wi, n).abs() / pdf;
            }
        }

        if depth + 1 < self.max_depth {
            L += sample_integrator.specular_reflect(ray, &isect, scene, sampler, depth);
            L += sample_integrator.specular_transmit(ray, &isect, scene, sampler, depth);
        }

        L
    }
}
