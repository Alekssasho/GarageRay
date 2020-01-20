use crate::pdf::PDF;
use crate::math::*;
use crate::hitable::Hitable;

pub struct HitablePDF<'a> {
    pub hitable: &'a dyn Hitable,
    pub o: Vec3,
}

impl PDF for HitablePDF<'_> {
    fn value(&self, direction: &Vec3) -> f32 {
        self.hitable.pdf_value(&self.o, direction)
    }

    fn generate(&self) -> Vec3 {
        self.hitable.random(&self.o)
    }
}