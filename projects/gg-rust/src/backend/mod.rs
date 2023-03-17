use plotters::prelude::*;

use crate::GGPlot;

pub trait GGBackend {
    fn draw(&mut self);
}

impl GGPlot {
    #[cfg(feature = "svg")]
    pub fn draw() {
        let root_drawing_area = BitMapBackend::new("images/2.2.png", (600, 400))
            .into_drawing_area();

        root_drawing_area.fill(&WHITE).unwrap();


        let mut ctx = ChartBuilder::on(&root_drawing_area)
            .build_cartesian_2d(0.0..100.0, 0.0..100.0)?;


        ctx.configure_mesh().draw()?
    }
}

fn main() {}