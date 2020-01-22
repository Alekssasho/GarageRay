use crate::math::*;
use crate::onb::ONB;
use crate::pdf::PDF;
use crate::random::random_cosine_direction;

pub struct Cosine {
    uvw: ONB,
}

impl Cosine {
    pub fn new(w: &Vec3) -> Self {
        Cosine {
            uvw: ONB::build_from_w(w),
        }
    }
}

impl PDF for Cosine {
    fn value(&self, direction: &Vec3) -> f32 {
        let cosine = dot(direction.normalize(), self.uvw.w);
        if cosine > 0.0 {
            cosine / std::f32::consts::PI
        } else {
            0.0
        }
    }

    fn generate(&self) -> Vec3 {
        self.uvw.local_vec(&random_cosine_direction())
    }
}
