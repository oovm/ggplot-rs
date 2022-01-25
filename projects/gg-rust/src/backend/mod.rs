use plotters_svg::SVGBackend;
use plotters::prelude::*;

use crate::GGPlot;

impl GGPlot {
    #[cfg(feature = "svg")]
    pub fn draw_svg() {

    }
}

pub fn ggplot() -> GGPlot {
    GGPlot::default()
}


#[test]
fn main() {
    let drawing_area = SVGBackend::new("images/2.1.png", (600, 400)).into_drawing_area();

    drawing_area.fill(&WHITE).unwrap();

    let mut chart = ChartBuilder::on(&drawing_area).build_cartesian_2d(0..100, 0..100).unwrap();

    chart.draw_series(LineSeries::new((0..100).map(|x| (x, 100 - x)), &BLACK)).unwrap();
}
