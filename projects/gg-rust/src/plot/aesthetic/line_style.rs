use super::*;

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
    pub fn new(raw: [u8; 8]) -> Self {
        Self(raw)
    }
    pub fn iter(&self) -> GGLineDrawOrBlank {
        if self.is_blank() {
            GGLineDrawOrBlank::Blank
        } else if self.is_solid() {
            GGLineDrawOrBlank::Solid
        } else {
            GGLineDrawOrBlank::Cycle { state: self.0.iter().cycle(), should_draw: true, rest_count: self.0[0] }
        }
    }
}



impl FromStr for GGLineStyle {
    type Err = GGError;

    fn from_str(hex: &str) -> Result<Self> {
        let style: Result<Vec<_>> = hex.chars().map(|c| hex_to_u8(hex)).collect();
        let out = match style?[..] {
            [] => Self::BLANK,
            [w4] => Self([w4, w4, w4, w4, w4, w4, w4, w4]),
            [b4, w4] => Self([b4, w4, b4, w4, b4, w4, b4, w4]),
            [b3, w3, b4, w4] => Self([b3, w3, b4, w4, b3, w3, b4, w4]),
            [b1, w1, b2, w2, b3, w3, b4, w4] => Self([b1, w1, b2, w2, b3, w3, b4, w4]),
            _ => return Err(GGError::SyntaxError("Unknown Hex Line Style".to_string())),
        };
        Ok(out)
    }
}

fn hex_to_u8(hex: &str) -> Result<u8> {
    Ok(u8::from_str_radix(hex, 16)?)
}

impl<'a> IntoIterator for &'a GGLineStyle {
    type Item = bool;
    type IntoIter = GGLineDrawOrBlank<'a>;

    fn into_iter(self) -> Self::IntoIter {
        self.iter()
    }
}

impl<'a> Iterator for GGLineDrawOrBlank<'a> {
    type Item = bool;

    fn next(&mut self) -> Option<Self::Item> {
        match self {
            Self::Blank => Some(false),
            Self::Solid => Some(true),
            Self::Cycle { state, should_draw, rest_count } => unsafe {
                if *rest_count == 0 {
                    *rest_count = *state.next().unwrap_unchecked();
                    *should_draw = !*should_draw;
                    Some(*should_draw)
                } else {
                    *rest_count -= 1;
                    Some(*should_draw)
                }
            },
        }
    }
}

pub enum GGLineDrawOrBlank<'a> {
    Blank,
    Solid,
    Cycle { state: Cycle<std::slice::Iter<'a, u8>>, should_draw: bool, rest_count: u8 },
}
