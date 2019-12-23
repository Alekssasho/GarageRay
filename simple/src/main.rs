mod camera;
mod hitable;
mod material;
mod math;
mod random;
mod ray;
mod texture;

#[macro_use]
extern crate lazy_static;

use camera::*;
use hitable::*;
use material::*;
use math::*;
use random::random_float;
use ray::*;
use texture::*;

use rand::distributions::Distribution;

fn color(ray: &Ray, world: &dyn Hitable, depth: i32) -> Vec3 {
    if let Some(rec) = world.hit(ray, 0.001, std::f32::MAX) {
        // TODO: this is potentially wrong
        match rec.material.unwrap().scatter(ray, &rec) {
            Some((attenuation, scattered)) if depth < 50 => {
                color(&scattered, world, depth + 1).mul_element_wise(attenuation)
            }
            _ => vec3(0.0, 0.0, 0.0),
        }
    } else {
        let unit_direction = ray.direction.normalize();
        let t = 0.5 * unit_direction.y + 1.0;
        vec3(1.0, 1.0, 1.0).lerp(vec3(0.5, 0.7, 1.0), t)
    }
}

#[allow(dead_code)]
fn random_scene() -> Vec<Box<dyn Hitable>> {
    let n = 500;
    let mut list: Vec<Box<dyn Hitable>> = Vec::with_capacity(n + 1);
    list.push(Box::new(Sphere {
        center: vec3(0.0, -1000.0, 0.0),
        radius: 1000.0,
        material: Box::new(Lambertian {
            albedo: Box::new(CheckerTexture {
                even: Box::new(ConstantTexture(vec3(0.2, 0.3, 0.1))),
                odd: Box::new(ConstantTexture(vec3(0.9, 0.9, 0.9))),
            }),
        }),
    }));
    for a in -11..11 {
        for b in -11..11 {
            let choose_mat = random_float();
            let center = vec3(
                a as f32 + 0.9 * random_float(),
                0.2,
                b as f32 + 0.9 * random_float(),
            );
            if (center - vec3(4.0, 0.2, 0.0)).magnitude() > 0.9 {
                match choose_mat {
                    x if x < 0.8 => list.push(Box::new(MovingSphere {
                        center0: center,
                        center1: center + vec3(0.0, 0.5 * random_float(), 0.0),
                        time0: 0.0,
                        time1: 1.0,
                        radius: 0.2,
                        material: Box::new(Lambertian {
                            albedo: Box::new(ConstantTexture(vec3(
                                random_float() * random_float(),
                                random_float() * random_float(),
                                random_float() * random_float(),
                            ))),
                        }),
                    })),
                    x if x < 0.95 => list.push(Box::new(Sphere {
                        center,
                        radius: 0.2,
                        material: Box::new(Metal {
                            albedo: vec3(
                                0.5 * (1.0 + random_float()),
                                0.5 * (1.0 + random_float()),
                                0.5 * (1.0 + random_float()),
                            ),
                            fuzz: 0.5 * random_float(),
                        }),
                    })),
                    _ => list.push(Box::new(Sphere {
                        center,
                        radius: 0.2,
                        material: Box::new(Dielectric { ref_index: 1.5 }),
                    })),
                }
            }
        }
    }

    list.push(Box::new(Sphere {
        center: vec3(0.0, 1.0, 0.0),
        radius: 1.0,
        material: Box::new(Dielectric { ref_index: 1.5 }),
    }));
    list.push(Box::new(Sphere {
        center: vec3(-4.0, 1.0, 0.0),
        radius: 1.0,
        material: Box::new(Lambertian {
            albedo: Box::new(ConstantTexture(vec3(0.4, 0.2, 0.1))),
        }),
    }));
    list.push(Box::new(Sphere {
        center: vec3(4.0, 1.0, 0.0),
        radius: 1.0,
        material: Box::new(Metal {
            albedo: vec3(0.7, 0.6, 0.5),
            fuzz: 0.0,
        }),
    }));
    list
}

fn two_perlin_spheres() -> Vec<Box<dyn Hitable>> {
    let noise = Box::new(NoiseTexture { scale: 4.0 });
    let list: Vec<Box<dyn Hitable>> = vec![
        Box::new(Sphere {
            center: vec3(0.0, -1000.0, 0.0),
            radius: 1000.0,
            material: Box::new(Lambertian {
                albedo: noise.clone(),
            }),
        }),
        Box::new(Sphere {
            center: vec3(0.0, 2.0, 0.0),
            radius: 2.0,
            material: Box::new(Lambertian { albedo: noise }),
        }),
    ];
    list
}

fn main() {
    let now = std::time::Instant::now();
    let width = 1200;
    let height = 800;
    let samples = 10;

    //let world = random_scene();
    let world = two_perlin_spheres();
    let accelerated_world = BVHNode::build(world, 0.0, 1.0);

    let look_from = vec3(13.0, 2.0, 3.0);
    let look_at = vec3(0.0, 0.0, 0.0);
    let camera = Camera::new(
        look_from,
        look_at,
        vec3(0.0, 1.0, 0.0),
        20.0,
        width as f32 / height as f32,
        0.0,
        10.0,
        0.0,
        1.0,
    );

    let mut rng = rand::thread_rng();
    let uniform_distribution = rand::distributions::Uniform::new(0.0, 1.0);

    let mut image = image::ImageBuffer::new(width, height);
    for (x, y, pixel) in image.enumerate_pixels_mut() {
        let color: Vec3 = (0..samples)
            .map(|_| {
                let u = (x as f32 + uniform_distribution.sample(&mut rng)) / width as f32;
                let v = ((height - y - 1) as f32 + uniform_distribution.sample(&mut rng))
                    / height as f32;
                let ray = camera.get_ray(u, v);
                color(&ray, &accelerated_world, 0)
            })
            .sum::<Vec3>()
            / samples as f32;

        let ir = (255.99 * color.x.sqrt()) as u8;
        let ig = (255.99 * color.y.sqrt()) as u8;
        let ib = (255.99 * color.z.sqrt()) as u8;
        *pixel = image::Rgb([ir, ig, ib]);
    }

    image.save("output.bmp").unwrap();
    println!("Render took {} seconds", now.elapsed().as_secs());
}
