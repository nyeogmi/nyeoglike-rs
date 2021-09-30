use std::cell::{Ref, RefCell};

use chiropterm::{Brush, CellSize};
use euclid::{size2};
use smallvec::SmallVec;

use crate::widgetry::{UI, WidgetDimensions, WidgetMenu, Widgetlike, widget::AnyWidget};

// Smallvec size -- set this to "higher than most users will ever put in one column/row"
const SM: usize = 32;

pub struct ColumnState<'draw> {
    widgets: SmallVec<[AnyWidget<'draw>; SM]>,
    plots_desired: RefCell<(isize, Plots)>,
    plots_practical: RefCell<(CellSize, Plots)>,
}

impl<'draw> Default for ColumnState<'draw> {
    fn default() -> Self {
        ColumnState { 
            widgets: SmallVec::new(),
            plots_desired: RefCell::new((-1, Plots::new())),
            plots_practical: RefCell::new((size2(-1, -1), Plots::new())),
        }
    }
}

impl<'draw> Widgetlike<'draw> for ColumnState<'draw> {
    fn draw(&self, _: bool, brush: Brush, menu: WidgetMenu<'draw, ColumnState<'draw>>) {
        let plots = self.get_plots_practical(brush.rect().size);
        for (w, p) in self.widgets.iter().zip(plots.1.plot_size.iter()) {
            w.draw(brush.clone(), menu.share())
        }
    }

    fn estimate_dimensions(&self, width: isize) -> WidgetDimensions {
        todo!()
    }
}

impl<'draw> ColumnState<'draw> {
    fn get_plots_desired(&self, width: isize) -> Ref<'_, (isize, Plots)> {
        {
            let b = self.plots_desired.borrow();
            let (sz, pl) = &*b;
            if sz == &width && pl.plot_size.len() == self.widgets.len() {
                return b
            }
        }
        self.plots_desired.replace((width, self.internal_compute_plots_desired(width)));
        return self.plots_desired.borrow()
    }

    fn get_plots_practical(&self, size: CellSize) -> Ref<'_, (CellSize, Plots)> {
        {
            let b = self.plots_practical.borrow();
            let (sz, pl) = &*b;
            if sz == &size && pl.plot_size.len() == self.widgets.len() {
                return b
            }
        }
        self.plots_practical.replace((size, self.internal_compute_plots_practical(size)));
        return self.plots_practical.borrow()
    }

    fn internal_compute_plots_desired(&self, width: isize) -> Plots {
        // TODO: Use the cache
        let mut preferred: SmallVec<[isize; SM]> = SmallVec::new();

        for w in self.widgets.iter() {
            let dim = w.estimate_dimensions(width);
            preferred.push(dim.preferred.height);
        }

        Plots { plot_size: preferred }
    }

    fn internal_compute_plots_practical(&self, size: CellSize) -> Plots {
        let mut minimum: SmallVec<[isize; SM]> = SmallVec::new();
        let mut practical: SmallVec<[isize; SM]> = SmallVec::new();
        let mut maximum: SmallVec<[isize; SM]> = SmallVec::new();

        for w in self.widgets.iter() {
            let dim = w.estimate_dimensions(size.width);
            minimum.push(dim.min.height);
            practical.push(dim.preferred.height);
            maximum.push(dim.max.height);
        }

        if practical.len() == 0 { return Plots { plot_size: practical }; }

        let mut practical_sum: isize = practical.iter().sum();
        if practical_sum < size.height {
            // Just align to the top-left by expanding the bottom cell
            // TODO: Pick the index based on which widget loves to be expanded
            let ix = practical.len() - 1;
            practical[ix] += size.height - practical_sum;
        }
        else {
            let mut desperate = false;
            while practical_sum > size.height {
                // Steal from everyone equally, starting at bottom
                let prev_sum = practical_sum;
                for i in (0..practical.len()).rev() {
                    if !desperate && practical[i] <= minimum[i] { continue }
                    if practical[i] <= 0 { continue }

                    practical[i] -= 1;
                }
                if prev_sum == practical_sum { 
                    desperate = true; 
                }
            }
        }

        return Plots { plot_size: practical }
    }
}

struct Plots {
    plot_size: SmallVec<[isize; SM]>,
}

impl Plots {
    fn new() -> Plots {
        Plots { plot_size: SmallVec::new(), }
    }
}