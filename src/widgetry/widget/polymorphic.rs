use chiropterm::{Brush, Menu};

use crate::widgetry::UI;

use super::{Widget, WidgetDimensions, WidgetMenu, Widgetlike};

pub struct AnyWidget<'gamestate, Out> {
    implementation: Box<dyn AWidget<'gamestate, Out>>,
}

impl<'gamestate, Out: 'gamestate> AnyWidget<'gamestate, Out> {
    pub fn wrap<X: Widgetlike<'gamestate, Out=Out>>(widget: Widget<'gamestate, X, Out>) -> AnyWidget<'gamestate, Out> {
        AnyWidget { 
            implementation: Box::new(widget)
        }
    }

    pub fn estimate_dimensions(&self, width: isize) -> WidgetDimensions {
        self.implementation.poly_estimate_dimensions(width)
    }

    pub fn draw<'frame, X: Widgetlike<'gamestate, Out=Out>>(&self, brush: Brush, menu: WidgetMenu<'gamestate, 'frame, X, Out>) {
        let ui = menu.ui;
        let menu = menu.menu;

        self.implementation.poly_draw(ui, brush, menu);
    }
}

trait AWidget<'gamestate, Out>: 'gamestate {
    fn poly_estimate_dimensions(&self, width: isize) -> WidgetDimensions;
    fn poly_draw<'frame>(&self, ui: UI, brush: Brush, menu: Menu<'frame, Out>)
    where 'gamestate: 'frame;
}

impl<'gamestate, T: Widgetlike<'gamestate, Out=Out>, Out: 'gamestate> AWidget<'gamestate, Out> for Widget<'gamestate, T, Out> {
    fn poly_estimate_dimensions(&self, width: isize) -> WidgetDimensions {
        self.estimate_dimensions(width)
    }

    fn poly_draw<'frame>(&self, ui: UI, brush: Brush, menu: Menu<'frame, Out>) 
    where 'gamestate: 'frame {
        self.draw(ui, brush, menu)
    }
}