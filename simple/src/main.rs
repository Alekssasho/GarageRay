mod math;

use math::*;

fn main() {
    let width = 200;
    let height = 100;
    let mut image = image::ImageBuffer::new(width, height);
    for (x, y, pixel) in image.enumerate_pixels_mut() {
        let color = vec3(x as f32 / width as f32, y as f32 / height as f32, 0.2);
        let ir = (255.99 * color.x) as u8;
        let ig = (255.99 * color.y) as u8;
        let ib = (255.99 * color.z) as u8;
        *pixel = image::Rgb([ir, ig, ib]);
    }

    image.save("output.bmp").unwrap();
}