use chiropterm::{Brush, Menu};

use crate::widgetry::UI;

use super::{WidgetDimensions, Widget, WidgetMenu, Widgetlike};

pub struct AnyWidget<'gamestate> {
    implementation: Box<dyn AWidget<'gamestate>>,
}

impl<'gamestate> AnyWidget<'gamestate> {
    pub fn wrap<X: Widgetlike<'gamestate>>(widget: Widget<'gamestate, X>) -> AnyWidget<'gamestate> {
        AnyWidget { 
            implementation: Box::new(widget)
        }
    }

    pub fn estimate_dimensions(&self, ui: &UI, width: isize) -> WidgetDimensions {
        self.implementation.poly_estimate_dimensions(ui, width)
    }

    pub fn draw<'frame, X: Widgetlike<'gamestate>>(&self, brush: Brush, menu: WidgetMenu<'gamestate, 'frame, X>) {
        let ui = menu.ui;
        let menu = menu.menu;

        self.implementation.poly_draw(ui, brush, menu);
    }

    pub(crate) fn clear_layout_cache_if_needed(&self, ui: &UI) {
        self.implementation.poly_clear_layout_cache_if_needed(ui)
    }
}

trait AWidget<'gamestate>: 'gamestate {
    fn poly_estimate_dimensions(&self, ui: &UI, width: isize) -> WidgetDimensions;
    fn poly_draw<'frame>(&self, ui: UI, brush: Brush, menu: Menu<'frame>)
    where 'gamestate: 'frame;
    fn poly_clear_layout_cache_if_needed(&self, ui: &UI);
}

impl<'gamestate, T: Widgetlike<'gamestate>> AWidget<'gamestate> for Widget<'gamestate, T> {
    fn poly_estimate_dimensions(&self, ui: &UI, width: isize) -> WidgetDimensions {
        self.estimate_dimensions(ui, width)
    }

    fn poly_draw<'frame>(&self, ui: UI, brush: Brush, menu: Menu<'frame>) 
    where 'gamestate: 'frame {
        self.draw(ui, brush, menu)
    }

    fn poly_clear_layout_cache_if_needed(&self, ui: &UI) {
        self.clear_layout_cache_if_needed(ui)
    }
}