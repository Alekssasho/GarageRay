mod camera;
mod hitable;
mod math;
mod ray;

use camera::*;
use hitable::*;
use math::*;
use ray::*;

use rand::distributions::{Distribution, Uniform};

fn random_in_unit_sphere<R: rand::Rng>(rng: &mut R, uniform: &Uniform<f32>) -> Vec3 {
    let mut rng_itr = uniform.sample_iter(rng);
    loop {
        let p = 2.0
            * vec3(
                rng_itr.next().unwrap(),
                rng_itr.next().unwrap(),
                rng_itr.next().unwrap(),
            );
        if dot(p, p) < 1.0 {
            break p;
        }
    }
}

fn color<R: rand::Rng>(
    ray: &Ray,
    world: &dyn Hitable,
    rng: &mut R,
    uniform: &Uniform<f32>,
) -> Vec3 {
    let mut rec = HitRecord::default();
    if world.hit(ray, 0.001, std::f32::MAX, &mut rec) {
        let target = rec.p + rec.normal + random_in_unit_sphere(rng, uniform);
        0.5 * color(
            &Ray {
                origin: rec.p,
                direction: target - rec.p,
            },
            world,
            rng,
            uniform,
        )
    } else {
        let unit_direction = ray.direction.normalize();
        let t = 0.5 * unit_direction.y + 1.0;
        vec3(1.0, 1.0, 1.0).lerp(vec3(0.5, 0.7, 1.0), t)
    }
}

fn main() {
    let width = 200;
    let height = 100;
    let samples = 100;

    let list: Vec<Box<dyn Hitable>> = vec![
        Box::new(Sphere {
            center: vec3(0.0, 0.0, -1.0),
            radius: 0.5,
        }),
        Box::new(Sphere {
            center: vec3(0.0, -100.5, -1.0),
            radius: 100.0,
        }),
    ];
    let world = HitableList { list };
    let camera = Camera::new();

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
                color(&ray, &world, &mut rng, &uniform_distribution)
            })
            .sum::<Vec3>()
            / samples as f32;

        let ir = (255.99 * color.x.sqrt()) as u8;
        let ig = (255.99 * color.y.sqrt()) as u8;
        let ib = (255.99 * color.z.sqrt()) as u8;
        *pixel = image::Rgb([ir, ig, ib]);
    }

    image.save("output.bmp").unwrap();
}
