mod merge;

use std::ops::Range;
use super::*;

#[derive(Clone, Debug)]
pub enum GGCoordinate {
    Cartesian2(GGCartesian2D),
    Cartesian3(GGCartesian3D),
}

#[derive(Clone, Debug, Merge)]
pub struct GGCartesian2D {
    x: Option<GGLimited>,
    y: Option<GGLimited>,
}

#[derive(Clone, Debug, Merge)]
pub struct GGCartesian3D {
    x: Option<GGLimited>,
    y: Option<GGLimited>,
    z: Option<GGLimited>,
}

#[derive(Clone, Debug, Default)]
pub struct GGLimited {
    start: Option<f64>,
    end: Option<f64>,
}


