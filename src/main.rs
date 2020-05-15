pub mod camera;
pub mod camera_controller;
pub mod gpu;
pub mod image;
pub mod math;
pub mod mesh;
pub mod text;

use glfw::*;
use std::fs;
// use notify::{Watcher, RecommendedWatcher, RecursiveMode, Result};

use camera::Camera;
use camera_controller::CameraController;
use math::*;
use mesh::{Mesh, MeshShader};
use text::{Text, TextShader};

fn main() {
    let width = 1200;
    let height = 900;
    let title = "rugl";

    // setup
    let mut glfw = glfw::init(glfw::FAIL_ON_ERRORS).unwrap();
    setup_window_hints(&mut glfw);
    let (mut window, events) = glfw
        .create_window(width, height, title, glfw::WindowMode::Windowed)
        .expect("Failed to create GLFW window.");
    setup_input(&mut window);
    setup_gl(&mut window);

    
    // camera
    let mut camera = Camera {
        position: 3. * Vector3::X + Vector3::Z,
        orientation: Quaternion::rotation(Vector3::new(1., 1., 1.), 2. * FRAC_PI_3),
        aspect_ratio: width as f32 / height as f32,
    };
    let mut camera_ctl = CameraController::new();

    // text
    let ft = text::init_library().unwrap();
    let mut font = text::Font::open(&ft, "/usr/share/fonts/TTF/DejaVuSerif.ttf", 32).unwrap();
    let font_atlas = font.make_atlas().unwrap();
    let text  = Text::new("Hello, world!", Vector2::new(30., 100.), &mut font, &font_atlas);
    let text2 = Text::new("Lorem ipsum dolor sit amet, consectetur adipisicing elit, sed do eiusmod",
        Vector2::new(30., 170.), &mut font, &font_atlas);

    // load image
    let img = image::open("resources/nav.png").unwrap().into_rgb();
    let img_width = img.width() as i32;
    let img_height = img.height() as i32;
    let img_data = img.into_raw();

    // meshes
    let mesh_source = fs::read_to_string("resources/ico.obj").unwrap();
    let icosphere = Mesh::load_obj(&mesh_source);
    let cube = Mesh::new_cube();
    let quad = Mesh::new_quad(1.0, 2.0);
    
    let icosphere_position = Vector3::new(0.,0.,0.);
    let mut cube_position = Vector3::new(0.,1.,0.);
    let quad_position = Vector3::new(0.,0.,2.);

    // shaders
    let mut mesh_shader = MeshShader::new("shaders/mesh_frag.glsl").unwrap();
    let mut card_shader = MeshShader::new("shaders/card_frag.glsl").unwrap();
    let mut text_shader = TextShader::new().unwrap();
    text_shader.set_screen_size(Vector2::new(width as f32, height as f32));

    // setup textures
    let texture_unit = gpu::TextureUnit(0);
    let mut quad_texture = gpu::Texture::new();
    quad_texture.load_data(gpu::TextureFormat::Rgb, img_width, img_height, &img_data);

    let quad_texture_uniform = card_shader.get_uniform("texture0").unwrap();

    // vars
    let mut t: f32 = 0.0;

    while !window.should_close() {
        // draw
        gpu::clear(1.0, 1.0, 1.0, 1.0);
        mesh_shader.set_view_matrix(&camera.view_projection_matrix());
        card_shader.set_view_matrix(&camera.view_projection_matrix());

        mesh_shader.set_model_transform(&Matrix4::translate(icosphere_position));
        mesh_shader.draw(&icosphere);

        mesh_shader.set_model_transform(&Matrix4::translate(cube_position));
        mesh_shader.draw(&cube);

        card_shader.set_texture(quad_texture_uniform, texture_unit, quad_texture);
        card_shader.set_model_transform(&Matrix4::translate(quad_position));
        card_shader.draw(&quad);

        text_shader.draw(&text);
        text_shader.draw(&text2);

        window.swap_buffers();

        // update
        glfw.poll_events();
        for (_, event) in glfw::flush_messages(&events) {
            match event {
                WindowEvent::Key(Key::Escape, _, Action::Press, _) => window.set_should_close(true),
                WindowEvent::Focus(false) => camera_ctl.deactivate(&mut window),
                WindowEvent::CursorEnter(false) => camera_ctl.deactivate(&mut window),
                WindowEvent::MouseButton(_, Action::Press, _) => camera_ctl.activate(&mut window),
                _ => {}
            }
        }

        camera_ctl.update(&mut camera, &window);
        let w = 0.25;
        let r = 3.0;
        cube_position = Vector3::new(0.0, r*(TAU * w * t).sin(), r*(TAU * w * t).cos());
        t += 0.016;
    }
}

fn setup_window_hints(glfw: &mut glfw::Glfw) {
    glfw.window_hint(glfw::WindowHint::ClientApi(glfw::ClientApiHint::OpenGlEs));
    glfw.window_hint(glfw::WindowHint::ContextVersion(3, 1));
}

fn setup_gl(window: &mut glfw::Window) {
    window.make_current();
    gl::load_with(|s| window.get_proc_address(s));
    gpu::setup();
}

fn setup_input(window: &mut glfw::Window) {
    window.set_key_polling(true);
    window.set_mouse_button_polling(true);
    window.set_focus_polling(true);
    window.set_cursor_enter_polling(true);
}


