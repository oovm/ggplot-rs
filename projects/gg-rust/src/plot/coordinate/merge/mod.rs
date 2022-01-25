use super::*;

impl Default for GGCoordinate {
    fn default() -> Self {
        Self::Cartesian2(GGCartesian2D::default())
    }
}


impl Default for GGCartesian2D {
    fn default() -> Self {
        Self {
            x: None,
            y: None,
        }
    }
}

impl Add<GGCartesian2D> for GGPlot {
    type Output = Self;

    fn add(self, rhs: GGCartesian2D) -> Self::Output {
        Self { coordinate: Some(GGCoordinate::Cartesian2(rhs)), ..self }
    }
}