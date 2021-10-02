use std::marker::PhantomData;

use chiropterm::{Brush, Brushable, Stamp};
use euclid::{rect, size2};

use crate::widgetry::{UI, Widget, WidgetDimensions, WidgetMenu, Widgetlike};

pub type Label<'gamestate, Out> = Widget<'gamestate, LabelState<Out>, Out>;

pub struct LabelState<Out> {
    pub text: String,

    phantom: PhantomData<*const Out>,
}

impl<Out> Default for LabelState<Out> {
    fn default() -> Self {
        Self {
            text: "".to_owned(),

            phantom: PhantomData,
        }
    }
}

impl <'gamestate, Out: 'gamestate> Widgetlike<'gamestate> for LabelState<Out> {
    type Out = Out;

    fn draw<'frame>(&self, _selected: bool, brush: Brush, _menu: WidgetMenu<'gamestate, 'frame, Self, Self::Out>) {
        brush.putfs(&self.text);
    }

    fn estimate_dimensions(&self, _: &UI, width: isize) -> WidgetDimensions {
        // TODO: Find a more efficient way to do this
        let stamp = Stamp::new();
        let brush = stamp.brush_at(rect(0, 0, width, isize::MAX));
        brush.putfs(&self.text);
        WidgetDimensions {
            min: size2(8.min(self.text.len() as isize), 2),
            preferred: stamp.rect().size,
            // TODO: Better foundation for this number
            max: size2(self.text.len() as isize, self.text.len() as isize),
        }
    }

    fn clear_layout_cache(&self, _: &UI) { }
}