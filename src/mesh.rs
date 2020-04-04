use crate::gpu;
use crate::gpu::{Attr, PointerConfig};
use crate::math::*;

#[derive(Debug)]
pub struct Mesh {
    positions: Vec<Vector3>,
    // colors: Vec<Vector3>,
    // normals: Vec<Vector3>,
    // tex_coords: Vec<Vector2>,
    indices: Vec<u32>,
    vertex_array: gpu::VertexArray,
}

impl Mesh {
    pub fn new_cube() -> Self {
        let positions = vec![
            Vector3(-0.5, -0.5, -1.0),
            Vector3(-0.5, 0.5, -1.0),
            Vector3(0.5, -0.5, -1.0),
            Vector3(0.5, 0.5, -1.0),
            Vector3(-0.5, -0.5, 1.0),
            Vector3(-0.5, 0.5, 1.0),
            Vector3(0.5, -0.5, 1.0),
            Vector3(0.5, 0.5, 1.0),
        ];

        let indices = vec![1, 3, 0, 0, 3, 2];

        let vertex_array = gpu::VertexArray::new();

        let mesh = Self {
            positions,
            indices,
            vertex_array,
        };

        println!("{:?}", mesh);

        // configure attributes
        let pos_buf_id = gpu::gen_buffer();
        mesh.vertex_array
            .setup_attribute(Attr::Position, pos_buf_id, PointerConfig::vector3());

        // load buffers data
        gpu::load_index_buffer_data(mesh.vertex_array.index_buffer_id, &mesh.indices[..]);
        gpu::load_buffer_data(pos_buf_id, &mesh.positions[..]);

        mesh
    }

    pub fn draw(&self) {
        self.vertex_array.draw(6, 0);
    }
}
