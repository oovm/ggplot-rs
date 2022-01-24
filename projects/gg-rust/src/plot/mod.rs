pub mod aesthetic;

use std::ops::Add;
use crate::GGAesthetic;
use std::ops::{ AddAssign};
pub use csscolorparser::Color;


#[derive(Clone)]
pub struct GGPlot {
    aesthetic: GGAesthetic,
    geometric: GGGeometric,
}

/// Builder pattern for GGPlot
impl GGPlot {
    /// Build with custom aesthetic
    pub fn with_aesthetic(mut self, aesthetic: impl Into<GGAesthetic>) -> Self {
        self.aesthetic = aesthetic.into();
        self
    }
    /// Build with custom geometric
    pub fn with_geometric(mut self, geometric: impl Into<GGGeometric>) -> Self {
        self.geometric = geometric.into();
        self
    }
}




#[derive(Clone,Debug)]
pub enum GGGeometric {}

#[test]
fn test() {}
