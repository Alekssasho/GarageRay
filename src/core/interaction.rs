use crate::core::*;
use crate::math::*;
use crate::ray::RayDifferential;
use crate::shapes::*;
use crate::spectrum::Spectrum;

pub struct Interaction {
    pub p: Point3,
    pub time: f32,
    pub p_error: Vec3,
    pub wo: Vec3,
    pub n: Normal3f,
    pub medium_interface: MediumInterface,
}

impl Interaction {
    pub fn is_surface_interaction(&self) -> bool {
        self.n.is_zero()
    }
}

pub struct Shading {
    pub n: Normal3f,
    pub dpdu: Vec3,
    pub dpdv: Vec3,
    pub dndu: Normal3f,
    pub dndv: Normal3f,
}

pub struct SurfaceInteraction {
    pub interaction: Interaction,
    pub uv: Point2,
    pub dpdu: Vec3,
    pub dpdv: Vec3,
    pub dndu: Normal3f,
    pub dndv: Normal3f,
    pub shape: Box<Shape>, // Possible better to be a reference
    pub shading: Shading,
    pub bsdf: BSDF,
}

impl SurfaceInteraction {
    pub fn new(
        p: Point3,
        p_error: Vec3,
        uv: Point2,
        wo: Vec3,
        dpdu: Vec3,
        dpdv: Vec3,
        dndu: Normal3f,
        dndv: Normal3f,
        time: f32,
        shape: Box<Shape>,
    ) -> SurfaceInteraction {
        let mut n = dpdu.cross(dpdv).normalize();
        // null check add
        if shape.reverse_orientation ^ shape.transform_swaps_handedness {
            n *= -1.0;
        }
        SurfaceInteraction {
            interaction: Interaction {
                p,
                n,
                p_error,
                wo,
                time,
                medium_interface: MediumInterface {},
            },
            uv,
            dpdu,
            dpdv,
            dndu,
            dndv,
            shape,
            shading: Shading {
                n,
                dpdu,
                dpdv,
                dndu,
                dndv,
            },
            bsdf: BSDF {},
        }
    }

    pub fn delete_me_default() -> SurfaceInteraction {
        SurfaceInteraction {
            interaction: Interaction {
                p: Point3::new(0.0, 0.0, 0.0),
                n: vec3(0.0, 0.0, 0.0),
                p_error: vec3(0.0, 0.0, 0.0),
                wo: vec3(0.0, 0.0, 0.0),
                time: 0.0,
                medium_interface: MediumInterface {},
            },
            uv: Point2::new(0.0, 0.0),
            dpdu: vec3(0.0, 0.0, 0.0),
            dpdv: vec3(0.0, 0.0, 0.0),
            dndu: vec3(0.0, 0.0, 0.0),
            dndv: vec3(0.0, 0.0, 0.0),
            shape: Box::new(Sphere::unit()),
            shading: Shading {
                n: vec3(0.0, 0.0, 0.0),
                dpdu: vec3(0.0, 0.0, 0.0),
                dpdv: vec3(0.0, 0.0, 0.0),
                dndu: vec3(0.0, 0.0, 0.0),
                dndv: vec3(0.0, 0.0, 0.0),
            },
            bsdf: BSDF {},
        }
    }

    pub fn compute_scattering_functions(&self, ray: &RayDifferential /* Memory Arena */) {}

    pub fn light_emission(&self, w: &Vec3) -> Spectrum {
        Spectrum::new()
    }

    pub fn set_shading_geometry(
        &mut self,
        dpdus: Vec3,
        dpdvs: Vec3,
        dndus: Normal3f,
        dndvs: Normal3f,
        orientation_is_authorative: bool,
    ) -> () {
        self.shading.n = dpdus.cross(dpdvs).normalize();
        if self.shape.reverse_orientation ^ self.shape.transform_swaps_handedness {
            self.shading.n = -self.shading.n;
        }
        if orientation_is_authorative {
            self.interaction.n = face_forward(self.interaction.n, self.shading.n);
        } else {
            self.interaction.n = face_forward(self.shading.n, self.interaction.n);
        }
        self.shading.dpdu = dpdus;
        self.shading.dpdv = dpdvs;
        self.shading.dndu = dndus;
        self.shading.dndv = dndvs;
    }
}
