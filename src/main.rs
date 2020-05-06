extern crate image;

pub mod camera;
pub mod camera_controller;
// pub mod font;
pub mod gpu;
pub mod math;
pub mod mesh;

use glfw::*;
use std::fs;
// use notify::{Watcher, RecommendedWatcher, RecursiveMode, Result};

use camera::Camera;
use camera_controller::CameraController;
// use font::Font;
use math::*;
use mesh::Mesh;

fn main() {
    let width = 800;
    let height = 600;
    let title = "rugl";

    // setup
    let mut glfw = glfw::init(glfw::FAIL_ON_ERRORS).unwrap();
    glfw.window_hint(glfw::WindowHint::ClientApi(glfw::ClientApiHint::OpenGlEs));
    glfw.window_hint(glfw::WindowHint::ContextVersion(3, 1));
    let (mut window, events) = glfw
        .create_window(width, height, title, glfw::WindowMode::Windowed)
        .expect("Failed to create GLFW window.");
    window.make_current();
    window.set_key_polling(true);
    window.set_mouse_button_polling(true);
    window.set_focus_polling(true);
    window.set_cursor_enter_polling(true);
    gl::load_with(|s| window.get_proc_address(s));
    gpu::setup();

    // camera
    let mut camera = Camera {
        position: 2. * Vector3::X,
        orientation: Quaternion::rotation(vec3(1., 1., 1.), 2. * FRAC_PI_3),
    };
    let mut camera_ctl = CameraController::new();

    // meshes
    // let mesh_source = fs::read_to_string("resources/ico.obj").unwrap();
    // let icosphere = Mesh::load_obj(&mesh_source);
    // let cube = Mesh::new_cube();
    let quad = Mesh::new_quad(1.0, 2.0);

    // shader
    let mut shader = gpu::Program::from_files("vertex.glsl", "fragment.glsl").unwrap();
    let model_transform_uniform = shader.get_uniform("T_model").unwrap();
    let view_transform_uniform = shader.get_uniform("T_view").unwrap();
    let projection_transform_uniform = shader.get_uniform("T_projection").unwrap();
    shader.set_uniform(model_transform_uniform, &Matrix4::id());
    shader.set_uniform(view_transform_uniform, &camera.view_matrix());
    shader.set_uniform(
        projection_transform_uniform,
        &Matrix4::perspective(FRAC_PI_3, width as f32 / height as f32, 0.5, 10.0),
    );

    // load image
    let img = image::open("resources/nav.png").unwrap().into_rgb();
    let img_width = img.width() as i32;
    let img_height = img.height() as i32;
    let img_data = img.into_raw();

    // let img_width = 2;
    // let img_height = 4;
    // let img_data: Vec<u8> = vec![
    //     255, 0, 0,   255, 255, 0,     //0,0,
    //     0, 255, 0,   255, 255, 0,     //0,0,
    //     0, 0, 255,   255, 0, 255,     //0,0,
    //     0, 255, 255,   255, 255, 255, //0,0,
    //     // 0, 255, 0,  0, 255, 0,
    //     // 0, 0, 255,  0, 0, 255,
    // ];

    // setup texture
    let mut texture = gpu::Texture::new();
    texture.load_data(img_width, img_height, &img_data);

    let texture_unit = gpu::TextureUnit(0);
    texture_unit.activate(texture);
    let texture_uniform = shader.get_uniform("texture0").unwrap();
    shader.set_uniform(texture_uniform, texture_unit);

    while !window.should_close() {
        // draw
        gpu::clear(1.0, 1.0, 1.0, 1.0);
        shader.activate();
        // cube.draw();
        // icosphere.draw();
        quad.draw();
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
        shader.set_uniform(view_transform_uniform, &camera.view_matrix());
    }
}
