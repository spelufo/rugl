pub mod camera;
pub mod camera_controller;
pub mod gpu;
pub mod math;
pub mod mesh;

use glfw::*;
use std::fs;

use camera::Camera;
use camera_controller::CameraController;
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
    let mesh_source = fs::read_to_string("resources/ico.obj").unwrap();
    let icosphere = Mesh::load_obj(&mesh_source);
    let cube = Mesh::new_cube();

    // shader
    let shader = gpu::Program::from_files("vertex.glsl", "fragment.glsl").unwrap();
    shader.set_uniform(b"T_model\0", &Matrix4::id());
    shader.set_uniform(b"T_view\0", &camera.view_matrix());
    shader.set_uniform(
        b"T_projection\0",
        &Matrix4::perspective(FRAC_PI_3, width as f32 / height as f32, 0.5, 10.0),
    );

    while !window.should_close() {
        // draw
        gpu::clear(1.0, 1.0, 1.0, 1.0);
        shader.activate();
        cube.draw();
        icosphere.draw();
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
        shader.set_uniform(b"T_view\0", &camera.view_matrix());
    }
}
