use std::marker::PhantomData;

use chiropterm::{Brush};
use euclid::{size2};

use crate::widgetry::{InternalWidgetDimensions, UI, Widget, WidgetMenu, Widgetlike, widget::LayoutHacks};

pub type Spacer<'gamestate, Out> = Widget<'gamestate, SpacerState<Out>, Out>;

pub struct SpacerState<Out> {
    phantom: PhantomData<*const Out>,
    pub horiz_count: usize,
    pub vert_count: usize,

    pub layout_hacks: LayoutHacks,
}

impl<Out> Default for SpacerState<Out> {
    fn default() -> Self {
        Self {
            phantom: PhantomData,
            horiz_count: 1,
            vert_count: 1,

            layout_hacks: LayoutHacks::new(),
        }
    }
}

impl <'gamestate, Out: 'gamestate> Widgetlike<'gamestate> for SpacerState<Out> {
    type Out = Out;

    fn draw<'frame>(&self, _: bool, _: Brush, _: WidgetMenu<'gamestate, 'frame, Self, Self::Out>) {
    }

    fn estimate_dimensions(&self, _: &UI, _: isize) -> InternalWidgetDimensions {
        // TODO: Find a more efficient way to do this
        InternalWidgetDimensions {
            min: size2(0, 0),
            preferred: size2(0, 0),
            max: None,
            align_size_to: size2(1, 1),
            horizontal_spacer_count: 1,
            vertical_spacer_count: 1,
        }
    }

    fn clear_layout_cache(&self, _: &UI) { }

    fn layout_hacks(&self) -> LayoutHacks { self.layout_hacks }
}