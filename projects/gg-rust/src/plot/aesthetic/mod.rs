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
pub struct GGLineStyle([u8; 8]);

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
        let u = Self::hex_to_u64(hex)?;
        let out = match hex.len() {
            0 => { Self::BLANK }
            1 => {
                let [_, _, _, _, _, _, _, w4] = u.to_be_bytes();
                Self([w4, w4, w4, w4, w4, w4, w4, w4])
            }
            2 => {
                let [_, _, _, _, _, _, _, w4] = u.to_be_bytes();
                Self([w4, w4, w4, w4, w4, w4, w4, w4])
            }
            4 => {
                let [_, _, _, _, _, _, b4, w4] = u.to_be_bytes();
                Self([b4, w4, b4, w4, b4, w4, b4, w4])
            }
            8 => {
                let [_, _, _, _, b3, w3, b4, w4] = u.to_be_bytes();
                Self([b3, w3, b4, w4, b3, w3, b4, w4])
            }
            16 => {
                let [b1, w1, b2, w2, b3, w3, b4, w4] = u.to_be_bytes();
                Self([b1, w1, b2, w2, b3, w3, b4, w4])
            }
            _ => return Err(GGError::SyntaxError("Unknown Hex Line Style".to_string()))
        };
        Ok(out)
    }
    fn hex_to_u64(hex: &str) -> Result<u64> {
        Ok(u64::from_str_radix(hex, 16)?)
    }
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
    println!("{:?}", GGLineStyle::parse_hex("FF"))
}