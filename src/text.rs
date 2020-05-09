use freetype::error::Error as FtError;


pub fn init_library() -> Result<freetype::Library, FtError> {
    freetype::Library::init()
}

pub struct Font<'a> {
    face: freetype::Face,
    // ft: &'a freetype::Library,
    _bounded_by_ft_library: std::marker::PhantomData<&'a freetype::Library>,
}

impl<'a> Font<'a> {
    pub fn open(ft: &'a freetype::Library, path: &str) -> Result<Font<'a>, FtError> {
        let face = ft.new_face(path, 0)?;
        face.set_char_size(40 * 60, 0, 50, 0)?;
        dbg!(&face);
        Ok(Font {
            face,
            _bounded_by_ft_library: std::marker::PhantomData,
        })
    }

    pub fn char_bitmap(&mut self, char: char) -> Result<freetype::Bitmap, FtError> {
        self.face.load_char(char as usize, freetype::face::LoadFlag::RENDER)?;
        let glyph = self.face.glyph();
        Ok(glyph.bitmap())
    }
}

// {
//     let ft = freetype::Library::init()?;
//     let face = ft.new_face(font_path, 0)?;
//     face.set_char_size(40 * 64, 0, 50, 0)?;
//     face.load_char('A' as usize, freetype::face::RENDER)?;
//     let glyph = face.glyph();
//     dbg!(glyph.bitmap());    
// }


// {
//     let font = Font::open("/usr/share/fonts/TTF/DejaVuSerif.ttf");
//     let atlas = FontAtlas::new(font, font_size);
//     let texture = gpu::load_texture(&atlas.bitmap());
//     let quads = ...
//     for c in str {
//         let uvs = atlas.uv_bounds_for_char('A')
//         quads.append(Vertex { pos, uvs })
//     }
//     gpu::render()
// }

