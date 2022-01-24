mod errors;
mod plot;

pub use errors::{Error, Result};

pub use self::plot::aesthetic::*;
pub use self::plot::GGPlot;