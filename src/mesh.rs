use crate::gpu;
use crate::gpu::{Attr, PointerConfig};
use crate::math::*;

#[derive(Debug)]
pub struct Mesh {
    positions: Vec<Vector3>,
    tex_coords: Vec<Vector2>,
    normals: Vec<Vector3>,
    indices: Vec<u32>,
    vertex_array: gpu::VertexArray,
}

impl Mesh {
    pub fn load_obj(source: &str) -> Mesh {
        let mut obj_positions = Vec::new();
        let mut obj_normals = Vec::new();
        let mut obj_tex_coords = Vec::new();

        for line in source.lines() {
            let mut args = line.split_whitespace();
            if let Some(cmd) = args.next() {
                match cmd {
                    "v" | "vn" => {
                        let x = args.next().unwrap().parse::<f32>().unwrap();
                        let y = args.next().unwrap().parse::<f32>().unwrap();
                        let z = args.next().unwrap().parse::<f32>().unwrap();
                        let buf = match cmd {
                            "v" => &mut obj_positions,
                            "vn" => &mut obj_normals,
                            _ => unreachable!(),
                        };
                        buf.push(Vector3::new(x, y, z));
                    }
                    "vt" => {
                        let x = args.next().unwrap().parse::<f32>().unwrap();
                        let y = args.next().unwrap().parse::<f32>().unwrap();
                        obj_tex_coords.push(Vector2::new(x, y));
                    }
                    _ => continue,
                }
            }
        }

        let mut positions: Vec<Vector3> = Vec::new();
        let mut normals: Vec<Vector3> = Vec::new();
        let mut tex_coords: Vec<Vector2> = Vec::new();
        let mut indices: Vec<u32> = Vec::new();

        for line in source.lines() {
            let mut args = line.split_whitespace();
            if let Some("f") = args.next() {
                let mut vertex_count = 0;
                for vertex_str in args {
                    vertex_count += 1;
                    let v_idxs: Vec<Option<usize>> = vertex_str
                        .split("/")
                        .map(|s| s.parse::<usize>().ok())
                        .collect();
                    assert!(v_idxs.len() == 3);
                    positions.push(v_idxs[0].map_or(Vector3::ZERO, |i| obj_positions[i - 1]));
                    tex_coords.push(v_idxs[1].map_or(Vector2::ZERO, |i| obj_tex_coords[i - 1]));
                    normals.push(v_idxs[2].map_or(Vector3::ZERO, |i| obj_normals[i - 1]));
                }
                let i0 = indices.len() as u32;
                for i in 2..vertex_count {
                    indices.push(i0);
                    indices.push(i0 + i);
                    indices.push(i0 + i - 1);
                }
            }
        }

        let vertex_array = gpu::VertexArray::new();
        let mesh = Mesh {
            positions,
            tex_coords,
            normals,
            indices,
            vertex_array,
        };
        mesh.setup_attributes();
        mesh
    }

    pub fn new_quad(width: f32, height: f32) -> Self {
        let indices = vec![
            0, 1, 3, 3, 1, 2,
            7, 5, 4, 7, 6, 5,
        ];
        let positions = vec![
            Vector3::new(0.0, -0.5*width,  0.5*height),
            Vector3::new(0.0,  0.5*width,  0.5*height),
            Vector3::new(0.0,  0.5*width, -0.5*height),
            Vector3::new(0.0, -0.5*width, -0.5*height),
            Vector3::new(0.0, -0.5*width,  0.5*height),
            Vector3::new(0.0,  0.5*width,  0.5*height),
            Vector3::new(0.0,  0.5*width, -0.5*height),
            Vector3::new(0.0, -0.5*width, -0.5*height),
        ];
        let tex_coords = vec![
            Vector2::new(0.0, 0.0),
            Vector2::new(1.0, 0.0),
            Vector2::new(1.0, 1.0),
            Vector2::new(0.0, 1.0),
            Vector2::new(0.0, 0.0),
            Vector2::new(1.0, 0.0),
            Vector2::new(1.0, 1.0),
            Vector2::new(0.0, 1.0),
        ];
        let normals = vec![
            Vector3::new(0., 0., 1.),
            Vector3::new(0., 0., 1.),
            Vector3::new(0., 0., 1.),
            Vector3::new(0., 0., 1.),
            Vector3::new(0., 0., -1.),
            Vector3::new(0., 0., -1.),
            Vector3::new(0., 0., -1.),
            Vector3::new(0., 0., -1.),
        ];

        let vertex_array = gpu::VertexArray::new();
        let mesh = Mesh {
            positions,
            tex_coords,
            normals,
            indices,
            vertex_array,
        };
        mesh.setup_attributes();
        mesh
    }

    pub fn new_cube() -> Self {
        let indices = vec![
            6, 3, 1, 6, 1, 4, // -x
            2, 7, 5, 2, 5, 0, // +x
            11, 10, 8, 11, 8, 9, // -y
            15, 14, 12, 15, 12, 13, // +y
            16, 21, 20, 16, 20, 17, // -z
            19, 22, 23, 19, 23, 18, // +z
        ];
        let positions = vec![
            Vector3::new(0.5, -0.5, -0.5),
            Vector3::new(-0.5, -0.5, -0.5),
            Vector3::new(0.5_, -0.5, 0.5),
            Vector3::new(-0.5, -0.5, 0.5),
            Vector3::new(-0.5, 0.5, -0.5),
            Vector3::new(0.5_, 0.5, -0.5),
            Vector3::new(-0.5, 0.5, 0.5),
            Vector3::new(0.5_, 0.5, 0.5),
            Vector3::new(0.5, -0.5, -0.5),
            Vector3::new(-0.5, -0.5, -0.5),
            Vector3::new(0.5, -0.5, 0.5),
            Vector3::new(-0.5, -0.5, 0.5),
            Vector3::new(-0.5, 0.5, -0.5),
            Vector3::new(0.5, 0.5, -0.5),
            Vector3::new(-0.5, 0.5, 0.5),
            Vector3::new(0.5, 0.5, 0.5),
            Vector3::new(0.5, -0.5, -0.5),
            Vector3::new(-0.5, -0.5, -0.5),
            Vector3::new(0.5, -0.5, 0.5),
            Vector3::new(-0.5, -0.5, 0.5),
            Vector3::new(-0.5, 0.5, -0.5),
            Vector3::new(0.5, 0.5, -0.5),
            Vector3::new(-0.5, 0.5, 0.5),
            Vector3::new(0.5, 0.5, 0.5),
        ];
        let tex_coords = vec![Vector2::new(0.0, 0.0); 24];
        let normals = vec![
            Vector3::new(1., 0., 0.),
            Vector3::new(-1., 0., 0.),
            Vector3::new(1., 0., 0.),
            Vector3::new(-1., 0., 0.),
            Vector3::new(-1., 0., 0.),
            Vector3::new(1., 0., 0.),
            Vector3::new(-1., 0., 0.),
            Vector3::new(1., 0., 0.),
            Vector3::new(0., -1., 0.),
            Vector3::new(0., -1., 0.),
            Vector3::new(0., -1., 0.),
            Vector3::new(0., -1., 0.),
            Vector3::new(0., 1., 0.),
            Vector3::new(0., 1., 0.),
            Vector3::new(0., 1., 0.),
            Vector3::new(0., 1., 0.),
            Vector3::new(0., 0., -1.),
            Vector3::new(0., 0., -1.),
            Vector3::new(0., 0., 1.),
            Vector3::new(0., 0., 1.),
            Vector3::new(0., 0., -1.),
            Vector3::new(0., 0., -1.),
            Vector3::new(0., 0., 1.),
            Vector3::new(0., 0., 1.),
        ];

        let vertex_array = gpu::VertexArray::new();
        let mesh = Mesh {
            positions,
            tex_coords,
            normals,
            indices,
            vertex_array,
        };
        mesh.setup_attributes();
        mesh
    }

    pub fn setup_attributes(&self) {
        // configure attributes
        let pos_buf_id = gpu::gen_buffer();
        let tex_buf_id = gpu::gen_buffer();
        let nor_buf_id = gpu::gen_buffer();
        self.vertex_array
            .setup_attribute(Attr::Position, pos_buf_id, PointerConfig::vector3());
        self.vertex_array.setup_attribute(
            Attr::TextureCoords,
            tex_buf_id,
            PointerConfig::vector2(),
        );
        self.vertex_array
            .setup_attribute(Attr::Normal, nor_buf_id, PointerConfig::vector3());

        // load buffers data
        gpu::load_index_buffer_data(self.vertex_array.index_buffer_id, &self.indices[..]);
        gpu::load_buffer_data(pos_buf_id, &self.positions[..]);
        gpu::load_buffer_data(tex_buf_id, &self.tex_coords[..]);
        gpu::load_buffer_data(nor_buf_id, &self.normals[..]);
    }

    pub fn draw(&self) {
        self.vertex_array.draw(self.indices.len(), 0);
    }
}


pub struct MeshShader {
    program: gpu::Program,
    view_projection_uniform: gpu::Uniform,
    model_transform_uniform: gpu::Uniform,
}


impl MeshShader {
    pub fn new(fragment_shader_path: &str) -> Result<Self, String> {
        let program = gpu::Program::from_files("shaders/mesh_vert.glsl", fragment_shader_path)?;
        let view_projection_uniform = program.get_uniform("T_view_projection")?;
        let model_transform_uniform = program.get_uniform("T_model")?;
        Ok(MeshShader {
            program,
            view_projection_uniform,
            model_transform_uniform,
        })
    }

    pub fn get_uniform(&self, name: &str) -> Result<gpu::Uniform, String> {
        self.program.get_uniform(name)
    }

    pub fn set_uniform<T: gpu::UniformValue>(&mut self, uniform: gpu::Uniform, value: T) {
        self.program.activate();
        value.set_uniform(uniform);
    }

    pub fn set_texture(&mut self, uniform: gpu::Uniform, texture_unit: gpu::TextureUnit, texture: gpu::Texture) {
        texture_unit.bind_texture(texture);
        self.set_uniform(uniform, texture_unit);
    }

    pub fn set_view_matrix(&mut self, view_projection_transform: &Matrix4) {
        self.program.activate();
        self.program.set_uniform(self.view_projection_uniform, view_projection_transform);
    }

    pub fn set_model_transform(&mut self, model_transform: &Matrix4) {
        self.program.activate();
        self.program.set_uniform(self.model_transform_uniform, model_transform);
    }

    pub fn draw(&mut self, mesh: &Mesh) {
        self.program.activate();
        mesh.draw();
    }
}
