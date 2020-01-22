use crate::math::*;

pub struct ONB {
    pub u: Vec3,
    pub v: Vec3,
    pub w: Vec3,
}

impl ONB {
    pub fn build_from_w(n: &Vec3) -> Self {
        let w = n.normalize();
        let a = if w.x.abs() > 0.9 {
            vec3(0.0, 1.0, 0.0)
        } else {
            vec3(1.0, 0.0, 0.0)
        };
        let v = w.cross(a).normalize();
        let u = w.cross(v);
        ONB { u, v, w }
    }

    // pub fn local_points(&self, a: f32, b: f32, c: f32) -> Vec3 {
    //     a * self.u + b * self.v + c * self.w
    // }

    pub fn local_vec(&self, a: &Vec3) -> Vec3 {
        a.x * self.u + a.y * self.v + a.z * self.w
    }
}
