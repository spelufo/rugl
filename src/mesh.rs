use crate::gpu;
use crate::gpu::{Attr, PointerConfig};
use crate::math::*;

#[derive(Debug)]
pub struct Mesh {
    positions: Vec<Vector3>,
    colors: Vec<Vector3>,
    // tex_coords: Vec<Vector2>,
    // normals: Vec<Vector3>,
    indices: Vec<u32>,
    vertex_array: gpu::VertexArray,
}

impl Mesh {
    pub fn new_cube() -> Self {
        let indices = vec![
            6, 3, 1, 6, 1, 4, // -x
            2, 7, 5, 2, 5, 0, // +x
            3, 2, 0, 3, 0, 1, // -y
            7, 6, 4, 7, 4, 5, // +y
            0, 5, 4, 0, 4, 1, // -z
            3, 6, 7, 3, 7, 2, // +z
        ];

        let positions = vec![
            Vector3( 0.5, -0.5, -0.5),
            Vector3(-0.5, -0.5, -0.5),
            Vector3( 0.5, -0.5,  0.5),
            Vector3(-0.5, -0.5,  0.5),
            Vector3(-0.5,  0.5, -0.5),
            Vector3( 0.5,  0.5, -0.5),
            Vector3(-0.5,  0.5,  0.5),
            Vector3( 0.5,  0.5,  0.5),
        ];

        let colors = vec![
            Vector3( 0.5,  0.0,  0.0),
            Vector3( 0.0,  0.5,  0.0),
            Vector3( 0.0,  0.0,  0.5),
            Vector3( 0.5,  0.5,  0.0),
            Vector3( 0.5,  0.0,  0.5),
            Vector3( 0.0,  0.5,  0.5),
            Vector3( 0.5,  0.5,  0.5),
            Vector3( 0.0,  0.0,  0.0),
        ];


        let vertex_array = gpu::VertexArray::new();

        let mesh = Self {
            positions,
            colors,
            indices,
            vertex_array,
        };

        // configure attributes
        let pos_buf_id = gpu::gen_buffer();
        let col_buf_id = gpu::gen_buffer();
        mesh.vertex_array
            .setup_attribute(Attr::Position, pos_buf_id, PointerConfig::vector3());
        mesh.vertex_array
            .setup_attribute(Attr::Color, col_buf_id, PointerConfig::vector3());

        // load buffers data
        gpu::load_index_buffer_data(mesh.vertex_array.index_buffer_id, &mesh.indices[..]);
        gpu::load_buffer_data(pos_buf_id, &mesh.positions[..]);
        gpu::load_buffer_data(col_buf_id, &mesh.colors[..]);

        mesh
    }

    pub fn draw(&self) {
        self.vertex_array.draw(self.indices.len(), 0);
    }
}
