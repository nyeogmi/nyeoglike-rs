use std::cell::Cell;

use chiropterm::{Brush, Brushable, CellSize};

use crate::widgetry::ui::Selection;

use super::{WidgetDimensions, WidgetMenu, Widgetlike};

pub struct WidgetCommon<T: Widgetlike> {
    pub unique: T,
    pub(in crate::widgetry) selection: Selection,

    last_dimensions: Cell<(isize, WidgetDimensions)>,
}

impl<T: Widgetlike> WidgetCommon<T> {
    pub fn new(value: T) -> Self {
        WidgetCommon {
            unique: value,
            selection: Selection::not_selected(),
            last_dimensions: Cell::new((-1, WidgetDimensions { 
                min: CellSize::zero(), 
                preferred: CellSize::zero(), 
                max: CellSize::zero() 
            })),
        }
    }

    pub fn draw(&self, brush: Brush, menu: &WidgetMenu<T>) {
        self.unique.draw(menu.ui.is_selected(self.selection), brush, menu)
    }

    pub fn estimate_dimensions(&self, mut width: isize) -> WidgetDimensions {
        if width < 0 { width = 0; }
        // TODO: If it's 0, provide a stock answer

        let (last_width, last_dims) = self.last_dimensions.get();
        if last_width == width {
            return last_dims;
        }

        let new_dims = self.unique.estimate_dimensions(width).fixup();
        self.last_dimensions.replace((last_width, new_dims));
        new_dims
    }
}