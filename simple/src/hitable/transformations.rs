use crate::hitable::*;
use crate::math::*;
use crate::ray::Ray;

pub struct Translate {
    pub offset: Vec3,
    pub hitable: Box<dyn Hitable>,
}

pub struct RotateY {
    sin_theta: f32,
    cos_theta: f32,
    bbox: Option<AABB>,
    hitable: Box<dyn Hitable>,
}

impl Hitable for Translate {
    fn hit(&self, ray: &Ray, t_min: f32, t_max: f32) -> Option<HitRecord> {
        let moved_ray = Ray {
            origin: ray.origin - self.offset,
            ..*ray
        };
        if let Some(rec) = self.hitable.hit(&moved_ray, t_min, t_max) {
            Some(HitRecord {
                p: rec.p + self.offset,
                ..rec
            })
        } else {
            None
        }
    }

    fn bounding_box(&self, t0: f32, t1: f32) -> Option<AABB> {
        if let Some(aabb) = self.hitable.bounding_box(t0, t1) {
            Some(AABB {
                min: aabb.min + self.offset,
                max: aabb.max + self.offset,
            })
        } else {
            None
        }
    }
}

impl RotateY {
    pub fn new(hitable: Box<dyn Hitable>, angle: f32) -> Self {
        let radians = (std::f32::consts::PI / 180.0) * angle;
        let cos_theta = radians.cos();
        let sin_theta = radians.sin();
        if let Some(bbox) = hitable.bounding_box(0.0, 1.0) {
            let mut min = vec3(std::f32::MAX, std::f32::MAX, std::f32::MAX);
            let mut max = vec3(-std::f32::MAX, -std::f32::MAX, -std::f32::MAX);

            for i in 0..2 {
                for j in 0..2 {
                    for k in 0..2 {
                        let x = i as f32 * bbox.max.x + (1 - i) as f32 * bbox.min.x;
                        let y = j as f32 * bbox.max.y + (1 - j) as f32 * bbox.min.y;
                        let z = k as f32 * bbox.max.z + (1 - k) as f32 * bbox.min.z;
                        let new_x = cos_theta * x + sin_theta * z;
                        let new_z = -sin_theta * x + cos_theta * z;
                        let tester = vec3(new_x, y, new_z);
                        for c in 0..3_usize {
                            if tester[c] > max[c] {
                                max[c] = tester[c];
                            }
                            if tester[c] < min[c] {
                                min[c] = tester[c];
                            }
                        }
                    }
                }
            }
            RotateY {
                sin_theta,
                cos_theta,
                bbox: Some(AABB { min, max }),
                hitable,
            }
        } else {
            RotateY {
                sin_theta,
                cos_theta,
                bbox: None,
                hitable,
            }
        }
    }
}

impl Hitable for RotateY {
    fn hit(&self, ray: &Ray, t_min: f32, t_max: f32) -> Option<HitRecord> {
        let origin = vec3(
            self.cos_theta * ray.origin.x - self.sin_theta * ray.origin.z,
            ray.origin.y,
            self.sin_theta * ray.origin.x + self.cos_theta * ray.origin.z,
        );
        let direction = vec3(
            self.cos_theta * ray.direction.x - self.sin_theta * ray.direction.z,
            ray.direction.y,
            self.sin_theta * ray.direction.x + self.cos_theta * ray.direction.z,
        );
        let rotated_ray = Ray {
            origin,
            direction,
            ..*ray
        };
        if let Some(rec) = self.hitable.hit(&rotated_ray, t_min, t_max) {
            Some(HitRecord {
                p: vec3(
                    self.cos_theta * rec.p.x + self.sin_theta * rec.p.z,
                    rec.p.y,
                    -self.sin_theta * rec.p.x + self.cos_theta * rec.p.z,
                ),
                normal: vec3(
                    self.cos_theta * rec.normal.x + self.sin_theta * rec.normal.z,
                    rec.normal.y,
                    -self.sin_theta * rec.normal.x + self.cos_theta * rec.normal.z,
                ),
                ..rec
            })
        } else {
            None
        }
    }

    fn bounding_box(&self, _t0: f32, _t1: f32) -> Option<AABB> {
        self.bbox
    }
}
