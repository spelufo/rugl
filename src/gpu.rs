use gl::types::*;
use std::ffi::CString;
use std::fs;
use std::mem::{size_of, size_of_val};
use std::ptr;

pub fn clear(r: f32, g: f32, b: f32, a: f32) {
    unsafe {
        gl::ClearColor(r, g, b, a);
        gl::Clear(gl::COLOR_BUFFER_BIT);
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

    // get attrib location

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
pub enum Attr {
    Position,
    Color,
    TextureCoords,
    Normal,
}

impl Attr {
    fn name(self) -> &'static str {
        match self {
            Attr::Position => "a_position",
            Attr::Color => "a_color",
            Attr::TextureCoords => "a_texture_coords",
            Attr::Normal => "a_normal",
        }
    }

    fn location(self) -> GLuint {
        match self {
            Attr::Position => 1,
            Attr::Color => 2,
            Attr::TextureCoords => 3,
            Attr::Normal => 4,
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

    pub fn draw(&self, n_indices: i32, offset: i32) {
        unsafe {
            gl::BindVertexArray(self.id);
            gl::DrawElements(
                gl::TRIANGLES,
                n_indices,
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
