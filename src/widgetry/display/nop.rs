use std::marker::PhantomData;

use chiropterm::Brush;

use crate::widgetry::{InternalWidgetDimensions, UI, Widget, WidgetMenu, Widgetlike, widget::LayoutHacks};

pub type Nop<'gamestate, Out> = Widget<'gamestate, NopState<Out>, Out>;

pub struct NopState<Out> {
    pub layout_hacks: LayoutHacks,
    phantom: PhantomData<*const Out>,
}

impl <'gamestate, Out: 'gamestate> Widgetlike<'gamestate> for NopState<Out> {
    type Out = Out;

    fn create() -> Self {
        Self {
            layout_hacks: LayoutHacks::new(),
            phantom: PhantomData,
        }
    }

    fn draw<'frame>(&self, _selected: bool, _brush: Brush, _menu: WidgetMenu<'gamestate, 'frame, Self, Self::Out>) { }

    fn estimate_dimensions(&self, _: &UI, _width: isize) -> InternalWidgetDimensions {
        InternalWidgetDimensions::zero()
    }

    fn clear_layout_cache(&self, _: &UI) { }

    fn layout_hacks(&self) -> LayoutHacks { self.layout_hacks }
}