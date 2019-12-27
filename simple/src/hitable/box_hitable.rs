use crate::hitable::*;
use crate::material::Material;
use crate::math::*;
use crate::ray::Ray;

pub struct BoxHitable {
    list: Vec<Box<dyn Hitable>>,
    p_min: Vec3,
    p_max: Vec3,
}

impl Hitable for BoxHitable {
    fn hit(&self, ray: &Ray, t_min: f32, t_max: f32) -> Option<HitRecord> {
        self.list.hit(ray, t_min, t_max)
    }

    fn bounding_box(&self, _t0: f32, _t1: f32) -> Option<AABB> {
        Some(AABB {
            min: self.p_min,
            max: self.p_max,
        })
    }
}

impl BoxHitable {
    pub fn new(p0: &Vec3, p1: &Vec3, material: Box<dyn Material>) -> Self {
        BoxHitable {
            p_min: *p0,
            p_max: *p1,
            list: vec![
                Box::new(XYRect {
                    x0: p0.x,
                    x1: p1.x,
                    y0: p0.y,
                    y1: p1.y,
                    k: p1.z,
                    material: material.clone(),
                }),
                Box::new(FlipNormals(Box::new(XYRect {
                    x0: p0.x,
                    x1: p1.x,
                    y0: p0.y,
                    y1: p1.y,
                    k: p0.z,
                    material: material.clone(),
                }))),
                Box::new(XZRect {
                    x0: p0.x,
                    x1: p1.x,
                    z0: p0.z,
                    z1: p1.z,
                    k: p1.y,
                    material: material.clone(),
                }),
                Box::new(FlipNormals(Box::new(XZRect {
                    x0: p0.x,
                    x1: p1.x,
                    z0: p0.z,
                    z1: p1.z,
                    k: p0.y,
                    material: material.clone(),
                }))),
                Box::new(YZRect {
                    y0: p0.y,
                    y1: p1.y,
                    z0: p0.z,
                    z1: p1.z,
                    k: p1.x,
                    material: material.clone(),
                }),
                Box::new(FlipNormals(Box::new(YZRect {
                    y0: p0.y,
                    y1: p1.y,
                    z0: p0.z,
                    z1: p1.z,
                    k: p0.x,
                    material: material,
                }))),
            ],
        }
    }
}
