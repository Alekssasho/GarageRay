use garage_ray_simple::*;

struct PixelRow {
    y: u32,
    data: Vec<(u8, u8, u8)>,
}

fn render_image(sender: std::sync::mpsc::Sender<PixelRow>) {
    let now = std::time::Instant::now();
    let width = 800;
    let height = 800;
    let samples = 100;

    //let world = random_scene();
    //let world = two_perlin_spheres();
    //let world = simple_light();
    let world = cornel_box();
    //let world = cornel_smoke();
    //let world = final_scene();
    let accelerated_world = BVHNode::build(world, 0.0, 1.0);
    //let accelerated_world = world;

    //let look_from = vec3(478.0, 278.0, -600.0);
    let look_from = vec3(278.0, 278.0, -800.0);
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

    for y in 0..800 {
        let mut row_data = Vec::with_capacity(800);
        for x in 0..800 {
            let (ir, ig, ib) =
                evaluate_pixel(x, y, width, height, samples, &accelerated_world, &camera);
            //*pixel = image::Rgb([ir, ig, ib]);
            row_data.push((ir, ig, ib));
        }
        if let Err(_) = sender.send(PixelRow { y, data: row_data }) {
            println!("Render interrupted at {} seconds", now.elapsed().as_secs());
            return;
        }
    }

    println!("Render took {} seconds", now.elapsed().as_secs());
}

use glium::backend::Facade;
use glium::glutin::{self, Event, WindowEvent};
use glium::Surface;
use imgui::*;
use imgui_glium_renderer::Renderer;
use imgui_winit_support::{HiDpiMode, WinitPlatform};
use std::time::Instant;

fn save_render(id: TextureId, renderer: &mut Renderer) {
    let raw_data: Vec<Vec<(u8, u8, u8, u8)>> = (*renderer.textures().get(id).unwrap()).read();
    let raw_data_flattened: Vec<u8> = raw_data
        .into_iter()
        .flatten()
        .collect::<Vec<(u8, u8, u8, u8)>>()
        .into_iter()
        .flat_map(|data| {
            let r = std::iter::once(data.0);
            let g = std::iter::once(data.1);
            let b = std::iter::once(data.2);
            r.chain(g).chain(b)
        })
        .collect::<Vec<u8>>();
    let image =
        image::ImageBuffer::<image::Rgb<u8>, _>::from_raw(800, 800, raw_data_flattened).unwrap();
    image.save("output.bmp").unwrap();
}

fn ui_code(ui: &mut Ui, id: TextureId, renderer: &mut Renderer) {
    let io = ui.io();
    Window::new(im_str!("Hello textures"))
        .size(io.display_size, Condition::Always)
        .position([0.0, 0.0], Condition::Always)
        .title_bar(false)
        .resizable(false)
        .movable(false)
        .scrollable(false)
        .scroll_bar(false)
        .save_settings(false)
        .bring_to_front_on_focus(false)
        .menu_bar(true)
        .draw_background(false)
        .build(ui, || {
            ui.menu_bar(|| {
                ui.menu(im_str!("File"), true, || {
                    if MenuItem::new(im_str!("Save Render")).build(ui) {
                        save_render(id, renderer);
                    }
                });
            });
            ui.text(im_str!("Rendered Imaged:"));
            Image::new(id, [800.0, 800.0])
                .border_col([1.0, 1.0, 1.0, 1.0])
                .build(ui);
        });
}

fn main() {
    let mut events_loop = glutin::EventsLoop::new();
    let context = glutin::ContextBuilder::new().with_vsync(true);
    let builder = glutin::WindowBuilder::new()
        .with_title("Garage Ray VFB: Simple Renderer")
        .with_dimensions(glutin::dpi::LogicalSize::new(1024_f64, 1024_f64));
    let display =
        glium::Display::new(builder, context, &events_loop).expect("Failed to initialize display");

    let mut imgui = imgui::Context::create();
    imgui.set_ini_filename(None);

    // Set clipboard

    let mut platform = WinitPlatform::init(&mut imgui);
    let gl_window = display.gl_window();
    let window = gl_window.window();
    platform.attach_window(imgui.io_mut(), &window, HiDpiMode::Rounded);

    let hidpi_factor = platform.hidpi_factor();
    let font_size = (13.0 * hidpi_factor) as f32;
    imgui.fonts().add_font(&[FontSource::DefaultFontData {
        config: Some(FontConfig {
            size_pixels: font_size,
            ..FontConfig::default()
        }),
    }]);

    imgui.io_mut().font_global_scale = (1.0 / hidpi_factor) as f32;
    let mut renderer = Renderer::init(&mut imgui, &display).expect("Failed to initialize renderer");

    let mut last_frame = Instant::now();
    let mut run = true;

    let dim: (u32, u32) = (800, 800);
    let black_data: Vec<u8> = vec![0; (dim.0 * dim.1 * 3) as usize];

    let raw = glium::texture::RawImage2d {
        data: std::borrow::Cow::Owned(black_data),
        width: dim.0,
        height: dim.1,
        format: glium::texture::ClientFormat::U8U8U8,
    };
    let gl_texture =
        glium::Texture2d::new(display.get_context(), raw).expect("Failed to create gl texture");
    let texture_id = renderer.textures().insert(std::rc::Rc::new(gl_texture));

    let (sender, receiver) = std::sync::mpsc::channel();
    let thread_handle = std::thread::spawn(move || {
        render_image(sender);
    });

    while run {
        events_loop.poll_events(|event| {
            platform.handle_event(imgui.io_mut(), &window, &event);

            if let Event::WindowEvent { event, .. } = event {
                if let WindowEvent::CloseRequested = event {
                    run = false;
                }
            }
        });

        for pixel in receiver.try_iter() {
            (*renderer.textures().get(texture_id).unwrap()).write(
                glium::Rect {
                    left: 0,
                    bottom: pixel.y,
                    width: dim.0,
                    height: 1,
                },
                vec![pixel.data],
            );
        }

        let io = imgui.io_mut();
        platform
            .prepare_frame(io, &window)
            .expect("Failed to start frame");
        last_frame = io.update_delta_time(last_frame);
        let mut ui = imgui.frame();
        ui_code(&mut ui, texture_id, &mut renderer);

        let mut target = display.draw();
        target.clear_color_srgb(0.0, 0.0, 0.0, 1.0);
        platform.prepare_render(&ui, &window);
        let draw_data = ui.render();
        renderer
            .render(&mut target, draw_data)
            .expect("Rendering failed");
        target.finish().expect("Failed to swap buffers");
    }
    drop(receiver);

    thread_handle.join().unwrap();
}
