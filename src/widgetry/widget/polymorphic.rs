use chiropterm::{Brush, Menu};

use crate::widgetry::UI;

use super::{WidgetDimensions, Widget, WidgetMenu, Widgetlike};

pub struct AnyWidget<'gamestate, Out> {
    implementation: Box<dyn AWidget<'gamestate, Out>>,
}

impl<'gamestate, Out: 'gamestate> AnyWidget<'gamestate, Out> {
    pub fn wrap<X: Widgetlike<'gamestate, Out=Out>>(widget: Widget<'gamestate, X, Out>) -> AnyWidget<'gamestate, Out> {
        AnyWidget { 
            implementation: Box::new(widget)
        }
    }

    pub fn estimate_dimensions(&self, ui: &UI, width: isize) -> WidgetDimensions {
        self.implementation.poly_estimate_dimensions(ui, width)
    }

    pub fn draw<'frame, X: Widgetlike<'gamestate, Out=Out>>(&self, brush: Brush, menu: WidgetMenu<'gamestate, 'frame, X, Out>) {
        let ui = menu.ui;
        let menu = menu.menu;

        self.implementation.poly_draw(ui, brush, menu);
    }

    pub(crate) fn clear_layout_cache_if_needed(&self, ui: &UI) {
        self.implementation.poly_clear_layout_cache_if_needed(ui)
    }
}

trait AWidget<'gamestate, Out>: 'gamestate {
    fn poly_estimate_dimensions(&self, ui: &UI, width: isize) -> WidgetDimensions;
    fn poly_draw<'frame>(&self, ui: UI, brush: Brush, menu: Menu<'frame, Out>)
    where 'gamestate: 'frame;
    fn poly_clear_layout_cache_if_needed(&self, ui: &UI);
}

impl<'gamestate, T: Widgetlike<'gamestate, Out=Out>, Out: 'gamestate> AWidget<'gamestate, Out> for Widget<'gamestate, T, Out> {
    fn poly_estimate_dimensions(&self, ui: &UI, width: isize) -> WidgetDimensions {
        self.estimate_dimensions(ui, width)
    }

    fn poly_draw<'frame>(&self, ui: UI, brush: Brush, menu: Menu<'frame, Out>) 
    where 'gamestate: 'frame {
        self.draw(ui, brush, menu)
    }

    fn poly_clear_layout_cache_if_needed(&self, ui: &UI) {
        self.clear_layout_cache_if_needed(ui)
    }
}