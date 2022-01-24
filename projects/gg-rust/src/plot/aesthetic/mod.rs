use std::iter::Cycle;
use ggplot_error::GGError;
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
    kind: GGLineStyle,
    size: GGSize,
}

/// (Black, White) × 4
#[derive(Clone, Debug, Ord, PartialOrd, Eq, PartialEq)]
pub struct GGLineStyle(pub [u8; 8]);

impl GGLineStyle {
    pub const BLANK: Self = Self([0, 0, 0, 0, 0, 0, 0, 0]);
    pub const SOLID: Self = Self([16, 16, 16, 16, 16, 16, 16, 16]);
    pub const DASHED: Self = Self([4, 4, 4, 4, 4, 4, 4, 4]);
    pub const DOTTED: Self = Self([1, 3, 1, 3, 1, 3, 1, 3]);
    pub const DOT_DASH: Self = Self([1, 3, 4, 3, 1, 3, 4, 3]);
    pub const LONG_DASH: Self = Self([7, 3, 7, 3, 7, 3, 7, 3]);
    pub const TWO_DASH: Self = Self([2, 2, 6, 2, 2, 2, 6, 2]);
    pub fn is_blank(&self) -> bool {
        self.eq(&Self::BLANK)
    }
    pub fn is_solid(&self) -> bool {
        self.eq(&Self::SOLID)
    }
    /// F
    pub fn parse_hex(hex: &str) -> Result<Self> {
        let style: Result<Vec<_>> = hex.chars().map(|c| Self::hex_to_u8(hex)).collect();
        let out = match style?[..] {
            [] => { Self::BLANK }
            [w4] => {
                Self([w4, w4, w4, w4, w4, w4, w4, w4])
            }
            [b4, w4] => {
                Self([b4, w4, b4, w4, b4, w4, b4, w4])
            }
            [b3, w3, b4, w4] => {
                Self([b3, w3, b4, w4, b3, w3, b4, w4])
            }
            [b1, w1, b2, w2, b3, w3, b4, w4] => {
                Self([b1, w1, b2, w2, b3, w3, b4, w4])
            }
            _ => return Err(GGError::SyntaxError("Unknown Hex Line Style".to_string()))
        };
        Ok(out)
    }
    fn hex_to_u8(hex: &str) -> Result<u8> {
        Ok(u8::from_str_radix(hex, 16)?)
    }
}

impl<'a> IntoIterator for &'a GGLineStyle {
    type Item = bool;
    type IntoIter = GGLineDashOrBlank<'a>;

    fn into_iter(self) -> Self::IntoIter {
        if self.is_blank() {
            GGLineDashOrBlank::Blank
        } else if self.is_solid() {
            GGLineDashOrBlank::Solid
        } else {
            GGLineDashOrBlank::Cycle {
                state: self.0.iter().cycle(),
                should_draw: true,
                rest_count: self.0[0],
            }
        }
    }
}

impl<'a> Iterator for GGLineDashOrBlank<'a> {
    type Item = bool;

    fn next(&mut self) -> Option<Self::Item> {
        match self {
            Self::Blank => { Some(false) }
            Self::Solid => { Some(true) }
            Self::Cycle { state, should_draw, rest_count } => unsafe {
                if *rest_count == 0 {
                    *rest_count = *state.next().unwrap_unchecked();
                    *should_draw = !*should_draw;
                    Some(*should_draw)
                } else {
                    *rest_count -= 1;
                    Some(*should_draw)
                }
            }
        }
    }
}

pub enum GGLineDashOrBlank<'a> {
    Blank,
    Solid,
    Cycle {
        state: Cycle<std::slice::Iter<'a, u8>>,
        should_draw: bool,
        rest_count: u8,
    },
}

impl GGLineStyle {}


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
fn test() {
    let s = GGLineStyle::parse_hex("5").unwrap();

    for (i, j) in s.into_iter().enumerate() {
        if i < 10 {
            println!("{}", j)
        }
        else { break; }
    }
}