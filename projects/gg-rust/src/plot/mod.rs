pub mod aesthetic;
pub mod geometric;

use crate::{GGAesthetic, GGGeometric, Result};
use csscolorparser::Color;
use std::ops::{Add, AddAssign};

#[macro_export]
macro_rules! add_impl {
    ($s:ty: ($key:ident: $t:ty)) => {
        impl Add<$t> for $s {
            type Output = Self;
            fn add(self, rhs: $t) -> Self::Output {
                Self { $key: rhs, ..self }
            }
        }
        impl AddAssign<$t> for $s {
            fn add_assign(&mut self, rhs: $t) {
                self.$key = rhs
            }
        }
    };
}

#[derive(Clone)]
pub struct GGPlot {
    aesthetic: GGAesthetic,
    geometric: GGGeometric,
    coordinate: GGCoordinate,
    scale: GGScale,
    view: GGScaleView,
    transform: GGTransform,
    transition: GGTransition,
    facet: GGFacet,
}


pub struct GGScaleView {}

pub struct GGTransform {}

pub struct GGTransition {}

pub struct GGCoordinate {}

pub struct GGScale {}

pub struct GGFacet {}

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
