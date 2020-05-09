pub mod camera;
pub mod camera_controller;
// pub mod font;
pub mod gpu;
pub mod math;
pub mod mesh;
pub mod shader;
pub mod text;

use glfw::*;
use std::fs;
// use notify::{Watcher, RecommendedWatcher, RecursiveMode, Result};

use camera::Camera;
use camera_controller::CameraController;
// use font::Font;
use math::*;
use mesh::Mesh;
use shader::MeshShader;


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

    // load image
    let img = image::open("resources/nav.png").unwrap().into_rgb();
    let img_width = img.width() as i32;
    let img_height = img.height() as i32;
    let img_data = img.into_raw();

    // rasterize text image
    let ft = text::init_library().unwrap();
    let mut font = text::Font::open(&ft, "/usr/share/fonts/TTF/DejaVuSerif.ttf").unwrap();
    let a_bitmap = font.char_bitmap('A').unwrap();
    let text_width = a_bitmap.width() as i32;
    let text_height = a_bitmap.rows() as i32;
    let text_img_data = a_bitmap.buffer();

    // camera
    let mut camera = Camera {
        position: 3. * Vector3::X + Vector3::Z,
        orientation: Quaternion::rotation(vec3(1., 1., 1.), 2. * FRAC_PI_3),
        aspect_ratio: width as f32 / height as f32,
    };
    let mut camera_ctl = CameraController::new();

    // meshes
    let mesh_source = fs::read_to_string("resources/ico.obj").unwrap();
    let icosphere = Mesh::load_obj(&mesh_source);
    let cube = Mesh::new_cube();
    let quad = Mesh::new_quad(1.0, 2.0);
    let text = Mesh::new_quad(0.005 * text_width as f32, 0.005 * text_height as f32);
    
    let icosphere_position = vec3(0.,0.,0.);
    let mut cube_position = vec3(0.,1.,0.);
    let quad_position = vec3(0.,0.,2.);
    let text_position = vec3(1.,0.,1.);

    // shaders
    let mut mesh_shader = MeshShader::new("shaders/mesh_frag.glsl").unwrap();
    let mut card_shader = MeshShader::new("shaders/card_frag.glsl").unwrap();
    let mut text_shader = MeshShader::new("shaders/text_frag.glsl").unwrap();

    // setup textures
    let texture_unit = gpu::TextureUnit(0);
    let mut quad_texture = gpu::Texture::new();
    quad_texture.load_data(gpu::TextureFormat::Rgb, img_width, img_height, &img_data);
    let mut text_texture = gpu::Texture::new();
    text_texture.load_data(gpu::TextureFormat::Alpha, text_width, text_height, &text_img_data);

    let quad_texture_uniform = card_shader.get_uniform("texture0").unwrap();
    let text_texture_uniform = text_shader.get_uniform("texture0").unwrap();

    // vars
    let mut t: f32 = 0.0;

    while !window.should_close() {
        // draw
        gpu::clear(1.0, 1.0, 1.0, 1.0);
        mesh_shader.set_view_matrix(&camera.view_projection_matrix());
        card_shader.set_view_matrix(&camera.view_projection_matrix());
        text_shader.set_view_matrix(&camera.view_projection_matrix());

        mesh_shader.set_model_transform(&Matrix4::translate(icosphere_position));
        mesh_shader.draw(&icosphere);

        mesh_shader.set_model_transform(&Matrix4::translate(cube_position));
        mesh_shader.draw(&cube);

        card_shader.set_texture(quad_texture_uniform, texture_unit, quad_texture);
        card_shader.set_model_transform(&Matrix4::translate(quad_position));
        card_shader.draw(&quad);

        text_shader.set_texture(text_texture_uniform, texture_unit, text_texture);
        text_shader.set_model_transform(&Matrix4::translate(text_position));
        text_shader.draw(&text);

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
        let w = 0.5;
        let r = 1.5;
        cube_position = vec3(0.0, r*(TAU * w * t).sin(), r*(TAU * w * t).cos());
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


