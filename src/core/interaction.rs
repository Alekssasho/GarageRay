use crate::core::*;
use crate::math::*;
use crate::ray::RayDifferential;
use crate::spectrum::Spectrum;

pub struct Shading {
    pub n: Normal3f,
}

pub struct SurfaceInteraction {
    pub shading: Shading,
    pub wo: Vec3,
    pub bsdf: BSDF,
}

impl SurfaceInteraction {
    pub fn new() -> SurfaceInteraction {
        SurfaceInteraction {
            shading: Shading {
                n: Normal3f::new(0.0, 0.0, 0.0),
            },
            wo: Vec3::new(0.0, 0.0, 0.0),
            bsdf: BSDF {},
        }
    }

    pub fn compute_scattering_functions(&self, ray: &RayDifferential /* Memory Arena */) {}

    pub fn light_emission(&self, w: &Vec3) -> Spectrum {
        Spectrum::new()
    }
}
