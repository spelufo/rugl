pub mod gpu;
pub mod math;
pub mod mesh;

use glfw::{Action, Context, Key, WindowEvent};
use math::*;
use mesh::Mesh;
use std::f32::consts::*;
use std::fs;

fn main() {
    let mut glfw = glfw::init(glfw::FAIL_ON_ERRORS).unwrap();
    glfw.window_hint(glfw::WindowHint::ClientApi(glfw::ClientApiHint::OpenGlEs));
    glfw.window_hint(glfw::WindowHint::ContextVersion(3, 1));

    let width = 800;
    let height = 600;
    let title = "rugl";
    let (mut window, events) = glfw
        .create_window(width, height, title, glfw::WindowMode::Windowed)
        .expect("Failed to create GLFW window.");

    window.make_current();
    window.set_key_polling(true);
    gl::load_with(|s| window.get_proc_address(s));

    gpu::setup();

    let mut t = 0.0;
    let shader_program = gpu::Program::from_files("vertex.glsl", "fragment.glsl").unwrap();
    shader_program.set_uniform(b"T_model\0", &Matrix4::id());
    shader_program.set_uniform(b"T_view\0", &(Matrix4::translate(0.0, 0.0, -1.0) * Matrix4::rotate_z(t)));
    shader_program.set_uniform(b"T_projection\0", &Matrix4::perspective(FRAC_PI_3, width as f32 / height as f32, 0.5, 10.0));

    let mesh_source = fs::read_to_string("resources/ico.obj").unwrap();
    let mesh = Mesh::load_obj(&mesh_source);
    // let mesh2 = Mesh::new_cube();

    while !window.should_close() {
        gpu::clear(1.0, 1.0, 1.0, 1.0);
        shader_program.activate();
        // mesh2.draw();
        mesh.draw();
        window.swap_buffers();

        shader_program.set_uniform(b"T_view\0", &(Matrix4::translate(0.0, 0.0, -2.0) * Matrix4::rotate_y(t)));
        t += 0.02;

        glfw.poll_events();
        for (_, event) in glfw::flush_messages(&events) {
            println!("{:?}", event);
            match event {
                WindowEvent::Key(Key::Escape, _, Action::Press, _) => window.set_should_close(true),
                _ => {}
            }
        }
    }
}
