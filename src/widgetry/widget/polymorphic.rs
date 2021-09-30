use std::any::Any;

use chiropterm::{Brush, Brushable, CellPoint, FSem, Menu};

use crate::widgetry::UI;

use super::{Widget, WidgetDimensions, WidgetMenu, Widgetlike};

pub struct AnyWidget<'draw> {
    implementation: Box<dyn AWidget<'draw>>,
}

impl<'draw> AnyWidget<'draw> {
    pub fn estimate_dimensions(&self, width: isize) -> WidgetDimensions {
        self.implementation.poly_estimate_dimensions(width)
    }

    pub fn draw<X: Widgetlike>(&self, brush: Brush, menu: WidgetMenu<'draw, X>) {
        let ui = menu.ui;
        let menu = menu.menu;

        self.implementation.poly_draw(ui, brush, menu);
    }
}

trait AWidget<'draw> {
    fn poly_estimate_dimensions(&self, width: isize) -> WidgetDimensions;
    fn poly_draw(&self, ui: UI<'draw>, brush: Brush, menu: Menu<'draw, ()>);
}

impl<'draw, T: 'draw+Widgetlike> AWidget<'draw> for Widget<'draw, T> {
    fn poly_estimate_dimensions(&self, width: isize) -> WidgetDimensions {
        self.estimate_dimensions(width)
    }

    fn poly_draw(&self, ui: UI<'draw>, brush: Brush, menu: Menu<'draw, ()>) {
        self.draw(ui, brush, menu)
    }
}