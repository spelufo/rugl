vector_space!(Vector2 f32 {
    x: f32,
    y: f32
});

pub fn vec2(x: f32, y: f32) -> Vector2 {
    Vector2 { x, y }
}

impl Vector2 {
    pub const _0: Vector2 = Vector2 { x: 0., y: 0. };
    pub const X: Vector2 = Vector2 { x: 1., y: 0. };
    pub const Y: Vector2 = Vector2 { x: 0., y: 1. };
}


vector_space!(Vector3 f32 {
    x: f32,
    y: f32,
    z: f32
});

pub fn vec3(x: f32, y: f32, z: f32) -> Vector3 {
    Vector3 { x, y, z }
}

impl Vector3 {
    pub const _0: Vector3 = Vector3 { x: 0., y: 0., z: 0. };
    pub const X: Vector3 = Vector3 { x: 1., y: 0., z: 0. };
    pub const Y: Vector3 = Vector3 { x: 0., y: 1., z: 0. };
    pub const Z: Vector3 = Vector3 { x: 0., y: 0., z: 1. };

    pub fn cross(v: Vector3, w: Vector3) -> Vector3 {
        Vector3 {
            x: v.y * w.z - v.z * w.y,
            y: v.z * w.x - v.x * w.z,
            z: v.x * w.y - v.y * w.x,
        }
    }
}
