
vector_space!(Color3 f32 {
    r: f32,
    g: f32,
    b: f32
});

impl Color3 {
    pub const K: Color3 = Color3 { r: 0., g: 0., b: 0. };
    pub const W: Color3 = Color3 { r: 1., g: 1., b: 1. };
    pub const R: Color3 = Color3 { r: 1., g: 0., b: 0. };
    pub const G: Color3 = Color3 { r: 0., g: 1., b: 0. };
    pub const B: Color3 = Color3 { r: 0., g: 0., b: 1. };
    pub const C: Color3 = Color3 { r: 0., g: 1., b: 1. };
    pub const M: Color3 = Color3 { r: 1., g: 0., b: 1. };
    pub const Y: Color3 = Color3 { r: 1., g: 1., b: 0. };
}

pub fn col3(r: f32, g: f32, b: f32) -> Color3 {
    Color3 { r, g, b }
}
