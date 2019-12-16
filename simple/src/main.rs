mod camera;
mod hitable;
mod material;
mod math;
mod random;
mod ray;

use camera::*;
use hitable::*;
use material::*;
use math::*;
use ray::*;

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

fn main() {
    let width = 200;
    let height = 100;
    let samples = 100;

    let world: Vec<Box<dyn Hitable>> = vec![
        Box::new(Sphere {
            center: vec3(0.0, 0.0, -1.0),
            radius: 0.5,
            material: Box::new(Lambertian {
                albedo: vec3(0.1, 0.2, 0.5),
            }),
        }),
        Box::new(Sphere {
            center: vec3(0.0, -100.5, -1.0),
            radius: 100.0,
            material: Box::new(Lambertian {
                albedo: vec3(0.8, 0.8, 0.0),
            }),
        }),
        Box::new(Sphere {
            center: vec3(1.0, 0.0, -1.0),
            radius: 0.5,
            material: Box::new(Metal {
                albedo: vec3(0.8, 0.6, 0.2),
                fuzz: 0.0,
            }),
        }),
        Box::new(Sphere {
            center: vec3(-1.0, 0.0, -1.0),
            radius: 0.5,
            material: Box::new(Dielectric { ref_index: 1.5 }),
        }),
        Box::new(Sphere {
            center: vec3(-1.0, 0.0, -1.0),
            radius: -0.45,
            material: Box::new(Dielectric { ref_index: 1.5 }),
        }),
    ];
    let look_from = vec3(3.0, 3.0, 2.0);
    let look_at = vec3(0.0, 0.0, -1.0);
    let camera = Camera::new(
        look_from,
        look_at,
        vec3(0.0, 1.0, 0.0),
        20.0,
        width as f32 / height as f32,
        2.0,
        (look_from - look_at).magnitude()
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
                color(&ray, &world, 0)
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
