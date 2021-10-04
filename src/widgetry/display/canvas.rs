use std::marker::PhantomData;

use chiropterm::{Brush};

use crate::widgetry::{InternalWidgetDimensions, UI, Widget, WidgetMenu, Widgetlike, widget::LayoutHacks};

pub type Canvas<'gamestate, Out> = Widget<'gamestate, CanvasState<'gamestate, Out>, Out>;

pub struct CanvasState<'gamestate, Out> {
    pub layout_hacks: LayoutHacks,
    pub draw: Option<Box<dyn 'gamestate+for<'frame> Fn(Brush, WidgetMenu<'gamestate, 'frame, CanvasState<Out>, Out>)>>,
    phantom: PhantomData<*const Out>,
}

impl<'gamestate, Out> CanvasState<'gamestate, Out> {
    pub fn set_draw(&mut self, draw: impl 'gamestate+for<'frame> Fn(Brush, WidgetMenu<'gamestate, 'frame, CanvasState<Out>, Out>)) {
        self.draw = Some(Box::new(draw))
    }
}

impl <'gamestate, Out: 'gamestate> Widgetlike<'gamestate> for CanvasState<'gamestate, Out> {
    type Out = Out;

    fn create() -> Self {
        Self {
            layout_hacks: LayoutHacks::new(),
            draw: None,
            phantom: PhantomData,
        }
    }

    fn draw<'frame>(&self, _selected: bool, brush: Brush, menu: WidgetMenu<'gamestate, 'frame, Self, Self::Out>) {
        if let Some(d) = &self.draw {
            d(brush, menu)
        }
    }

    fn estimate_dimensions(&self, _: &UI, _width: isize) -> InternalWidgetDimensions {
        // let the client use layout hacks
        InternalWidgetDimensions::zero()
    }

    fn clear_layout_cache(&self, _: &UI) { }

    fn layout_hacks(&self) -> LayoutHacks { self.layout_hacks }
}