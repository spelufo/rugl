extern crate freetype;


pub struct Font {

}



{
    let ft = freetype::Library::init()?;
    let face = ft.new_face(font_path, 0)?;
    face.set_char_size(40 * 64, 0, 50, 0)?;
    face.load_char('A' as usize, freetype::face::RENDER)?;
    let glyph = face.glyph();
    dbg!(glyph.bitmap());    
}


{
    let font = Font::open("/usr/share/fonts/TTF/DejaVuSerif.ttf");
    let atlas = FontAtlas::new(font, font_size);
    let texture = gpu::load_texture(&atlas.bitmap());
    let quads = ...
    for c in str {
        let uvs = atlas.uv_bounds_for_char('A')
        quads.append(Vertex { pos, uvs })
    }
    gpu::render()
}

