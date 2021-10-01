use std::cell::Cell;

use chiropterm::{Brush, CellSize};

use crate::widgetry::ui::Selection;

use super::{WidgetDimensions, WidgetMenu, Widgetlike};

pub struct WidgetCommon<T> {
    pub unique: T,
    pub(in crate::widgetry) selection: Selection,

    last_dimensions: Cell<(isize, WidgetDimensions)>,
}

impl<'gamestate, T: Widgetlike<'gamestate>> WidgetCommon<T> {
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

    pub fn draw<'frame>(&self, brush: Brush, menu: WidgetMenu<'gamestate, 'frame, T, T::Out>) {
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