mod plot;
mod backend;

pub use self::plot::{aesthetic::*, geometric::*, GGPlot};
use ggplot_error::{GGError, Result};
