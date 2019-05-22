use crate::math::*;
use crate::spectrum::Spectrum;
use bitmask::bitmask;

bitmask! {
    pub mask BxDFType: u32 where flags BSDF_TYPES {
        BSDF_REFLECTION = 1 << 0,
        BSDF_SPECULAR = 1 << 1,
        BSDF_TRANSMISSION = 1 << 2,
    }
}

pub struct BSDF {}

impl BSDF {
    pub fn f(&self, wo: &Vec3, wi: &Vec3, flags: BxDFType) -> Spectrum {
        Spectrum::new()
    }

    pub fn sample_f(&self, wo: &Vec3, u: &Point2, flags: BxDFType) -> (Spectrum, Vec3, f32) {
        (Spectrum::new(), Vec3::new(0.0, 0.0, 0.0), 0.0)
    }
}
