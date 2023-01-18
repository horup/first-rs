#[derive(Debug, Clone, Copy)]
pub struct Color {
    pub r:f32,
    pub g:f32,
    pub b:f32,
    pub a:f32
}

impl Default for Color {
    fn default() -> Self {
        Self::WHITE
    }
}

impl Color {
    pub const WHITE:Self = Self { r: 1.0, g: 1.0, b: 1.0, a: 1.0 };
    pub const BLACK:Self = Self { r: 0., g: 0., b: 0., a: 1.0 };
    pub const RED:Self = Self { r: 1.0, g: 0., b: 0., a: 1.0 };
    pub const GREEN:Self = Self { r: 0., g: 1.0, b: 0., a: 1.0 };
    pub const BLUE:Self = Self { r: 0., g: 0., b: 1.0, a: 1.0 };
}

impl From<Color> for [f32;4] {
    fn from(color: Color) -> Self {
        [color.r, color.g, color.b, color.a]
    }
}

impl From<[f32;4]> for Color {
    fn from(c: [f32;4]) -> Self {
        Color {
            r: c[0],
            g: c[1],
            b: c[2],
            a: c[3],
        }
    }
} 