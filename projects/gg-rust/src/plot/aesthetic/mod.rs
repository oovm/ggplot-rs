mod line;
mod line_style;

use super::*;
use crate::add_impl;
use ggplot_error::GGError;
use std::{iter::Cycle, str::FromStr};

#[derive(Clone, Debug, Default, Merge)]
pub struct GGAesthetic {
    color: Option<Color>,
    line: Option<GGLine>,
    polygon: Option<GGPolygon>,
    point: Option<GGPoint>,
    text: Option<GGText>,
}

// https://ggplot2.tidyverse.org/articles/ggplot2-specs.html#lines-1
#[derive(Clone, Debug, Default)]
pub struct GGLine {
    /// The size of a line, must positive
    size: f32,
    /// Define how to dash the line
    style: GGLineStyle,
    /// Define the joint between line
    joint_between: GGLineJoint,
    /// Define the joint at start of line
    joint_end: GGLineJoint,
    /// Define the joint at end of line
    joint_start: GGLineJoint,
}

/// (Black, White) × 4
#[derive(Clone, Debug, Ord, PartialOrd, Eq, PartialEq)]
pub struct GGLineStyle([u8; 8]);

#[derive(Clone, Debug, Ord, PartialOrd, Eq, PartialEq)]
pub enum GGLineJoint {
    Round,
    Circle,
    Square,
}


#[derive(Clone, Debug)]
pub struct GGPolygon {
    fill: Color,
    lines: Vec<GGLine>,
}

impl Default for GGPolygon {
    fn default() -> Self {
        Self{
            fill: Default::default(),
            lines: vec![]
        }
    }
}

impl Default for GGPoint {
    fn default() -> Self {
        Self{
            size: 0.0,
            edge: Default::default(),
            fill: Default::default()
        }
    }
}

#[derive(Clone, Debug)]
pub struct GGPoint {
    size: f32,
    edge: Color,
    fill: Color,
}

impl Default for GGText {
    fn default() -> Self {
        Self {
            font_family: vec![]
        }
    }
}

#[derive(Clone, Debug)]
pub struct GGText {
    font_family: Vec<String>,
}

/// Builder pattern of Aesthetic
impl GGAesthetic {
    #[inline]
    pub fn with_color_name(mut self, name: &str) -> Result<Self> {
        self.color = Some(name.parse()?);
        Ok(self)
    }
    #[inline]
    pub fn with_color_rgba(mut self, r: u8, g: u8, b: u8, a: u8) -> Self {
        self.color = Some(Color::from_rgba_u8(r, g, b, a));
        self
    }
    #[inline]
    pub fn with_color_hex(mut self, hex: &str) -> Result<Self> {
        self.color = Some(hex.parse()?);
        Ok(self)
    }
    #[inline]
    pub fn with_line_style(mut self, line: GGLine) -> Result<Self> {
        self.line = Some(line);
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
