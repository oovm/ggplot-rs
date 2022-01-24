use csscolorparser::Color;
use std::ops::Add;

#[derive(Clone)]
pub struct GGPlot {
    aesthetic: Option<GGAesthetic>,
    geometric: Option<GGGeometric>,
}

/// Builder pattern for GGPlot
impl GGPlot {
    /// Build with custom aesthetic
    pub fn with_aesthetic(mut self, aesthetic: impl Into<GGAesthetic>) -> Self {
        self.aesthetic = Some(aesthetic.into());
        self
    }
    /// Build with custom geometric
    pub fn with_geometric(mut self, geometric: impl Into<GGGeometric>) -> Self {
        self.geometric = Some(geometric.into());
        self
    }
}

pub struct GGAesthetic {
    color: Color,
}

impl GGAesthetic {
    pub fn with_color(mut self, color: impl Into<Color>) -> Self {
        self.color = color.into();
        self
    }
    pub fn with_color_name(mut self, name: &str) -> Self {
        todo!()
    }
    pub fn with_color_rgba(mut self, r: u8, g: u8, b: u8, a: u8) -> Self {
        todo!()
    }
    pub fn with_color_hex(mut self, hex: &str) -> Self {
        todo!()
    }
}

pub enum GGGeometric {}

#[test]
fn test() {}
