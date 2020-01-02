use garage_ray_simple::*;

fn render_image() -> image::ImageBuffer<image::Rgb<u8>, Vec<u8>> {
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
    image
}

use glium::glutin::{self, Event, WindowEvent};
use glium::{Surface};
use glium::backend::Facade;
use imgui::*;
use imgui_glium_renderer::Renderer;
use imgui_winit_support::{HiDpiMode, WinitPlatform};
use std::time::Instant;

fn ui_code(ui: &mut Ui, id: TextureId) {
    Window::new(im_str!("Hello textures"))
        .size([800.0, 800.0], Condition::FirstUseEver)
        .build(ui, || {
            //ui.text(im_str!("Hello textures!"));
            Image::new(id, [800.0, 800.0]).build(ui);
        });
}

fn main() {
    let image = render_image();

    let mut events_loop = glutin::EventsLoop::new();
    let context = glutin::ContextBuilder::new().with_vsync(true);
    let builder = glutin::WindowBuilder::new()
        .with_title("Garage Ray VFB: Simple Renderer")
        .with_dimensions(glutin::dpi::LogicalSize::new(1024_f64, 1024_f64));
    let display = glium::Display::new(builder, context, &events_loop).expect("Failed to initialize display");

    let mut imgui = imgui::Context::create();
    imgui.set_ini_filename(None);

    // Set clipboard

    let mut platform = WinitPlatform::init(&mut imgui);
    let gl_window = display.gl_window();
    let window = gl_window.window();
    platform.attach_window(imgui.io_mut(), &window, HiDpiMode::Rounded);

    let hidpi_factor = platform.hidpi_factor();
    let font_size = (13.0 * hidpi_factor) as f32;
    imgui.fonts().add_font(&[
        FontSource::DefaultFontData {
            config: Some(FontConfig {
                size_pixels: font_size,
                ..FontConfig::default()
            }),
        }
    ]);

    imgui.io_mut().font_global_scale = (1.0 / hidpi_factor) as f32;
    let mut renderer = Renderer::init(&mut imgui, &display).expect("Failed to initialize renderer");

    let mut last_frame = Instant::now();
    let mut run = true;

    let dim = image.dimensions();

    let raw = glium::texture::RawImage2d{
        data: std::borrow::Cow::Owned(image.into_raw()),
        width: dim.0,
        height: dim.1,
        format: glium::texture::ClientFormat::U8U8U8,
    };
    let gl_texture = glium::Texture2d::new(display.get_context(), raw).expect("Failed to create gl texture");
    let texture_id = renderer.textures().insert(std::rc::Rc::new(gl_texture));

    while run {
        events_loop.poll_events(|event| {
            platform.handle_event(imgui.io_mut(), &window, &event);

            if let Event::WindowEvent{ event, .. } = event {
                if let WindowEvent::CloseRequested = event {
                    run = false;
                }
            }
        });
        let io = imgui.io_mut();
        platform.prepare_frame(io, &window).expect("Failed to start frame");
        last_frame = io.update_delta_time(last_frame);
        let mut ui = imgui.frame();
        ui_code(&mut ui, texture_id);

        let mut target = display.draw();
        target.clear_color_srgb(1.0, 1.0, 1.0, 1.0);
        platform.prepare_render(&ui, &window);
        let draw_data = ui.render();
        renderer.render(&mut target, draw_data).expect("Rendering failed");
        target.finish().expect("Failed to swap buffers");
    }
}