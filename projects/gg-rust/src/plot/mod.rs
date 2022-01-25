pub mod aesthetic;
pub mod geometric;
pub mod mergeable;

use crate::{GGAesthetic, GGGeometric, Result};
use csscolorparser::Color;
use std::ops::{Add, AddAssign};

#[macro_export]
macro_rules! add_impl {
    ($s:ty: ($key:ident: $t:ty)) => {
        impl Add<$t> for $s {
            type Output = Self;
            fn add(self, rhs: $t) -> Self::Output {
                Self { $key: Some(rhs), ..self }
            }
        }
        impl AddAssign<$t> for $s {
            fn add_assign(&mut self, rhs: $t) {
                self.$key = Some(rhs)
            }
        }
    };
}

#[derive(Clone)]
pub struct GGPlot {
    aesthetic: Option<GGAesthetic>,
    geometric: Option<GGGeometric>,
    coordinate: Option<GGCoordinate>,
    scale: Option<GGScale>,
    view: Option<GGScaleView>,
    transform: Option<GGTransform>,
    transition: Option<GGTransition>,
    facet: Option<GGFacet>,
}

#[derive(Clone)]
pub struct GGScaleView {}
#[derive(Clone)]
pub struct GGTransform {}
#[derive(Clone)]
pub struct GGTransition {}
#[derive(Clone)]
pub struct GGCoordinate {}
#[derive(Clone)]
pub struct GGScale {}
#[derive(Clone)]
pub struct GGFacet {}

/// Builder pattern for GGPlot
impl GGPlot {
    /// Build with custom aesthetic
    pub fn with_aesthetic(mut self, aesthetic: impl Into<GGAesthetic>) -> Self {
        self.aesthetic = Some(aesthetic.into());
        self
    }
    /// Build with custom geometric
    pub fn with_geometric(mut self, geometric: impl Into<GGGeometric>) -> Self {
        self.geometric = Some(geometric.into());
        self
    }
}


