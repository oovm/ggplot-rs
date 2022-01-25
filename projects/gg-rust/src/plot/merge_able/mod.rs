use super::*;


impl Add<GGPlot> for GGPlot {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Self {
            aesthetic: rhs.aesthetic.and(self.aesthetic),
            geometric: rhs.geometric.and(self.geometric),
            coordinate: rhs.coordinate.and(self.coordinate),
            scale: rhs.scale.and(self.scale),
            view: rhs.view.and(self.view),
            transform: rhs.transform.and(self.transform),
            transition: rhs.transition.and(self.transition),
            facet: rhs.facet.and(self.facet),
        }
    }
}

impl AddAssign<GGPlot> for GGPlot {
    fn add_assign(&mut self, rhs: GGPlot) {
        if let Some(new) = rhs.aesthetic {
            self.aesthetic = Some(new)
        }
        if let Some(new) = rhs.geometric {
            self.geometric = Some(new)
        }
        if let Some(new) = rhs.coordinate {
            self.coordinate = Some(new)
        }
    }
}