// Matrix type is column major in cgmath, but row major in book
// Most likely cgmath is implemented with right hand rule, but book is left handed

//pub type Vec2 = cgmath::Vector2<f32>;
pub type Vec2i = cgmath::Vector2<i32>;
pub type Vec3 = cgmath::Vector3<f32>;

pub type Point2 = cgmath::Point2<f32>;
pub type Point2i = cgmath::Point2<i32>;
pub type Point3 = cgmath::Point3<f32>;

pub type Normal3f = cgmath::Vector3<f32>;

pub use cgmath::dot;
pub use cgmath::vec3;
pub use cgmath::EuclideanSpace;

use cgmath::*;
use cgmath::Transform as Tr;
use std::ops::Index;

pub use num_traits::identities::Zero; // needed for vector is_zero method
pub use cgmath::InnerSpace; // needed for normalize

use crate::ray::Ray;
use crate::core::{ Interaction, SurfaceInteraction, Shading };

pub fn min<T: BaseNum>(lhs: T, rhs: T) -> T {
    match lhs.partial_cmp(&rhs) {
        Some(std::cmp::Ordering::Less) => lhs,
        Some(std::cmp::Ordering::Equal) => lhs,
        Some(std::cmp::Ordering::Greater) => rhs,
        None => panic!(),
    }
}

pub fn max<T: BaseNum>(lhs: T, rhs: T) -> T {
    match lhs.partial_cmp(&rhs) {
        Some(std::cmp::Ordering::Less) => rhs,
        Some(std::cmp::Ordering::Equal) => rhs,
        Some(std::cmp::Ordering::Greater) => lhs,
        None => panic!(),
    }
}

pub fn lerp<T: BaseNum>(t: T, lhs: T, rhs: T) -> T {
    (T::from(1.0).unwrap() - t) * lhs + t * rhs
}

// Bounds 2d implementation
pub struct Bounds2D<T> {
    pub min: cgmath::Point2<T>,
    pub max: cgmath::Point2<T>,
}

impl<T: BaseNum> Bounds2D<T> {
    pub fn diagonal(&self) -> Vector2<T> {
        self.max - self.min
    }

    pub fn from_point(p: cgmath::Point2<T>) -> Bounds2D<T> {
        Bounds2D {
            min: p,
            max: p,
        }
    }

    pub fn from_two_points(p1: cgmath::Point2<T>, p2: cgmath::Point2<T>) -> Bounds2D<T> {
        Bounds2D {
            min: cgmath::Point2::new(min(p1.x, p2.x), min(p1.y, p2.y)),
            max: cgmath::Point2::new(max(p1.x, p2.x), max(p1.y, p2.y)),
        }
    }

    pub fn maximum_extent(&self) -> i32 {
        let d = self.diagonal();
        if d.x > d.y {
            0
        } else {
            1
        }
    }

    pub fn lerp(&self, t: &cgmath::Point2<T>) -> cgmath::Point2<T> {
        cgmath::Point2::new(
            lerp(t.x, self.min.x, self.max.x),
            lerp(t.y, self.min.y, self.max.y),
        )
    }

    pub fn offset(&self, p: &cgmath::Point2<T>) -> Vector2<T> {
        let mut o = p - self.min;
        if self.max.x > self.min.x {
            o.x /= self.max.x - self.min.x;
        }
        if self.max.y > self.min.y {
            o.y /= self.max.y - self.min.y;
        }
        o
    }
}

impl<T: BaseFloat> Bounds2D<T> {
    pub fn bounding_sphere(&self) -> (cgmath::Point2<T>, T) {
        let center = cgmath::Point2::<T>::from_vec((self.min.to_vec() + self.max.to_vec()) / T::from(2).unwrap());
        let radius = if inside_2d(&center, self) {
                        center.distance(self.max)
                    } else {
                        T::zero()
                    };
        (center, radius)
    }
}

impl<T> Index<i32> for Bounds2D<T> {
    type Output = cgmath::Point2<T>;

    fn index(&self, index: i32) -> &Self::Output {
        match index {
            0 => &self.min,
            1 => &self.max,
            _ => panic!(),
        }
    }
}

// Possible IndexMut for the same as well ?

impl<T: Bounded> Default for Bounds2D<T> {
    fn default() -> Bounds2D<T> {
        Bounds2D {
            min: cgmath::Point2::new(T::max_value(), T::max_value()),
            max: cgmath::Point2::new(T::min_value(), T::min_value()),
        }
    }
}

// Iterator for integer points inside bounds
pub type Bounds2Di = Bounds2D<i32>;

// Bounds freeroam functions
pub fn union_2d_with_point<T: BaseNum>(b: &Bounds2D<T>, p: &Vector2<T>) -> Bounds2D<T> {
    Bounds2D {
        min: cgmath::Point2::new(min(b.min.x, p.x), min(b.min.y, p.y)),
        max: cgmath::Point2::new(max(b.max.x, p.x), max(b.max.y, p.y))
    }
}

pub fn union_bounds_2d<T: BaseNum>(b1: &Bounds2D<T>, b2: &Bounds2D<T>) -> Bounds2D<T> {
    Bounds2D {
        min: cgmath::Point2::new(min(b1.min.x, b2.min.x), min(b1.min.y, b2.max.x)),
        max: cgmath::Point2::new(max(b1.max.x, b2.max.x), max(b1.max.y, b2.max.y))
    }
}

pub fn intersect_2d<T: BaseNum>(b1: &Bounds2D<T>, b2: &Bounds2D<T>) -> Bounds2D<T> {
    Bounds2D {
        min: cgmath::Point2::new(max(b1.min.x, b2.min.x), max(b1.min.y, b2.max.x)),
        max: cgmath::Point2::new(min(b1.max.x, b2.max.x), min(b1.max.y, b2.max.y))
    }
}

pub fn overlaps_2d<T: BaseNum>(b1: &Bounds2D<T>, b2: &Bounds2D<T>) -> bool {
    let x = (b1.max.x >= b2.min.x) && (b1.min.x <= b2.max.x);
    let y = (b1.max.y >= b2.min.y) && (b1.min.y <= b2.max.y);
    x && y
}

pub fn inside_2d<T: BaseNum>(p: &cgmath::Point2<T>, b: &Bounds2D<T>) -> bool {
    p.x >= b.min.x && p.x <= b.max.x &&
    p.y >= b.min.y && p.y <= b.max.y
}

pub fn inside_exclusive_2d<T: BaseNum>(p: &cgmath::Point2<T>, b: &Bounds2D<T>) -> bool {
    p.x >= b.min.x && p.x < b.max.x &&
    p.y >= b.min.y && p.y < b.max.y
}

pub fn expand_2d<T: BaseNum, U: BaseNum>(b: &Bounds2D<T>, delta: U) -> Bounds2D<T> {
    let delta_t = T::from(delta).unwrap();
    Bounds2D {
        min: b.min - vec2(delta_t, delta_t),
        max: b.max + vec2(delta_t, delta_t),
    }
}

// Bounds 3d implementation
pub struct Bounds3D<T> {
    pub min: cgmath::Point3<T>,
    pub max: cgmath::Point3<T>,
}

impl<T: BaseNum> Bounds3D<T> {
    pub fn diagonal(&self) -> Vector3<T> {
        self.max - self.min
    }

    pub fn from_point(p: cgmath::Point3<T>) -> Bounds3D<T> {
        Bounds3D {
            min: p,
            max: p,
        }
    }

    pub fn from_two_points(p1: cgmath::Point3<T>, p2: cgmath::Point3<T>) -> Bounds3D<T> {
        Bounds3D {
            min: cgmath::Point3::new(min(p1.x, p2.x), min(p1.y, p2.y), min(p1.z, p2.z)),
            max: cgmath::Point3::new(max(p1.x, p2.x), max(p1.y, p2.y), max(p1.z, p2.z)),
        }
    }

    pub fn corner(&self, corner: i32) -> cgmath::Point3<T> {
        cgmath::Point3::new(
            self[corner & 1].x,
            self[if (corner & 2) != 0 { 1 } else { 0 }].y,
            self[if (corner & 4) != 0 { 1 } else { 0 }].z,
        )
    }

    pub fn surface_area(&self) -> T {
        let d = self.diagonal();
        T::from(2).unwrap() * (d.x * d.y + d.x * d.z + d.y * d.z)
    }

    pub fn volume(&self) -> T {
        let d = self.diagonal();
        d.x * d.y * d.z
    }

    pub fn maximum_extent(&self) -> i32 {
        let d = self.diagonal();
        if d.x > d.y {
            0
        } else {
            1
        }
    }

    pub fn lerp(&self, t: &cgmath::Point3<T>) -> cgmath::Point3<T> {
        cgmath::Point3::new(
            lerp(t.x, self.min.x, self.max.x),
            lerp(t.y, self.min.y, self.max.y),
            lerp(t.z, self.min.z, self.max.z),
        )
    }

    pub fn offset(&self, p: &cgmath::Point3<T>) -> Vector3<T> {
        let mut o = p - self.min;
        if self.max.x > self.min.x {
            o.x /= self.max.x - self.min.x;
        }
        if self.max.y > self.min.y {
            o.y /= self.max.y - self.min.y;
        }
        o
    }
}

impl<T: BaseFloat> Bounds3D<T> {
    pub fn bounding_sphere(&self) -> (cgmath::Point3<T>, T) {
        let center = cgmath::Point3::<T>::from_vec((self.min.to_vec() + self.max.to_vec()) / T::from(2).unwrap());
        let radius = if inside_3d(&center, self) {
                        center.distance(self.max)
                    } else {
                        T::zero()
                    };
        (center, radius)
    }
}

impl<T> Index<i32> for Bounds3D<T> {
    type Output = cgmath::Point3<T>;

    fn index(&self, index: i32) -> &Self::Output {
        match index {
            0 => &self.min,
            1 => &self.max,
            _ => panic!(),
        }
    }
}

// Possible IndexMut for the same as well ?

impl<T: Bounded> Default for Bounds3D<T> {
    fn default() -> Bounds3D<T> {
        Bounds3D {
            min: cgmath::Point3::new(T::max_value(), T::max_value(), T::max_value()),
            max: cgmath::Point3::new(T::min_value(), T::min_value(), T::min_value()),
        }
    }
}

// Iterator for integer points inside bounds
pub type Bounds3Di = Bounds3D<i32>;
pub type Bounds3Df = Bounds3D<f32>;

// Bounds 3d freeroam functions
pub fn union_3d_with_point<T: BaseNum>(b: &Bounds3D<T>, p: &cgmath::Point3<T>) -> Bounds3D<T> {
    Bounds3D {
        min: cgmath::Point3::new(min(b.min.x, p.x), min(b.min.y, p.y), min(b.min.z, p.z)),
        max: cgmath::Point3::new(max(b.max.x, p.x), max(b.max.y, p.y), max(b.min.z, p.z))
    }
}

pub fn union_bounds_3d<T: BaseNum>(b1: &Bounds3D<T>, b2: &Bounds3D<T>) -> Bounds3D<T> {
    Bounds3D {
        min: cgmath::Point3::new(min(b1.min.x, b2.min.x), min(b1.min.y, b2.max.x), min(b1.min.z, b2.max.z)),
        max: cgmath::Point3::new(max(b1.max.x, b2.max.x), max(b1.max.y, b2.max.y), max(b1.min.z, b2.max.z))
    }
}

pub fn intersect_3d<T: BaseNum>(b1: &Bounds3D<T>, b2: &Bounds3D<T>) -> Bounds3D<T> {
    Bounds3D {
        min: cgmath::Point3::new(max(b1.min.x, b2.min.x), max(b1.min.y, b2.max.x), max(b1.min.z, b2.max.z)),
        max: cgmath::Point3::new(min(b1.max.x, b2.max.x), min(b1.max.y, b2.max.y), min(b1.min.z, b2.max.z))
    }
}

pub fn overlaps_3d<T: BaseNum>(b1: &Bounds3D<T>, b2: &Bounds3D<T>) -> bool {
    let x = (b1.max.x >= b2.min.x) && (b1.min.x <= b2.max.x);
    let y = (b1.max.y >= b2.min.y) && (b1.min.y <= b2.max.y);
    let z = (b1.max.z >= b2.min.z) && (b1.min.z <= b2.max.z);
    x && y && z
}

pub fn inside_3d<T: BaseNum>(p: &cgmath::Point3<T>, b: &Bounds3D<T>) -> bool {
    p.x >= b.min.x && p.x <= b.max.x &&
    p.y >= b.min.y && p.y <= b.max.y &&
    p.z >= b.min.z && p.z <= b.max.z
}

pub fn inside_exclusive_3d<T: BaseNum>(p: &cgmath::Point3<T>, b: &Bounds3D<T>) -> bool {
    p.x >= b.min.x && p.x < b.max.x &&
    p.y >= b.min.y && p.y < b.max.y &&
    p.z >= b.min.z && p.z < b.max.z
}

pub fn expand_3d<T: BaseNum, U: BaseNum>(b: &Bounds3D<T>, delta: U) -> Bounds3D<T> {
    let delta_t = T::from(delta).unwrap();
    Bounds3D {
        min: b.min - vec3(delta_t, delta_t, delta_t),
        max: b.max + vec3(delta_t, delta_t, delta_t),
    }
}

// Freeroam functions
pub fn coordinate_system(v1: Vec3) -> (Vec3, Vec3, Vec3) {
    let v2;
    if v1.x.abs() > v1.y.abs() {
        v2 = vec3(-v1.z, 0.0, v1.x).normalize();
    } else {
        v2 = vec3(0.0, v1.z, -v1.y).normalize();
    }

    (v1, v2, v1.cross(v2))
}

pub fn face_forward(n: Normal3f, v: Vec3) -> Normal3f {
    if dot(n, v) < 0.0 {
        -n
    } else {
        n
    }
}

// Transformations
pub struct Transform {
    m: cgmath::Matrix4<f32>,
    m_inv: cgmath::Matrix4<f32>,
}

impl Default for Transform {
    fn default() -> Self {
        Transform{
            m: Matrix4::identity(),
            m_inv: Matrix4::identity(),
        }
    }
}

impl From<Matrix4<f32>> for Transform {
    fn from(mat: Matrix4<f32>) -> Transform {
        Transform{
            m: mat,
            m_inv: mat.invert().unwrap(),
        }
    }
}

impl Transform {
    pub fn new(m: Matrix4<f32>, m_inv: Matrix4<f32>) -> Transform {
        Transform { m, m_inv }
    }

    pub fn inverse(&self) -> Transform {
        Transform{ m: self.m_inv, m_inv: self.m }
    }

    pub fn transpose(&self) -> Transform {
        Transform{ m: self.m.transpose(), m_inv: self.m_inv.transpose() }
    }

    pub fn has_scale() -> bool {
        // TODO: implement me properly // page 88
        false
    }

    pub fn transform_point(&self, p: Point3) -> Point3 {
        self.m.transform_point(p)
    }

    pub fn transform_vec(&self, v: Vec3) -> Vec3 {
        self.m.transform_vector(v)
    }

    pub fn transform_normal(&self, n: Vec3) -> Vec3 {
        vec3(
            self.m_inv.x[0] * n.x + self.m_inv.x[1] * n.y + self.m_inv.x[2] * n.z,
            self.m_inv.y[0] * n.x + self.m_inv.y[1] * n.y + self.m_inv.y[2] * n.z,
            self.m_inv.z[0] * n.x + self.m_inv.z[1] * n.y + self.m_inv.z[2] * n.z
        )
    }

    pub fn transform_ray(&self, r: Ray) -> Ray {
        let o = self.transform_point(r.o);
        let d = self.transform_vec(r.d);
        // Offset ray origin to edge of error bounds
        Ray{o, d, ..r}
    }

    pub fn transform_bounds(&self, b: &Bounds3Df) -> Bounds3Df {
        let ret = Bounds3Df::from_point(self.transform_point(b.min));
        let ret = union_3d_with_point(&ret, &self.transform_point(Point3::new(b.max.x, b.min.y, b.min.z)));
        let ret = union_3d_with_point(&ret, &self.transform_point(Point3::new(b.min.x, b.max.y, b.min.z)));
        let ret = union_3d_with_point(&ret, &self.transform_point(Point3::new(b.min.x, b.min.y, b.max.z)));
        let ret = union_3d_with_point(&ret, &self.transform_point(Point3::new(b.min.x, b.max.y, b.max.z)));
        let ret = union_3d_with_point(&ret, &self.transform_point(Point3::new(b.max.x, b.max.y, b.min.z)));
        let ret = union_3d_with_point(&ret, &self.transform_point(Point3::new(b.max.x, b.min.y, b.max.z)));
        let ret = union_3d_with_point(&ret, &self.transform_point(b.max));
        ret
    }

    pub fn transform_surface_interaction(&self, si: SurfaceInteraction) -> SurfaceInteraction {
        // Transform p and pError
        let mut ret = SurfaceInteraction{
            interaction: Interaction {
                p: si.interaction.p,
                n: self.transform_normal(si.interaction.n).normalize(),
                p_error: si.interaction.p_error,
                wo: self.transform_vec(si.interaction.wo).normalize(),
                ..si.interaction
            },
            dpdu: self.transform_vec(si.dpdu),
            dpdv: self.transform_vec(si.dpdv),
            dndu: self.transform_vec(si.dndu),
            dndv: self.transform_vec(si.dndv),
            shading: Shading{
                n: self.transform_normal(si.shading.n).normalize(),
                dpdu: self.transform_vec(si.shading.dpdu),
                dpdv: self.transform_vec(si.shading.dpdv),
                dndu: self.transform_vec(si.shading.dndu),
                dndv: self.transform_vec(si.shading.dndv),
            },
            ..si
        };
        ret.shading.n = face_forward(ret.shading.n, ret.interaction.n);
        ret
    }

    pub fn swaps_handedness(&self) -> bool {
        let det = self.m.determinant();
        det < 0.0
    }
}

impl std::ops::Mul for Transform {
    type Output = Self;
    fn mul(self, rhs: Self) -> Self {
        Transform{
            m: self.m * rhs.m,
            m_inv: rhs.m_inv * self.m_inv
        }
    }
}

pub fn translate(delta: Vec3) -> Transform {
    let m = Matrix4::from_translation(delta);
    let m_inv = Matrix4::from_translation(-delta);
    Transform{ m, m_inv }
}

pub fn scale(x: f32, y: f32, z: f32) -> Transform {
    let m = Matrix4::from_nonuniform_scale(x, y, z);
    let m_inv = Matrix4::from_nonuniform_scale(1.0 / x, 1.0 / y, 1.0 / z);
    Transform{ m, m_inv }
}

pub fn rotate_x(theta: f32) -> Transform {
    let m = Matrix4::from_angle_x(Deg(theta));
    let m_inv = m.transpose();
    Transform{ m, m_inv }
}

pub fn rotate_y(theta: f32) -> Transform {
    let m = Matrix4::from_angle_y(Deg(theta));
    let m_inv = m.transpose();
    Transform{ m, m_inv }
}

pub fn rotate_z(theta: f32) -> Transform {
    let m = Matrix4::from_angle_z(Deg(theta));
    let m_inv = m.transpose();
    Transform{ m, m_inv }
}

pub fn rotate_axis(theta: f32, axis: Vec3) -> Transform {
    let m = Matrix4::from_axis_angle(axis, Deg(theta));
    let m_inv = m.transpose();
    Transform{ m, m_inv }
}

pub fn look_at(pos: Point3, look: Point3, up: Vec3) -> Transform {
    let m = Matrix4::look_at(pos, look, up);
    let m_inv = m.invert().unwrap();
    Transform{ m, m_inv }
}