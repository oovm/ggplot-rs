use ggrust::{GGAesthetic, GGPlot, GGPoint};


/// ggplot(data)
pub fn ggplot() -> GGPlot {
    GGPlot::default()
}

/// ggplot(data) + geom_point()
pub fn geom_point() -> GGPoint {
    GGPoint::default()
}

pub fn aes() -> GGAesthetic {
    GGAesthetic::default()
}

