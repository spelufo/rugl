use crate::gpu;
use crate::gpu::{Attr, PointerConfig};
use crate::math::*;

// TODO: Remove. Used for debugging. Should be 1 for pixel perfect rendering.
const TEXTURE_MAGNIFICATION: f32 = 1.0;

pub fn init_library() -> Result<freetype::Library, freetype::Error> {
    freetype::Library::init()
}

pub struct Font<'a> {
    _ft: &'a freetype::Library,
    face: freetype::Face,
    size_px: u32,
}

impl<'a> Font<'a> {
    pub fn open(ft: &'a freetype::Library, path: &str, font_size_px: u32) -> Result<Font<'a>, freetype::Error> {
        let face = ft.new_face(path, 0)?;
        let size_px = ((font_size_px as f32) * TEXTURE_MAGNIFICATION) as u32;
        face.set_pixel_sizes(0, size_px)?;
        Ok(Font {
            _ft: ft,
            face,
            size_px,
        })
    }

    pub fn make_atlas(&mut self) -> Result<Atlas, freetype::Error> {
        Atlas::new(self)
    }

    pub fn load_char(&mut self, c: char, load_flags: freetype::face::LoadFlag) -> Result<freetype::GlyphSlot, freetype::Error> {
        self.face.load_char(c as usize, load_flags)?;
        Ok(*self.face.glyph())
    }

    pub fn get_kerning(&mut self, c0: char, c1: char) -> Result<freetype::Vector, freetype::Error> {
        let i0 = self.face.get_char_index(c0 as usize);
        let i1 = self.face.get_char_index(c1 as usize);
        self.face.get_kerning(i0, i1, freetype::face::KerningMode::KerningDefault)
    }

    pub fn char_bitmap(&mut self, c: char) -> Result<freetype::Bitmap, freetype::Error> {
        let glyph = self.load_char(c, freetype::face::LoadFlag::RENDER)?;
        Ok(glyph.bitmap())
    }
}

pub struct Atlas {
    image: image::GrayImage,
    tex_coords: [RectangleI; 128],
    metrics: [freetype::GlyphMetrics; 128],
}

impl Atlas {
    pub fn new(font: &mut Font) -> Result<Atlas, freetype::Error> {
        let mut width = 0;
        let mut height = 0;
        let mut tex_coords = [RectangleI::ZERO; 128];
        let mut metrics: [freetype::GlyphMetrics; 128] = unsafe { std::mem::zeroed() };

        let printable_ascii: std::ops::Range<u8> = 0x20..0x7e;
        for c in printable_ascii.clone() {
            if let Ok(glyph) = font.load_char(c as char, freetype::face::LoadFlag::DEFAULT) {
                let glpyh_metrics = glyph.metrics();
                metrics[c as usize] = glpyh_metrics;
                let w = fixed64_round(glpyh_metrics.width);
                let h = fixed64_round(glpyh_metrics.height);
                tex_coords[c as usize] = RectangleI::new(Vector2I::new(width, 0), w, h);
                width += w;
                height = height.max(h);
            } else {
                println!("Atlas::new: unable to load glyph for char {:?}", c)
            }
        }

        let mut image = match image::DynamicImage::new_luma8(width as u32, height as u32) {
            image::DynamicImage::ImageLuma8(image) => image,
            _ => unreachable!(),
        };

        for c in printable_ascii {
            if let Ok(glyph) = font.load_char(c as char, freetype::face::LoadFlag::RENDER) {
                let bounds = tex_coords[c as usize];
                let bitmap = glyph.bitmap();
                assert!(bitmap.rows() <= bounds.height() && bitmap.width() <= bounds.width());
                let bitmap_pitch = bitmap.pitch() as usize;
                let bitmap_buffer = bitmap.buffer();
                for y in bounds.min.y .. bounds.max.y {
                    for x in bounds.min.x .. bounds.max.x {
                        let yr = (y - bounds.min.y) as usize;
                        let xr = (x - bounds.min.x) as usize;
                        let i: usize = bitmap_pitch * yr + xr;
                        if i < bitmap_buffer.len() {
                            image.put_pixel(x as u32, y as u32, image::Luma([bitmap_buffer[i]]));
                        }
                    }
                }
            }
        }

        Ok(Atlas { image, tex_coords, metrics })
    }

    pub fn width(&self) -> i32 {
        self.image.width() as i32
    }

    pub fn height(&self) -> i32 {
        self.image.height() as i32
    }

    pub fn image_data(&self) -> &[u8] {
        &self.image  // TODO: Check. Does this yield a slice of the image data? It seems to work.
    }

    pub fn tex_coords(&self, c: char) -> Option<Rectangle> {
        let i = c as usize;
        if i >= 128 || self.tex_coords[i] == RectangleI::ZERO {
            None
        } else {
            let width = self.width() as f32;
            let height = self.height() as f32;
            let r = self.tex_coords[i];
            Some(Rectangle {
                min: Vector2::new(r.min.x as f32 / width, r.min.y as f32 / height),
                max: Vector2::new(r.max.x as f32 / width, r.max.y as f32 / height),
            })
        }
    }

    pub fn metrics(&self, c: char) -> freetype::GlyphMetrics {
        self.metrics[c as usize]
    }
}


#[derive(Debug)]
pub struct Text {
    positions: Vec<Vector2>,
    tex_coords: Vec<Vector2>,
    indices: Vec<u32>,
    texture: gpu::Texture,
    vertex_array: gpu::VertexArray,
}

impl Text {
    pub fn new(s: &str, position: Vector2, font: &mut Font, atlas: &Atlas) -> Text {

        let mut texture = gpu::Texture::new();
        texture.load_data(gpu::TextureFormat::Alpha, atlas.width(), atlas.height(), atlas.image_data());

        let mut positions: Vec<Vector2> = Vec::new();
        let mut tex_coords: Vec<Vector2> = Vec::new();
        let mut indices: Vec<u32> = Vec::new();
        let mut pen = position;

        let scale = 1.0 / TEXTURE_MAGNIFICATION;
        let mut i = 0;
        let mut last_char: Option<char> = None;
        for c in s.chars() {
            if let Some(uvs) = atlas.tex_coords(c) {
                indices.extend_from_slice(&[i, i+1, i+3, i+3, i+1, i+2]);
                let metrics = atlas.metrics(c);
                let w = scale * fixed64_to_f32(metrics.width);
                let h = scale * fixed64_to_f32(metrics.height);
                let top_left = pen + scale * Vector2::new(
                    fixed64_to_f32(metrics.horiBearingX),
                    -fixed64_to_f32(metrics.horiBearingY),
                );
                positions.push(top_left);
                positions.push(top_left + Vector2::new(w, 0.));
                positions.push(top_left + Vector2::new(w, h));
                positions.push(top_left + Vector2::new(0., h));
                tex_coords.push(uvs.min);
                tex_coords.push(Vector2::new(uvs.max.x, uvs.min.y));
                tex_coords.push(uvs.max);
                tex_coords.push(Vector2::new(uvs.min.x, uvs.max.y));
                pen.x += scale * fixed64_to_f32(metrics.horiAdvance);
                if let Some(last_char) = last_char {
                    if let Ok(kerning) = font.get_kerning(last_char, c) {
                        pen.x += fixed64_to_f32(kerning.x)
                    }
                }
                i += 4;
                last_char = Some(c);
            } else {
                if c == ' ' {
                    pen.x += font.size_px as f32 / 3.0;  // TODO: Text layout.
                }
                last_char = None;
            }
        }
        let vertex_array = gpu::VertexArray::new();
        let text = Text {
            positions,
            tex_coords,
            indices,
            texture,
            vertex_array,
        };
        text.setup_attributes();
        text
    }

    pub fn setup_attributes(&self) {
        // configure attributes
        let pos_buf_id = gpu::gen_buffer();
        let tex_buf_id = gpu::gen_buffer();
        self.vertex_array.setup_attribute(Attr::Position, pos_buf_id, PointerConfig::vector2());
        self.vertex_array.setup_attribute(Attr::TextureCoords, tex_buf_id, PointerConfig::vector2());

        // load buffers data
        gpu::load_index_buffer_data(self.vertex_array.index_buffer_id, &self.indices[..]);
        gpu::load_buffer_data(pos_buf_id, &self.positions[..]);
        gpu::load_buffer_data(tex_buf_id, &self.tex_coords[..]);
    }

    pub fn texture(&self) -> gpu::Texture {
        self.texture
    }

    pub fn draw(&self) {
        self.vertex_array.draw(self.indices.len(), 0);
    }
}



pub struct TextShader {
    program: gpu::Program,
    texture_uniform: gpu::Uniform,
    screen_size_uniform: gpu::Uniform,
}

impl TextShader {
    pub fn new() -> Result<Self, String> {
        let program = gpu::Program::from_files("shaders/text_vert.glsl", "shaders/text_frag.glsl")?;
        let texture_uniform = program.get_uniform("texture0")?;
        let screen_size_uniform = program.get_uniform("screen_size")?;
        Ok(TextShader {
            program,
            texture_uniform,
            screen_size_uniform,
        })
    }

    pub fn set_texture(&mut self, texture: gpu::Texture) {
        self.program.activate();
        let texture_unit = gpu::TextureUnit(0);
        texture_unit.bind_texture(texture);
        self.program.set_uniform(self.texture_uniform, texture_unit);
    }

    pub fn set_screen_size(&mut self, screen_size: Vector2) {
        self.program.activate();
        self.program.set_uniform(self.screen_size_uniform, screen_size);
    }

    pub fn draw(&mut self, text: &Text) {
        self.set_texture(text.texture());
        self.program.activate();
        text.draw();
    }
}

fn fixed64_round(x: freetype::freetype_sys::FT_Pos) -> i32 {
    let mut res = x as i32 >> 6;
    if (x & 63) != 0 {
        res += 1
    }
    res
}

fn fixed64_to_f32(x: freetype::freetype_sys::FT_Pos) -> f32 {
    // truncates with precision 0.25
    (x >> 4) as f32 / 4.0
}
