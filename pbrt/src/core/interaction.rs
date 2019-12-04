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

pub struct SurfaceInteraction<'a> {
    pub interaction: Interaction,
    pub uv: Point2,
    pub dpdu: Vec3,
    pub dpdv: Vec3,
    pub dndu: Normal3f,
    pub dndv: Normal3f,
    pub shape: &'a Shape,
    pub shading: Shading,
    pub bsdf: BSDF,
}

impl<'a> SurfaceInteraction<'a> {
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
        shape: &Shape,
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
