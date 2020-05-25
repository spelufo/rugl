use crate::gpu;
use crate::gpu::{Attr, PointerConfig};
use crate::math::*;
use std::borrow::Borrow;
use std::fmt;
use std::ops::RangeInclusive;


// No longer used. Text is drawn at the size it is rasterized. This was used to test scaling.
// const UPSCALE_POWER: i32 = 0;
// const NUM_GLYPH_RASTERIZATIONS_POWER: i32 = 0;
// const PRECISION: i32 = NUM_GLYPH_RASTERIZATIONS_POWER - UPSCALE_POWER;


pub fn init_library() -> Result<freetype::Library, freetype::Error> {
    freetype::Library::init()
}

pub struct Font<'a> {
    face: Face<'a>,
    atlas: Atlas,
}

impl<'a> Font<'a> {
    pub fn open(path: &str, font_size_px: u32, ft: &'a freetype::Library) -> Result<Font<'a>, freetype::Error> {
        let face = Face::open(path, /* (1 << UPSCALE_POWER) as u32 * */ font_size_px, ft)?;
        let atlas = Atlas::new();
        let mut font = Font { face, atlas };
        font.load_page(0);
        Ok(font)
    }

    pub fn load_page(&mut self, page_num: u32) {
        self.atlas.load_page(page_num, &self.face);
    }

    pub fn glyph(&self, c: char) -> Option<&AtlasGlyph> {
        self.atlas.glyph(c)
    }

    pub fn texture(&self, page_num: u32) -> Option<gpu::Texture> {
        self.atlas.texture(page_num)
    }

    pub fn tex_coords(&self, c: char) -> Option<Rectangle> {
        self.atlas.tex_coords(c)
    }

    pub fn size_px(&self) -> u32 {
        self.face.size_px
    }

    pub fn kerning(&self, c0: char, c1: char) -> Result<Vector2I, freetype::Error> {
        self.face.kerning(c0, c1)
    }
}


pub struct Face<'a> {
    size_px: u32,
    face: freetype::Face,
    _ft: &'a freetype::Library,
}

impl<'a> Face<'a> {
    pub fn open(path: &str, size_px: u32, ft: &'a freetype::Library) -> Result<Face<'a>, freetype::Error> {
        let face = ft.new_face(path, 0)?;
        face.set_pixel_sizes(0, size_px)?;
        Ok(Face { size_px, face, _ft: ft })
    }

    pub fn load_char(&self, c: char, load_flags: freetype::face::LoadFlag) -> Result<freetype::GlyphSlot, freetype::Error> {
        self.face.load_char(c as usize, load_flags)?;
        Ok(*self.face.glyph())
    }

    pub fn kerning(&self, c0: char, c1: char) -> Result<Vector2I, freetype::Error> {
        let i0 = self.face.get_char_index(c0 as usize);
        let i1 = self.face.get_char_index(c1 as usize);
        let kerning_ft = self.face.get_kerning(i0, i1, freetype::face::KerningMode::KerningDefault)?;
        Ok(Vector2I::new(kerning_ft.x as i32, kerning_ft.y as i32))
    }
}

pub struct Atlas {
    // Atlas covers the first 64k unicode codepoints, the basic multilingual plane.
    // Page i, if present, has data for codepoints [256*i, 256*(i+1)).
    pages: [Option<Box<AtlasPage>>; 256],
}

impl Default for Atlas {
    fn default() -> Self {
        let mut x: Self = unsafe { std::mem::zeroed() };
        for i in 0..256 {
            x.pages[i] = None;
        }
        x
    }
}

impl fmt::Debug for Atlas {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Atlas {{ pages: [\n")?;
        for i in 0..256 {
            if let Some(page) = &self.pages[i] {
                write!(f, "    {}: {:?},\n", i, page)?;
            }
        }
        write!(f, "] }}")
    }
}

pub struct AtlasPage {
    texture: gpu::Texture,
    width: i32,
    height: i32,
    glyphs: [Option<Box<AtlasGlyph>>; 256],
}

impl Default for AtlasPage {
    fn default() -> Self {
        let mut x: Self = unsafe { std::mem::zeroed() };
        for i in 0..256 {
            x.glyphs[i] = None;
        }
        x
    }
}

impl fmt::Debug for AtlasPage {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "AtlasPage {{ texture: {:?}, width: {}, height: {}, glyphs: ({} glyphs...) }}",
            self.texture, self.width, self.height, self.glyphs.iter().filter(|g| g.is_some()).count()
        )
    }
}

#[derive(Copy, Clone, Default, Debug)]
pub struct AtlasGlyph {
    tex_coords: RectangleI,  // px
    size: Vector2I,  // 26.6 px
    horizontal_bearing: Vector2I,  // 26.6 px
    vertical_bearing: Vector2I,  // 26.6 px
    horizontal_advance: f26_6,  // 26.6 px
    vertical_advance: f26_6,  // 26.6 px
}

impl AtlasGlyph {
    pub fn width(&self) -> i32{
        self.size.x
    }

    pub fn height(&self) -> i32 {
        self.size.y
    }
}

impl Atlas {
    pub fn new() -> Atlas {
        Default::default()
    }

    fn range_for_page(&self, page_num: u32) -> RangeInclusive<u8> {
        match page_num {
            0 => 0x20..=255,
            _ => 0..=255,
        }
    }

    pub fn load_page(&mut self, page_num: u32, face: &Face) {
        assert!(page_num < 256);
        let range = self.range_for_page(page_num);
        let page_num = page_num as usize;
        if self.pages[page_num].is_some() {
            return
        }
        let mut page: Box<AtlasPage> = Default::default();
        let mut width = page.width;
        let mut height = page.height;
        
        for i in range.clone() {
            if let Some(c) = std::char::from_u32(256*(page_num as u32) + i as u32) {
                let i = i as usize;
                if let Ok(ft_glyph) = face.load_char(c, freetype::face::LoadFlag::DEFAULT) {
                    let m = ft_glyph.metrics();
                    let size = Vector2I::new(m.width as f26_6, m.height as f26_6);
                    let w = fixed_26_6::ceil_to_i32(m.width as i32);
                    let h = fixed_26_6::ceil_to_i32(m.height as i32);
                    page.glyphs[i] = Some(Box::new(AtlasGlyph {
                        tex_coords: RectangleI::new(Vector2I::new(width, 0), w, h),
                        size,
                        horizontal_bearing: Vector2I::new(m.horiBearingX as i32, m.horiBearingY as i32),
                        vertical_bearing: Vector2I::new(m.vertBearingX as i32, m.vertBearingY as i32),
                        horizontal_advance: m.horiAdvance as i32,
                        vertical_advance: m.vertAdvance as i32,
                    }));
                    width += w + 1;
                    height = height.max(h + 1);
                } else {
                    eprintln!("Atlas::new: unable to load glyph for char {:?}", c)
                }
            }
        }

        let mut texture = gpu::Texture::new();
        texture.allocate(width, height, gpu::TextureFormat::Alpha);
        page.width = width;
        page.height = height;
        page.texture = texture;

        for i in range {
            if let Some(c) = std::char::from_u32(256*(page_num as u32) + i as u32) {
                let i = i as usize;
                if let Ok(ft_glyph) = face.load_char(c, freetype::face::LoadFlag::RENDER) {
                    let bitmap = ft_glyph.bitmap();
                    let glyph = page.glyphs[i].as_mut().unwrap();
                    let region = glyph.tex_coords;
                    assert!(bitmap.rows() >= region.height() && bitmap.width() >= region.width());
                    assert!(region.max.y <= height && region.max.x <= width);
                    texture.load_region_data(region, gpu::TextureFormat::Alpha, bitmap.buffer(), gpu::TextureUnit(0));
                }
            }
        }

        self.pages[page_num] = Some(page);
    }

    fn glyph_page(&self, c: char) -> Option<&AtlasPage> {
        let page_num = (c as u32 / 256) as usize;
        match self.pages[page_num] {
            Some(ref page) => Some(page.borrow()),
            None => None,
        }
    }

    pub fn glyph(&self, c: char) -> Option<&AtlasGlyph> {
        let page = self.glyph_page(c)?;
        let i = (c as u32 % 256) as usize;
        match page.glyphs[i] {
            Some(ref glyph) => Some(glyph.borrow()),
            None => None,
        }
    }

    pub fn texture(&self, page_num: u32) -> Option<gpu::Texture> {
        match self.pages[page_num as usize] {
            Some(ref page) => Some(page.texture),
            None => None,
        }
    }


    pub fn tex_coords(&self, c: char) -> Option<Rectangle> {
        let page = self.glyph_page(c)?;
        let width = page.width as f32;
        let height = page.height as f32;
        let glyph = self.glyph(c)?;
        let r = glyph.tex_coords;
        Some(Rectangle {
            min: Vector2::new(r.min.x as f32 / width, r.min.y as f32 / height),
            max: Vector2::new(r.max.x as f32 / width, r.max.y as f32 / height),
        })
    }
}


#[derive(Debug)]
pub struct Text {
    gpu_data: Vec<TextGpuData>,
}

impl Text {
    pub fn new(s: &str, position: Vector2, font: &mut Font) -> Text {
        let mut gpu_data = Vec::<TextGpuData>::new();
        let mut pen = position;
        let scale = 1.;  // 1. / (1 << UPSCALE_POWER) as f32;
        let mut i = 0;
        let mut last_char: Option<char> = None;

        for c in s.chars() {
            let unicode_page = c as u32 / 256;
            if gpu_data.iter().find(|d| d.unicode_page == unicode_page).is_none() {
                gpu_data.push(TextGpuData::new(unicode_page));
            }
        }

        for data in gpu_data.iter_mut() {
            font.load_page(data.unicode_page);
            data.texture = font.texture(data.unicode_page).unwrap();
        }

        for c in s.chars() {
            let unicode_page = c as u32 / 256;
            let data = gpu_data.iter_mut().find(|d| d.unicode_page == unicode_page).unwrap();
            if let Some(glyph) = font.glyph(c) {
                let uvs = font.tex_coords(c).unwrap();
                data.indices.extend_from_slice(&[i, i+1, i+3, i+3, i+1, i+2]);
                let w = scale * fixed_26_6::to_f32(glyph.width(), 0);
                let h = scale * fixed_26_6::to_f32(glyph.height(), 0);
                let top_left = pen + scale * Vector2::new(
                    fixed_26_6::to_f32(glyph.horizontal_bearing.x, 0),
                    -fixed_26_6::to_f32(glyph.horizontal_bearing.y, 0),
                );
                data.positions.push(top_left);
                data.positions.push(top_left + Vector2::new(w, 0.));
                data.positions.push(top_left + Vector2::new(w, h));
                data.positions.push(top_left + Vector2::new(0., h));
                data.tex_coords.push(uvs.min);
                data.tex_coords.push(Vector2::new(uvs.max.x, uvs.min.y));
                data.tex_coords.push(uvs.max);
                data.tex_coords.push(Vector2::new(uvs.min.x, uvs.max.y));
                pen.x += scale * fixed_26_6::to_f32(glyph.horizontal_advance, 0);
                if let Some(last_char) = last_char {
                    if let Ok(kerning) = font.kerning(last_char, c) {
                        pen.x += fixed_26_6::to_f32(kerning.x, 0)
                    }
                }
                i += 4;
                last_char = Some(c);
            } else {
                if c == ' ' {
                    pen.x += scale * font.size_px() as f32 / 3.0;  // TODO: Text layout.
                }
                last_char = None;
            }
        }

        for data in gpu_data.iter() {
            data.load_buffers();
        }
        
        Text { gpu_data }
    }

    pub fn draw(&self, shader: &mut TextShader) {
        for data in self.gpu_data.iter() {
            data.draw(shader);
        }
    }
}

#[derive(Debug)]
pub struct TextGpuData {
    unicode_page: u32,
    positions: Vec<Vector2>,
    tex_coords: Vec<Vector2>,
    indices: Vec<u32>,
    texture: gpu::Texture,
    vertex_array: gpu::VertexArray,
}

impl TextGpuData {
    pub fn new(unicode_page: u32) -> TextGpuData {
        TextGpuData {
            unicode_page,
            positions: Vec::new(),
            tex_coords: Vec::new(),
            indices: Vec::new(),
            texture: gpu::Texture::NONE,
            vertex_array: gpu::VertexArray::new(),
        }
    }

    pub fn load_buffers(&self) {
        let pos_buf_id = gpu::gen_buffer();
        let tex_buf_id = gpu::gen_buffer();
        self.vertex_array.setup_attribute(Attr::Position, pos_buf_id, PointerConfig::vector2());
        self.vertex_array.setup_attribute(Attr::TextureCoords, tex_buf_id, PointerConfig::vector2());

        gpu::load_index_buffer_data(self.vertex_array.index_buffer_id, &self.indices[..]);
        gpu::load_buffer_data(pos_buf_id, &self.positions[..]);
        gpu::load_buffer_data(tex_buf_id, &self.tex_coords[..]);
    }

    pub fn draw(&self, shader: &mut TextShader) {
        shader.set_texture(self.texture);
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
        self.program.activate();
        text.draw(self);
    }
}

#[allow(non_camel_case_types)]
type f26_6 = i32;

mod fixed_26_6 {
    use super::f26_6;

    pub fn ceil_to_i32(x: f26_6) -> i32 {
        let mut res = x as i32 >> 6;
        if (x & 63) != 0 {
            res += 1
        }
        res
    }

    pub fn to_f32(x: f26_6, precision: i32) -> f32 {
        (x >> (6 - precision)) as f32 / 2.0_f32.powf(precision as f32)    
    }
}
