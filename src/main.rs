pub mod camera;
pub mod camera_controller;
pub mod gpu;
pub mod image;
pub mod math;
pub mod mesh;
pub mod text;

use glfw::*;

use math::*;
use text::{Font, Text, TextShader};

fn main() {
    let width = 1200;
    let height = 900;
    let title = "rugl";

    // setup
    let mut glfw = glfw::init(glfw::FAIL_ON_ERRORS).unwrap();
    setup_window_hints(&mut glfw);
    let (mut window, events) = glfw.create_window(width, height, title, glfw::WindowMode::Windowed).unwrap();
    setup_input(&mut window);
    setup_gl(&mut window);

    // text
    let freetype = text::init_library().unwrap();
    let mut font = Font::open("resources/times.ttf", 18, &freetype).unwrap();
    let lorem = "Lorem ipsum dolor sit amet, consectetur adipisicing elit, sed do eiusmod";
    let text  = Text::new(lorem, Vector2::new(30., 100.), &mut font);
    let text2 = Text::new(lorem, Vector2::new(30.25, 140.), &mut font);
    let mut font2 = Font::open("resources/times.ttf", 42, &freetype).unwrap();
    let text3 = Text::new("Welló!!", Vector2::new(30., 200.), &mut font2);


    // shaders
    let mut text_shader = TextShader::new().unwrap();
    text_shader.set_screen_size(Vector2::new(width as f32, height as f32));

    let mut msaa = false;
    let mut gamma_correction = false;

    while !window.should_close() {
        // draw
        gpu::clear(1.0, 1.0, 1.0, 1.0);

        text_shader.draw(&text);
        text_shader.draw(&text2);
        text_shader.draw(&text3);

        window.swap_buffers();

        // update
        glfw.poll_events();
        for (_, event) in glfw::flush_messages(&events) {
            match event {
                WindowEvent::Key(Key::Escape, _, Action::Press, _) => window.set_should_close(true),
                WindowEvent::Key(Key::A, _, Action::Press, _) => toggle_gpu_feature(&mut msaa, gl::MULTISAMPLE),
                WindowEvent::Key(Key::G, _, Action::Press, _) => toggle_gpu_feature(&mut gamma_correction, gl::FRAMEBUFFER_SRGB),
                // WindowEvent::Focus(false) => camera_ctl.deactivate(&mut window),
                // WindowEvent::CursorEnter(false) => camera_ctl.deactivate(&mut window),
                // WindowEvent::MouseButton(_, Action::Press, _) => camera_ctl.activate(&mut window),
                _ => {}
            }
        }
    }
}

fn setup_window_hints(glfw: &mut glfw::Glfw) {
    glfw.window_hint(glfw::WindowHint::ClientApi(glfw::ClientApiHint::OpenGlEs));
    glfw.window_hint(glfw::WindowHint::ContextVersion(3, 1));
    glfw.window_hint(glfw::WindowHint::Samples(Some(4)));
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

fn toggle_gpu_feature(state: &mut bool, flag: u32) {
    *state = !*state;
    if *state {
        gpu::enable(flag);
    } else {
        gpu::disable(flag);
    }

    dbg!(gpu::is_enabled(gl::MULTISAMPLE)); // doesn't change
    dbg!(gpu::is_enabled(gl::FRAMEBUFFER_SRGB)); // changes but has no effect?
}