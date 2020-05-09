use crate::math::*;

pub struct Camera {
    pub position: Vector3,
    pub orientation: Quaternion,
    pub aspect_ratio: f32
    // pub target: Vector3,
    // pub world_up: Vector3,
}

impl Camera {
    pub fn view_matrix(&self) -> Matrix4 {
        // V = inv(Cam_translate * Cam_rotate) = inv(Cam_rotate) * inv(Cam_translate)
        self.orientation.conj().rotation_matrix() * Matrix4::translate(-self.position)
    }

    pub fn projection_matrix(&self) -> Matrix4 {
        Matrix4::perspective(FRAC_PI_3, self.aspect_ratio, 0.5, 10.0)
    }

    pub fn view_projection_matrix(&self) -> Matrix4 {
        self.projection_matrix() * self.view_matrix()
    }

    pub fn rotate(&mut self, axis: Vector3, angle: f32) {
        self.orientation = Quaternion::rotation(axis, angle) * self.orientation;
    }
}
