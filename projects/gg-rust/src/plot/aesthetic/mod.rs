mod line_style;

use super::*;
use crate::add_impl;
use ggplot_error::GGError;
use std::iter::Cycle;
use std::str::FromStr;

#[derive(Clone, Debug)]
pub struct GGAesthetic {
    color: Color,
    line: GGLine,
    polygon: GGPolygon,
    point: GGPoint,
    text: GGText,
}
// https://ggplot2.tidyverse.org/articles/ggplot2-specs.html#lines-1
#[derive(Clone, Debug)]
pub struct GGLine {
    style: GGLineStyle,
    /// The size of a line is its width in mm.
    size: usize,
    joint: GGLineJoint
}

/// (Black, White) × 4
#[derive(Clone, Debug, Ord, PartialOrd, Eq, PartialEq)]
pub struct GGLineStyle([u8; 8]);

pub enum GGLineJoint {

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
    }
    #[inline]
    pub fn with_color_name(mut self, name: &str) -> Result<Self> {
        self.color = name.parse()?;
        Ok(self)
    }
    #[inline]
    pub fn with_color_rgba(mut self, r: u8, g: u8, b: u8, a: u8) -> Self {
        self.color = Color::from_rgba_u8(r, g, b, a);
        self
    }
    #[inline]
    pub fn with_color_hex(mut self, hex: &str) -> Result<Self> {
        self.color = hex.parse()?;
        Ok(self)
    }
    #[inline]
    pub fn with_line_style(mut self, line: GGLine) -> Result<Self> {
        self.line = line;
        Ok(self)
    }

}

impl GGLine {
    #[inline]
    pub fn with_line_style_hex(mut self, hex: &str) -> Result<Self> {
        self.style = hex.parse()?;
        Ok(self)
    }
}

add_impl!(GGPlot: (aesthetic: GGAesthetic));
add_impl!(GGAesthetic: (color: Color));

