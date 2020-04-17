use glfw::*;

use crate::camera::*;
use crate::math::*;

pub struct CameraController {
    pub active: bool,
    last_cursor_pos: Option<Vector2>,
}

impl CameraController {
    pub fn new() -> CameraController {
        CameraController {
            active: false,
            last_cursor_pos: None,
        }
    }

    pub fn activate(&mut self, window: &mut Window) {
        window.set_cursor_mode(glfw::CursorMode::Disabled);
        self.active = true;
        self.last_cursor_pos = None;
    }

    pub fn deactivate(&mut self, window: &mut Window) {
        window.set_cursor_mode(glfw::CursorMode::Normal);
        self.active = false;
        self.last_cursor_pos = None;
    }
    pub fn update(&mut self, camera: &mut Camera, window: &Window) {
        if !self.active {
            return;
        }

        let dir_up = Vector3::Z;
        let dir_forward = -camera.orientation.frame_z();
        let dir_right = camera.orientation.frame_x();

        // update camera.position
        let camera_speed = 0.1;
        let mut camera_v_dir = Vector3::_0;
        if window.get_key(Key::W) == glfw::Action::Press {
            camera_v_dir += dir_forward;
        }
        if window.get_key(Key::S) == glfw::Action::Press {
            camera_v_dir -= dir_forward;
        }
        if window.get_key(Key::A) == glfw::Action::Press {
            camera_v_dir -= dir_right;
        }
        if window.get_key(Key::D) == glfw::Action::Press {
            camera_v_dir += dir_right;
        }
        if window.get_key(Key::Q) == glfw::Action::Press {
            camera_v_dir += dir_up;
        }
        if window.get_key(Key::E) == glfw::Action::Press {
            camera_v_dir -= dir_up;
        }
        let camera_v = camera_speed * camera_v_dir;
        camera.position += camera_v;

        // update camera.orientation
        let (x, y) = window.get_cursor_pos();
        let cursor_pos = vec2(x as f32, y as f32);
        if let Some(last_cursor_pos) = self.last_cursor_pos {
            let cursor_off = cursor_pos - last_cursor_pos;
            camera.rotate(dir_right, -0.005 * cursor_off.y);
            camera.rotate(dir_up, -0.005 * cursor_off.x);
        }
        self.last_cursor_pos = Some(cursor_pos);
    }
}
