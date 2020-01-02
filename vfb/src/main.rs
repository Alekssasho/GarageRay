use garage_ray_simple::*;

fn main() {
    let now = std::time::Instant::now();
    let width = 800;
    let height = 800;
    let samples = 10;

    //let world = random_scene();
    //let world = two_perlin_spheres();
    //let world = simple_light();
    //let world = cornel_box();
    //let world = cornel_smoke();
    let world = final_scene();
    let accelerated_world = BVHNode::build(world, 0.0, 1.0);

    let look_from = vec3(478.0, 278.0, -600.0);
    let look_at = vec3(278.0, 278.0, 0.0);
    let camera = Camera::new(
        look_from,
        look_at,
        vec3(0.0, 1.0, 0.0),
        40.0,
        width as f32 / height as f32,
        0.0,
        10.0,
        0.0,
        1.0,
    );

    
    let mut image = image::ImageBuffer::new(width, height);
    for (x, y, pixel) in image.enumerate_pixels_mut() {
        let (ir, ig, ib) = evaluate_pixel(x, y, width, height, samples, &accelerated_world, &camera);
        *pixel = image::Rgb([ir, ig, ib]);
    }

    image.save("output.bmp").unwrap();
    println!("Render took {} seconds", now.elapsed().as_secs());
}
