use chiropterm::{Brush, Menu};

use crate::widgetry::UI;

use super::{WidgetDimensions, Widget, WidgetMenu, Widgetlike};

pub struct AnyWidget {
    implementation: Box<dyn AWidget>,
}

impl AnyWidget {
    pub fn wrap<X: Widgetlike>(widget: Widget<X>) -> AnyWidget {
        AnyWidget { 
            implementation: Box::new(widget)
        }
    }

    pub fn estimate_dimensions(&self, ui: &UI, width: isize) -> WidgetDimensions {
        self.implementation.poly_estimate_dimensions(ui, width)
    }

    pub fn draw<'frame, X: Widgetlike>(&self, brush: Brush, menu: WidgetMenu<'frame, X>) {
        let ui = menu.ui;
        let menu = menu.menu;

        self.implementation.poly_draw(ui, brush, menu);
    }

    pub(crate) fn clear_layout_cache_if_needed(&self, ui: &UI) {
        self.implementation.poly_clear_layout_cache_if_needed(ui)
    }
}

trait AWidget: 'static {
    fn poly_estimate_dimensions(&self, ui: &UI, width: isize) -> WidgetDimensions;
    fn poly_draw<'frame>(&self, ui: UI, brush: Brush, menu: Menu<'frame>);
    fn poly_clear_layout_cache_if_needed(&self, ui: &UI);
}

impl<T: Widgetlike> AWidget for Widget<T> {
    fn poly_estimate_dimensions(&self, ui: &UI, width: isize) -> WidgetDimensions {
        self.estimate_dimensions(ui, width)
    }

    fn poly_draw<'frame>(&self, ui: UI, brush: Brush, menu: Menu<'frame>) {
        self.draw(ui, brush, menu)
    }

    fn poly_clear_layout_cache_if_needed(&self, ui: &UI) {
        self.clear_layout_cache_if_needed(ui)
    }
}