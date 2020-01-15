use crate::hitable::HitRecord;
use crate::material::{Material, ScatterResult};
use crate::math::*;
use crate::ray::Ray;
use crate::texture::Texture;

#[derive(Clone)]
pub struct DiffuseLight {
    pub emit: Box<dyn Texture>,
}

impl Material for DiffuseLight {
    fn scatter(&self, _ray: &Ray, _rec: &HitRecord) -> Option<ScatterResult> {
        None
    }
    fn emitted(&self, u: f32, v: f32, p: &Vec3) -> Vec3 {
        self.emit.value(u, v, p)
    }
}
