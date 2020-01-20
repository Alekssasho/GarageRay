use crate::pdf::PDF;
use crate::math::*;
use crate::random::random_float;

pub struct Mixture<'a> {
    pub p: [&'a dyn PDF; 2],
}

impl PDF for Mixture<'_> {
    fn value(&self, direction: &Vec3) -> f32 {
        0.5 * self.p[0].value(direction) + 0.5 * self.p[1].value(direction)
    }

    fn generate(&self) -> Vec3 {
        if random_float() < 0.5 {
            self.p[0].generate()
        } else {
            self.p[1].generate()
        }
    }
}