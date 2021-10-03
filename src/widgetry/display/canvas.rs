use std::marker::PhantomData;

use chiropterm::{Brush};

use crate::widgetry::{UI, Widget, WidgetDimensions, WidgetMenu, Widgetlike, widget::LayoutHacks};

pub type Canvas<'gamestate, Out> = Widget<'gamestate, CanvasState<'gamestate, Out>, Out>;

pub struct CanvasState<'gamestate, Out> {
    pub layout_hacks: LayoutHacks,
    pub draw: Option<Box<dyn 'gamestate+for<'frame> Fn(Brush, WidgetMenu<'gamestate, 'frame, CanvasState<Out>, Out>)>>,
    phantom: PhantomData<*const Out>,
}

impl<'gamestate, Out> Default for CanvasState<'gamestate, Out> {
    fn default() -> Self {
        Self {
            layout_hacks: LayoutHacks::new(),
            draw: None,
            phantom: PhantomData,
        }
    }
}

impl<'gamestate, Out> CanvasState<'gamestate, Out> {
    pub fn set_draw(&mut self, draw: impl 'gamestate+for<'frame> Fn(Brush, WidgetMenu<'gamestate, 'frame, CanvasState<Out>, Out>)) {
        self.draw = Some(Box::new(draw))
    }
}

impl <'gamestate, Out: 'gamestate> Widgetlike<'gamestate> for CanvasState<'gamestate, Out> {
    type Out = Out;

    fn draw<'frame>(&self, _selected: bool, brush: Brush, menu: WidgetMenu<'gamestate, 'frame, Self, Self::Out>) {
        if let Some(d) = &self.draw {
            d(brush, menu)
        }
    }

    fn estimate_dimensions(&self, _: &UI, _width: isize) -> WidgetDimensions {
        // NOTE: Use the layout hacks for this
        let mut wd = WidgetDimensions::zero();
        wd.max.width = 1600;
        wd.max.height = 1600;
        wd
    }

    fn clear_layout_cache(&self, _: &UI) { }

    fn layout_hacks(&self) -> LayoutHacks { self.layout_hacks }
}