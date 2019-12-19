use crate::math::*;
use crate::random::*;
use crate::ray::*;

pub struct Camera {
    origin: Vec3,
    lower_left_corner: Vec3,
    horizontal: Vec3,
    vertical: Vec3,
    u: Vec3,
    v: Vec3,
    _w: Vec3,
    lens_radius: f32,
    time0: f32,
    time1: f32,
}

fn random_in_unit_disk() -> Vec3 {
    loop {
        let p = 2.0 * vec3(random_float(), random_float(), 0.0) - vec3(1.0, 1.0, 0.0);
        if dot(p, p) < 1.0 {
            break p;
        }
    }
}

impl Camera {
    pub fn new(
        look_from: Vec3,
        look_at: Vec3,
        up: Vec3,
        vertical_fov: f32,
        aspect: f32,
        aperture: f32,
        focus_dist: f32,
        time0: f32,
        time1: f32,
    ) -> Self {
        let lens_radius = aperture / 2.0;
        let theta = vertical_fov * std::f32::consts::PI / 180.0;
        let half_height = (theta / 2.0).tan();
        let half_width = aspect * half_height;
        let w = (look_from - look_at).normalize();
        let u = up.cross(w).normalize();
        let v = w.cross(u);
        Camera {
            origin: look_from,
            lower_left_corner: look_from
                - half_width * focus_dist * u
                - half_height * focus_dist * v
                - focus_dist * w,
            horizontal: 2.0 * half_width * focus_dist * u,
            vertical: 2.0 * half_height * focus_dist * v,
            u,
            v,
            _w: w,
            lens_radius,
            time0,
            time1
        }
    }

    pub fn get_ray(&self, s: f32, t: f32) -> Ray {
        let rd = self.lens_radius * random_in_unit_disk();
        let offset = self.u * rd.x + self.v * rd.y;
        let time = self.time0 + random_float() * (self.time1 - self.time0);
        Ray {
            origin: self.origin + offset,
            direction: self.lower_left_corner + s * self.horizontal + t * self.vertical
                - self.origin
                - offset,
            time,
        }
    }
}
