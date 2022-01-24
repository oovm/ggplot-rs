use crate::add_impl;
use super::*;


#[derive(Clone, Debug)]
pub struct GGAesthetic {
    color: Color,
    line: GGLine,
    polygon: GGPolygon,
    point: GGPoint,
    text: GGText,
}

#[derive(Clone, Debug)]
pub struct GGLine {
    size: GGSize,
}

#[derive(Clone, Debug)]
pub struct GGSize {}

#[derive(Clone, Debug)]
pub struct GGPolygon {}

#[derive(Clone, Debug)]
pub struct GGPoint {}

#[derive(Clone, Debug)]
pub struct GGText {}

/// Builder pattern of Aesthetic
impl GGAesthetic {
    #[inline]
    pub fn with_color(mut self, color: impl Into<Color>) -> Self {
        self.color = color.into();
        self
    }    #[inline]
    pub fn with_color_name(mut self, name: &str) -> Result<Self> {
        self.color = name.parse()?;
        Ok(self)
    }    #[inline]
    pub fn with_color_rgba(mut self, r: u8, g: u8, b: u8, a: u8) -> Self {
        self.color = Color::from_rgba_u8(r, g, b, a);
        self
    }    #[inline]
    pub fn with_color_hex(mut self, hex: &str) -> Result<Self> {
        self.color = hex.parse()?;
        Ok(self)
    }
}



add_impl!(GGPlot:(aesthetic:GGAesthetic));

impl Add<Color> for GGAesthetic {
    type Output = Self;

    fn add(self, rhs: Color) -> Self::Output {
        Self {
            color: rhs,
            ..self
        }
    }
}

#[test]
fn test() {}