pub struct Ray {}

pub struct RayDifferential {
    pub ray: Ray,
}

impl RayDifferential {
    // TODO: maybe this is better to not modify self but return new self
    pub fn scale_differentials(&mut self, scalar: f32) -> () {}
}
