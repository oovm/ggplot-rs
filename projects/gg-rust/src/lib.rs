mod backend;
mod plot;
mod color;

pub use self::plot::{aesthetic::*, geometric::*, GGPlot};
use ggplot_error::{GGError, Result};
