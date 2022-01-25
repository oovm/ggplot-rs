pub mod aesthetic;
pub mod geometric;
pub mod merge_able;
pub mod coordinate;

use crate::{GGAesthetic, GGGeometric, Result};
use csscolorparser::Color;
use std::ops::{Add, AddAssign};
use ggplot_derive::Merge;

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

#[derive(Clone, Debug, Default, Merge)]
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

#[derive(Clone, Debug, Default, Merge)]
pub struct GGScaleView {}

#[derive(Clone, Debug, Default, Merge)]
pub struct GGTransform {}

#[derive(Clone, Debug, Default, Merge)]
pub struct GGTransition {}



#[derive(Clone, Debug, Default, Merge)]
pub struct GGScale {}

#[derive(Clone, Debug, Default, Merge)]
pub struct GGFacet {}
