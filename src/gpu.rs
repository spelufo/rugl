use crate::math::*;
use gl::types::*;
use std::ffi::{CString};
use std::fs;
use std::mem::{size_of, size_of_val};
use std::ptr;

pub fn setup() {
    unsafe {
        gl::Enable(gl::DEPTH_TEST);
        gl::Enable(gl::CULL_FACE);
        gl::CullFace(gl::FRONT);
        gl::PolygonMode(gl::FRONT, gl::LINE);
        gl::PixelStorei(gl::UNPACK_ALIGNMENT, 1);
    }
}

pub fn clear(r: f32, g: f32, b: f32, a: f32) {
    unsafe {
        gl::ClearColor(r, g, b, a);
        gl::Clear(gl::COLOR_BUFFER_BIT | gl::DEPTH_BUFFER_BIT);
    }
}

#[allow(dead_code)]
pub struct Program {
    id: GLuint,
    vert_shader: GLuint,
    frag_shader: GLuint,
}

impl Program {
    pub fn activate(&self) {
        unsafe { gl::UseProgram(self.id) };
    }

    pub fn from_files(
        vertex_shader_path: &str,
        fragment_shader_path: &str,
    ) -> Result<Self, String> {
        let vsrc = fs::read_to_string(vertex_shader_path).map_err(|e| e.to_string())?;
        let fsrc = fs::read_to_string(fragment_shader_path).map_err(|e| e.to_string())?;
        Self::from_sources(vsrc, fsrc)
    }

    pub fn from_sources(
        vertex_shader_source: String,
        fragment_shader_source: String,
    ) -> Result<Self, String> {
        let id = unsafe { gl::CreateProgram() };
        let vert_shader = unsafe { gl::CreateShader(gl::VERTEX_SHADER) };
        let frag_shader = unsafe { gl::CreateShader(gl::FRAGMENT_SHADER) };
        Self::compile_shader(vert_shader, vertex_shader_source)?;
        Self::compile_shader(frag_shader, fragment_shader_source)?;
        unsafe {
            gl::AttachShader(id, vert_shader);
            gl::AttachShader(id, frag_shader);
            gl::LinkProgram(id);
            gl::UseProgram(id);
        }
        Ok(Self {
            id,
            vert_shader,
            frag_shader,
        })
    }

    pub fn get_uniform(&self, name: &str) -> Result<Uniform, String> {
        unsafe {
            let name = CString::new(name).map_err(|e| e.to_string())?;
            let location = gl::GetUniformLocation(self.id, name.as_ptr() as *const GLchar);
            Ok(Uniform { location })
        }
    }

    pub fn set_uniform<T: UniformValue>(&mut self, uniform: Uniform, value: T) {
        value.set_uniform(uniform);
    }

    fn compile_shader(shader: GLuint, source: String) -> Result<(), String> {
        let src = CString::new(source).map_err(|e| e.to_string())?;
        unsafe {
            gl::ShaderSource(shader, 1, &src.as_ptr(), ptr::null());
            gl::CompileShader(shader);
            let mut status: i32 = 0;
            gl::GetShaderiv(shader, gl::COMPILE_STATUS, &mut status);
            if status != gl::TRUE as i32 {
                let mut log_len: i32 = 0;
                gl::GetShaderiv(shader, gl::INFO_LOG_LENGTH, &mut log_len);
                assert!(log_len >= 0);
                let log_buffer: Vec<u8> = vec![0; log_len as usize + 1];
                let log: CString = CString::from_vec_unchecked(log_buffer);
                gl::GetShaderInfoLog(
                    shader,
                    log_len,
                    ptr::null_mut(),
                    log.as_ptr() as *mut GLchar,
                );
                return Err(log.into_string().unwrap());
            }
        }
        Ok(())
    }
}

#[derive(Copy, Clone)]
#[repr(C)]
pub struct Uniform {
    pub location: i32,
}

pub trait UniformValue {
    fn set_uniform(&self, uniform: Uniform);
}

impl UniformValue for &Matrix4 {
    fn set_uniform(&self, uniform: Uniform) {
        unsafe {
            gl::UniformMatrix4fv(uniform.location, 1, gl::FALSE, self.as_ptr() as *const GLfloat);
        }
    }
}

impl UniformValue for TextureUnit {
    fn set_uniform(&self, uniform: Uniform) {
        unsafe {
            gl::Uniform1i(uniform.location, self.0 as i32);
        }
    }
}


#[derive(Copy, Clone)]
pub enum Attr {
    Position,
    Color,
    TextureCoords,
    Normal,
}

impl Attr {
    pub fn name(self) -> &'static str {
        match self {
            Attr::Position => "a_position",
            Attr::TextureCoords => "a_texture_coords",
            Attr::Normal => "a_normal",
            Attr::Color => "a_color",
        }
    }

    fn location(self) -> GLuint {
        match self {
            Attr::Position => 0,
            Attr::TextureCoords => 1,
            Attr::Normal => 2,
            Attr::Color => 3,
        }
    }
}

#[repr(u32)]
pub enum Type {
    F32 = gl::FLOAT,
    F16 = gl::HALF_FLOAT,
    I32 = gl::INT,
    U32 = gl::UNSIGNED_INT,
    I16 = gl::SHORT,
    U16 = gl::UNSIGNED_SHORT,
    I8 = gl::BYTE,
    U8 = gl::UNSIGNED_BYTE,
    Fixed = gl::FIXED,
    I2_10_10_10Rev = gl::INT_2_10_10_10_REV,
    U2_10_10_10Rev = gl::UNSIGNED_INT_2_10_10_10_REV,
}

pub struct PointerConfig {
    pub type_: Type,
    pub size: usize,
    pub stride: isize,
    pub offset: isize,
}

impl PointerConfig {
    pub fn vector2() -> Self {
        Self {
            type_: Type::F32,
            size: 2,
            stride: 2 * size_of::<f32>() as isize,
            offset: 0,
        }
    }
    pub fn vector3() -> Self {
        Self {
            type_: Type::F32,
            size: 3,
            stride: 3 * size_of::<f32>() as isize,
            offset: 0,
        }
    }
}

#[derive(Debug)]
pub struct VertexArray {
    pub id: u32,
    pub index_buffer_id: u32,
}

impl VertexArray {
    pub fn new() -> Self {
        let mut id: u32 = 0;
        let mut index_buffer_id: u32 = 0;
        unsafe {
            gl::GenVertexArrays(1, &mut id as *mut GLuint);
            gl::BindVertexArray(id);
            gl::GenBuffers(1, &mut index_buffer_id as *mut GLuint);
        }
        Self {
            id,
            index_buffer_id,
        }
    }

    pub fn setup_attribute(&self, attr: Attr, buffer_id: u32, config: PointerConfig) {
        unsafe {
            gl::BindVertexArray(self.id);
            gl::BindBuffer(gl::ARRAY_BUFFER, buffer_id);
            let attr_loc = attr.location();
            gl::VertexAttribPointer(
                attr_loc,
                config.size as GLint,
                config.type_ as GLenum,
                gl::FALSE, // normalized
                config.stride as i32,
                config.offset as *const GLvoid,
            );
            gl::EnableVertexAttribArray(attr_loc);
        }
    }

    pub fn draw(&self, n_indices: usize, offset: isize) {
        unsafe {
            gl::BindVertexArray(self.id);
            gl::DrawElements(
                gl::TRIANGLES,
                n_indices as i32,
                gl::UNSIGNED_INT,
                offset as *const GLvoid,
            );
        }
    }
}

pub fn gen_buffer() -> u32 {
    let mut id: u32 = 0;
    unsafe {
        gl::GenBuffers(1, &mut id as *mut GLuint);
    }
    // TODO: Check failure
    id
}

pub fn load_index_buffer_data<T>(buffer_id: u32, data: &[T]) {
    load_buffer_data_impl(gl::ELEMENT_ARRAY_BUFFER, buffer_id, data);
}

pub fn load_buffer_data<T>(buffer_id: u32, data: &[T]) {
    load_buffer_data_impl(gl::ARRAY_BUFFER, buffer_id, data);
}

fn load_buffer_data_impl<T>(kind: GLenum, buffer_id: u32, data: &[T]) {
    unsafe {
        gl::BindBuffer(kind, buffer_id);
        gl::BufferData(
            kind,
            size_of_val(data) as isize,
            data.as_ptr() as *const GLvoid,
            gl::STATIC_DRAW,
        );
    }
}

#[derive(Copy, Clone)]
pub struct TextureUnit(pub u32);

impl TextureUnit {
    pub fn activate(self, texture: Texture) {
        let TextureUnit(slot) = self;
        assert!(slot < 16);
        unsafe {
            gl::ActiveTexture(gl::TEXTURE0 + slot as GLenum);
            gl::BindTexture(gl::TEXTURE_2D, texture.id);
        }
    }
}


#[derive(Copy, Clone)]
pub struct Texture {
    id: u32,
}

impl Texture {
    pub fn new() -> Texture {
        let mut id: u32 = 0;
        unsafe {
            gl::GenTextures(1, &mut id as *mut GLuint);
        }
        // TODO: Check failure
        let mut texture = Texture { id };
        texture.set_min_filter_mode(TextureMinFilterMode::Linear);
        texture.set_mag_filter_mode(TextureMagFilterMode::Nearest);
        // texture.set_s_wrap_mode(TextureWrapMode::ClampToEdge);
        // texture.set_t_wrap_mode(TextureWrapMode::ClampToEdge);
        texture
    }

    pub fn load_data<T>(&mut self, width: i32, height: i32, data: &[T]) {
        // TODO: Other texture formats.
        unsafe {
            gl::BindTexture(gl::TEXTURE_2D, self.id);
            gl::TexImage2D(
                gl::TEXTURE_2D,
                0,
                gl::RGB as i32,
                width as GLsizei,
                height as GLsizei,
                0,
                gl::RGB,
                gl::UNSIGNED_BYTE,
                data.as_ptr() as *const GLvoid
            );
        }
    }

    pub fn set_s_wrap_mode(&mut self, mode: TextureWrapMode) {
        unsafe {
            gl::BindTexture(gl::TEXTURE_2D, self.id);
            gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_WRAP_S, mode as i32);
        }
    }

    pub fn set_t_wrap_mode(&mut self, mode: TextureWrapMode) {
        unsafe {
            gl::BindTexture(gl::TEXTURE_2D, self.id);
            gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_WRAP_T, mode as i32);
        }
    }

    pub fn set_min_filter_mode(&mut self, mode: TextureMinFilterMode) {
        unsafe {
            gl::BindTexture(gl::TEXTURE_2D, self.id);
            gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MIN_FILTER, mode as i32);
        }
    }

    pub fn set_mag_filter_mode(&mut self, mode: TextureMagFilterMode) {
        unsafe {
            gl::BindTexture(gl::TEXTURE_2D, self.id);
            gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MAG_FILTER, mode as i32);
        }
    }

    // TODO: Mipmaps.
    // pub fn make_mipmaps() {}
}

#[repr(u32)]
pub enum TextureWrapMode {
    Repeat = gl::REPEAT,
    ClampToEdge = gl::CLAMP_TO_EDGE,
    MirroredRepeat = gl::MIRRORED_REPEAT,
}

#[repr(u32)]
pub enum TextureMinFilterMode {
    Nearest = gl::NEAREST,
    Linear = gl::LINEAR,
    NearestMipmapNearest = gl::NEAREST_MIPMAP_NEAREST,
    LinearMipmapNearest = gl::LINEAR_MIPMAP_NEAREST,
    NearestMipmapLinear = gl::NEAREST_MIPMAP_LINEAR,
    LinearMipmapLinear = gl::LINEAR_MIPMAP_LINEAR,
}

#[repr(u32)]
pub enum TextureMagFilterMode {
    Nearest = gl::NEAREST,
    Linear = gl::LINEAR,
}
