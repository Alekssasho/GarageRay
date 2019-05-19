use crate::math::{Vec2i, Point2i, Bounds2Di};
use crate::core::Scene;
use crate::integrators::Integrator;
use crate::samplers::Sampler;
use crate::cameras::Camera;
use crate::core::parallel_for_2d;

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

        let sample_bounds = self.camera.film().get_sample_bounds();
        let sample_extent = sample_bounds.diagonal();
        let tile_size = 16;
        let num_tiles = Vec2i::new(
            (sample_extent.x + tile_size - 1) / tile_size,
            (sample_extent.y + tile_size - 1) / tile_size
        );

        parallel_for_2d(&|tile| {
            // Memory Arena
            let seed = tile.y * num_tiles.x + tile.x;
            // TODO: this is weird
            let tile_sampler = self.sampler.clone();
            let x0 = sample_bounds.min.x + tile.x * tile_size;
            let x1 = std::cmp::min(x0 + tile_size, sample_bounds.max.x);
            let y0 = sample_bounds.min.y + tile.y * tile_size;
            let y1 = std::cmp::min(y0 + tile_size, sample_bounds.max.y);
            let tile_bounds = Bounds2Di{ min: Vec2i::new(x0, y0), max: Vec2i::new(x1, y1) };

            let film_tile = self.camera.film().get_film_tile(tile_bounds);
            for pixel in film_tile {
                for sample in tile_sampler.start_pixel(pixel) {
                    // Page 30
                }
            }

        }, Point2i::from((num_tiles.x, num_tiles.y)));
        // Save final image
    }
}