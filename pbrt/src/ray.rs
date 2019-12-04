use crate::core::Medium;
use crate::math::*;
use std::cell::Cell;
use std::rc::Rc;

pub struct Ray<'a> {
    pub o: Point3,
    pub d: Vec3,
    pub t_max: Cell<f32>, // This is mutable in original C++ code so we need Cell to mutate it
    pub time: f32,
    pub medium: Option<&'a Medium>,
}

impl<'a> Ray<'a> {
    pub fn new(o: Point3, d: Vec3) -> Ray<'a> {
        Ray {
            o,
            d,
            t_max: Cell::new(std::f32::INFINITY),
            time: 0.0,
            medium: None,
        }
    }

    pub fn at(&self, t: f32) -> Point3 {
        self.o + self.d * t
    }
}

pub struct RayDifferential<'a> {
    pub ray: Ray<'a>,
    pub hasDifferentials: bool,
    pub rxOrigin: Point3,
    pub ryOrigin: Point3,
    pub rxDirection: Vec3,
    pub ryDirection: Vec3,
}

impl<'a> RayDifferential<'a> {
    // TODO: maybe this is better to not modify self but return new self
    pub fn scale_differentials(&mut self, scalar: f32) -> () {
        self.rxOrigin = self.ray.o + (self.rxOrigin - self.ray.o) * scalar;
        self.ryOrigin = self.ray.o + (self.ryOrigin - self.ray.o) * scalar;
        self.rxDirection = self.ray.d + (self.rxDirection - self.ray.d) * scalar;
        self.ryDirection = self.ray.d + (self.ryDirection - self.ray.d) * scalar;
    }

    pub fn new(o: Point3, d: Vec3) -> RayDifferential<'a> {
        RayDifferential {
            ray: Ray::new(o, d),
            hasDifferentials: false,
            rxOrigin: Point3::new(0.0, 0.0, 0.0),
            ryOrigin: Point3::new(0.0, 0.0, 0.0),
            rxDirection: vec3(0.0, 0.0, 0.0),
            ryDirection: vec3(0.0, 0.0, 0.0),
        }
    }
}

impl<'a> From<Ray<'a>> for RayDifferential<'a> {
    fn from(ray: Ray) -> RayDifferential {
        RayDifferential {
            ray,
            hasDifferentials: false,
            rxOrigin: Point3::new(0.0, 0.0, 0.0),
            ryOrigin: Point3::new(0.0, 0.0, 0.0),
            rxDirection: vec3(0.0, 0.0, 0.0),
            ryDirection: vec3(0.0, 0.0, 0.0),
        }
    }
}
