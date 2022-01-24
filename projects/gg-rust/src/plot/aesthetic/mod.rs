use super::*;



#[derive(Clone,Debug)]
pub struct GGAesthetic {
    color: Color,
    line: GGLine,
    polygon: GGPolygon,
    point: GGPoint,
    text: GGText,
}
#[derive(Clone,Debug)]
pub struct GGLine {
    size: GGSize,
}
#[derive(Clone,Debug)]
pub struct GGSize {}
#[derive(Clone,Debug)]
pub struct GGPolygon {}
#[derive(Clone,Debug)]
pub struct GGPoint {}
#[derive(Clone,Debug)]
pub struct GGText {}

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

macro_rules! add_impl {
    ($tar:ty: ($key:ident: $ty:ty)) => {
impl Add<GGAesthetic> for GGPlot {
    type Output = Self;

    fn add(self, rhs: GGAesthetic) -> Self::Output {
        Self {
            aesthetic: rhs,
            ..self
        }
    }
}

impl AddAssign<GGAesthetic> for GGPlot {
    fn add_assign(&mut self, rhs: GGAesthetic) {
        self.aesthetic = rhs
    }
}
    };
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

}