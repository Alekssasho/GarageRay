use crate::math::Vec3;
use crate::math::*;
use crate::random::random_float;
use crate::texture::Texture;

#[derive(Clone)]
pub struct NoiseTexture {
    pub scale: f32,
}

impl Texture for NoiseTexture {
    fn value(&self, _u: f32, _v: f32, p: &Vec3) -> Vec3 {
        //vec3(1.0, 1.0, 1.0) * turb(&(p * self.scale), 7)
        //vec3(1.0, 1.0, 1.0) * 0.5 * (1.0 + turb(&(p * self.scale), 7))
        vec3(1.0, 1.0, 1.0)
            * 0.5
            * (1.0 + (self.scale * p.z + 10.0 * turb(*p * self.scale, 7)).sin())
    }
}

fn turb(mut p: Vec3, depth: i32) -> f32 {
    let mut accum = 0.0;
    let mut weight = 1.0;
    for _ in 0..depth {
        accum += weight * perlin_noise(&p);
        weight *= 0.5;
        p *= 2.0;
    }
    accum.abs()
}

fn trilinear_intepolation(c: &[[[Vec3; 2]; 2]; 2], u: f32, v: f32, w: f32) -> f32 {
    let uu = u * u * (3.0 - 2.0 * u);
    let vv = v * v * (3.0 - 2.0 * v);
    let ww = w * w * (3.0 - 2.0 * w);
    let mut accum = 0.0;
    for i in 0..2 {
        for j in 0..2 {
            for k in 0..2 {
                let weight_v = vec3(u - i as f32, v - j as f32, w - k as f32);
                accum += (i as f32 * uu + (1 - i) as f32 * (1.0 - uu))
                    * (j as f32 * vv + (1 - j) as f32 * (1.0 - vv))
                    * (k as f32 * ww + (1 - k) as f32 * (1.0 - ww))
                    * dot(c[i][j][k], weight_v);
            }
        }
    }
    accum
}

fn perlin_noise(p: &Vec3) -> f32 {
    let u = p.x - p.x.floor();
    let v = p.y - p.y.floor();
    let w = p.z - p.z.floor();
    let i = p.x.floor() as i32;
    let j = p.y.floor() as i32;
    let k = p.z.floor() as i32;
    let mut c: [[[Vec3; 2]; 2]; 2] = [[[Vec3::zero(); 2]; 2]; 2];
    for di in 0..2 {
        for dj in 0..2 {
            for dk in 0..2 {
                c[di][dj][dk] = PERLIN_RANDOM_FLOAT[PERLIN_PERMUTATION_X
                    [((i + di as i32) & 255) as usize]
                    ^ PERLIN_PERMUTATION_Y[((j + dj as i32) & 255) as usize]
                    ^ PERLIN_PERMUTATION_Z[((k + dk as i32) & 255) as usize]];
            }
        }
    }
    trilinear_intepolation(&c, u, v, w)
}

type NoiseData = [usize; 256];

fn permute(p: &mut NoiseData) {
    for i in (0..p.len()).rev() {
        let target = (random_float() * (i + 1) as f32) as usize;
        p.swap(i, target);
    }
}

fn perlin_generate_permutation() -> NoiseData {
    let mut result: NoiseData = [0; 256];
    for i in 0..256 {
        result[i] = i;
    }
    permute(&mut result);
    result
}

fn perlin_generate() -> [Vec3; 256] {
    let mut result: [Vec3; 256] = [Vec3::zero(); 256];
    for i in 0..256 {
        result[i] = vec3(
            -1.0 + 2.0 * random_float(),
            -1.0 + 2.0 * random_float(),
            -1.0 + 2.0 * random_float(),
        )
        .normalize();
    }
    result
}

lazy_static! {
    static ref PERLIN_PERMUTATION_X: NoiseData = perlin_generate_permutation();
    static ref PERLIN_PERMUTATION_Y: NoiseData = perlin_generate_permutation();
    static ref PERLIN_PERMUTATION_Z: NoiseData = perlin_generate_permutation();
    static ref PERLIN_RANDOM_FLOAT: [Vec3; 256] = perlin_generate();
}
