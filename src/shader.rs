use crate::gpu;
use crate::math::*;
use crate::mesh::Mesh;


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
