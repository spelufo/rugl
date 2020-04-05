pub mod gpu;
pub mod math;
pub mod mesh;

use glfw::{Action, Context, Key, WindowEvent};
use math::*;
use mesh::Mesh;

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

    let shader_program = gpu::Program::from_files("vertex.glsl", "fragment.glsl").unwrap();
    let mesh = Mesh::new_cube();

    while !window.should_close() {
        gpu::clear(1.0, 1.0, 1.0, 1.0);
        shader_program.activate();
        mesh.draw();
        window.swap_buffers();

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
