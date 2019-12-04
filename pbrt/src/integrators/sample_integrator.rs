use crate::cameras::Camera;
use crate::core::reflection::BSDF_TYPES::*;
use crate::core::*;
use crate::integrators::Integrator;
use crate::math::*;
use crate::ray::*;
use crate::samplers::Sampler;
use crate::spectrum::Spectrum;

pub struct SampleIntegrator {
    sampler: Box<Sampler>,
    camera: Box<Camera>,
    implementor: Box<SampleIntegratorInterface>,
}

impl SampleIntegrator {
    fn new(
        sampler: Box<Sampler>,
        camera: Box<Camera>,
        implementor: Box<SampleIntegratorInterface>,
    ) -> SampleIntegrator {
        SampleIntegrator {
            sampler,
            camera,
            implementor,
        }
    }

    pub fn specular_reflect(
        &self,
        ray: &RayDifferential,
        isect: &SurfaceInteraction,
        scene: &Scene,
        sampler: &mut Sampler,
        depth: i32,
    ) -> Spectrum {
        let wo = isect.interaction.wo;
        let (f, wi, pdf) =
            isect
                .bsdf
                .sample_f(&wo, &sampler.get_2d(), BSDF_REFLECTION | BSDF_SPECULAR);
        let ns = isect.shading.n;
        if pdf > 0.0 && !f.is_black() && (dot(wi, ns).abs() != 0.0) {
            // Compute ray differential rd for specular
            // ray should be from upper line
            f * self
                .implementor
                .light_incoming(self, ray, scene, sampler, depth + 1)
                * dot(wi, ns).abs()
                / pdf
        } else {
            Spectrum::new()
        }
    }

    pub fn specular_transmit(
        &self,
        ray: &RayDifferential,
        isect: &SurfaceInteraction,
        scene: &Scene,
        sampler: &mut Sampler,
        depth: i32,
    ) -> Spectrum {
        let wo = isect.interaction.wo;
        let (f, wi, pdf) =
            isect
                .bsdf
                .sample_f(&wo, &sampler.get_2d(), BSDF_TRANSMISSION | BSDF_SPECULAR);
        let ns = isect.shading.n;
        if pdf > 0.0 && !f.is_black() && (dot(wi, ns).abs() != 0.0) {
            // Compute ray differential rd for specular
            // ray should be from upper line
            f * self
                .implementor
                .light_incoming(self, ray, scene, sampler, depth + 1)
                * dot(wi, ns).abs()
                / pdf
        } else {
            Spectrum::new()
        }
    }
}

pub trait SampleIntegratorInterface {
    fn preprocess(&self, scene: &Scene, sampler: &Sampler);
    fn light_incoming(
        &self,
        sample_integrator: &SampleIntegrator,
        ray: &RayDifferential,
        scene: &Scene,
        sampler: &mut Sampler,
        // Memory Arena
        depth: i32,
    ) -> Spectrum;
}

impl Integrator for SampleIntegrator {
    fn render(&self, scene: &Scene) -> () {
        self.implementor.preprocess(scene, &*self.sampler);

        let sample_bounds = self.camera.film().get_sample_bounds();
        let sample_extent = sample_bounds.diagonal();
        let tile_size = 16;
        let num_tiles = Vec2i::new(
            (sample_extent.x + tile_size - 1) / tile_size,
            (sample_extent.y + tile_size - 1) / tile_size,
        );

        parallel_for_2d(
            &|tile| {
                // Memory Arena
                let seed = tile.y * num_tiles.x + tile.x;
                // TODO: this is weird
                let mut tile_sampler = self.sampler.clone();
                let x0 = sample_bounds.min.x + tile.x * tile_size;
                let x1 = std::cmp::min(x0 + tile_size, sample_bounds.max.x);
                let y0 = sample_bounds.min.y + tile.y * tile_size;
                let y1 = std::cmp::min(y0 + tile_size, sample_bounds.max.y);
                let tile_bounds =
                    Bounds2Di::from_two_points(Point2i::new(x0, y0), Point2i::new(x1, y1));

                let film_tile = self.camera.film().get_film_tile(tile_bounds);
                for pixel in film_tile.iter() {
                    for sample in tile_sampler.start_pixel(pixel) {
                        let camera_sample = tile_sampler.get_camera_sample(pixel);

                        let (mut ray, ray_weight) =
                            self.camera.generate_ray_differential(camera_sample);
                        ray.scale_differentials(
                            1.0 / (tile_sampler.get_samples_per_pixel() as f32).sqrt(),
                        );

                        let mut light = Spectrum::new();
                        if ray_weight > 0.0 {
                            light = self.implementor.light_incoming(
                                self,
                                &ray,
                                scene,
                                &mut *tile_sampler,
                                0,
                            );
                        }
                        // Issue warning if unexpected
                        // TODO: this should be camera_sample.pfilm
                        film_tile.add_sample(&self.camera.film(), light, ray_weight);
                    }
                }

                self.camera.film().merge_film_tile(film_tile);
            },
            Point2i::from_vec(num_tiles),
        );

        self.camera.film().write_image();
    }
}
