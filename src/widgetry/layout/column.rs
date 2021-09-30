use std::cell::RefCell;

use chiropterm::{Brush, CellSize};
use euclid::Size2D;

use crate::widgetry::{WidgetDimensions, WidgetMenu, Widgetlike, widget::AnyWidget};


pub struct ColumnState<'draw> {
    widgets: Vec<AnyWidget<'draw>>,
    plots: RefCell<(isize, Plots)>,
}

impl<'draw> Default for ColumnState<'draw> {
    fn default() -> Self {
        ColumnState { 
            widgets: vec![],
            plots: RefCell::new((-1, Plots::new())),
        }
    }
}

impl<'draw> Widgetlike for ColumnState<'draw> {
    fn draw(&self, selected: bool, brush: Brush, menu: &WidgetMenu<Self>) {
        todo!()
    }

    fn estimate_dimensions(&self, width: isize) -> WidgetDimensions {
        todo!()
    }
}

impl<'draw> ColumnState<'draw> {
    fn underlying_compute_plots(&self, width: isize) -> Plots {
        assert!(width >= 0);
        let mut total_height_pref = 0;
        for w in self.widgets.iter() {
            let dim = w.estimate_dimensions(width);
            total_height_pref += dim.preferred.height;
        }
        todo!();
    }
}

struct Plots {
    plot_size: Vec<isize>,
}

impl Plots {
    fn new() -> Plots {
        Plots { 
            plot_size: Vec::new(),
        }
    }
}