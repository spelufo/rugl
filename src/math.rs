#[derive(Debug, Copy, Clone)]
pub struct Vector2(pub f32, pub f32);

#[derive(Debug, Copy, Clone)]
pub struct Vector3(pub f32, pub f32, pub f32);

#[derive(Debug, Copy, Clone)]
pub struct Frame3 {
    pub i: Vector3,
    pub j: Vector3,
    pub k: Vector3,
}
