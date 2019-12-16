use crate::math::*;
use rand::Rng;

pub fn random_in_unit_sphere() -> Vec3 {
    let mut rng = rand::thread_rng();
    loop {
        let p = 2.0
            * vec3(
                rng.gen_range(0.0_f32, 1.0),
                rng.gen_range(0.0_f32, 1.0),
                rng.gen_range(0.0_f32, 1.0),
            );
        if dot(p, p) < 1.0 {
            break p;
        }
    }
}

pub fn random_float() -> f32 {
    let mut rng = rand::thread_rng();
    rng.gen_range(0.0_f32, 1.0)
}
