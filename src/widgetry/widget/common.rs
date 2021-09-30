use chiropterm::{Brush, Brushable};

use crate::widgetry::ui::Selection;

use super::{WidgetDimensions, WidgetMenu, Widgetlike};

pub struct WidgetCommon<T: Widgetlike> {
    pub unique: T,
    pub(in crate::widgetry) selection: Selection,
}

impl<T: Widgetlike> WidgetCommon<T> {
    pub fn new(value: T) -> Self {
        WidgetCommon {
            unique: value,
            selection: Selection::not_selected(),
        }
    }

    pub fn draw<X: Brushable>(&self, brush: Brush<X>, menu: &WidgetMenu<T>) {
        self.unique.draw(menu.ui.is_selected(self.selection), brush, menu)
    }

    pub fn estimate_dimensions(&self, width: isize) -> WidgetDimensions {
        self.unique.estimate_dimensions(width)
    }
}