use crate::math::Vec3;

mod cosine;
mod hitable_pdf;
mod mixture;

pub use cosine::Cosine;
pub use hitable_pdf::HitablePDF;
pub use mixture::Mixture;

pub trait PDF {
    fn value(&self, direction: &Vec3) -> f32;
    fn generate(&self) -> Vec3;
}