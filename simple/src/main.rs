mod math;
mod ray;

use math::*;
use ray::*;

fn color(ray: &Ray) -> Vec3 {
    let unit_direction = ray.direction.normalize();
    let t = 0.5 * unit_direction.y + 1.0;
    vec3(1.0, 1.0, 1.0).lerp(vec3(0.5, 0.7, 1.0), t)
}

fn main() {
    let width = 200;
    let height = 100;

    let lower_left_corner = vec3(-2.0, -1.0, -1.0);
    let horizontal = vec3(4.0, 0.0, 0.0);
    let vertical = vec3(0.0, 2.0, 0.0);
    let origin = vec3(0.0, 0.0, 0.0);

    let mut image = image::ImageBuffer::new(width, height);
    for (x, y, pixel) in image.enumerate_pixels_mut() {
        let u = x as f32 / width as f32;
        let v = (height - y - 1) as f32 / height as f32;

        let ray = Ray {
            origin,
            direction: lower_left_corner + u * horizontal + v * vertical,
        };
        let color = color(&ray);

        let ir = (255.99 * color.x) as u8;
        let ig = (255.99 * color.y) as u8;
        let ib = (255.99 * color.z) as u8;
        *pixel = image::Rgb([ir, ig, ib]);
    }

    image.save("output.bmp").unwrap();
}
