pub trait PlotColor {}


pub struct Color {
    pub r: u8,
    pub g: u8,
    pub b: u8,
    pub a: u8,
}

impl Color {
    pub fn alpha_blend(&self) -> Color {
        let alpha = self.a as f32 / 255.0;
        let r = (self.r as f32 * alpha) as u8;
        let g = (self.g as f32 * alpha) as u8;
        let b = (self.b as f32 * alpha) as u8;
        Color { r, g, b, a: 255 }
    }
}