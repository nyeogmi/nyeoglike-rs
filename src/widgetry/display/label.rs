use chiropterm::{Brush, Brushable, Stamp};
use euclid::{rect, size2};

use crate::widgetry::{InternalWidgetDimensions, UI, Widget, WidgetMenu, Widgetlike, widget::LayoutHacks};

pub type Label<'gamestate> = Widget<'gamestate, LabelState>;

pub struct LabelState {
    pub text: String,

    pub layout_hacks: LayoutHacks,
}

impl <'gamestate> Widgetlike<'gamestate> for LabelState {
    fn create() -> Self {
        Self {
            text: "".to_owned(),

            layout_hacks: LayoutHacks::new(),
        }
    }

    fn draw<'frame>(&self, _selected: bool, brush: Brush, _menu: WidgetMenu<'gamestate, 'frame, Self>) {
        brush.putfs(&self.text);
    }

    fn estimate_dimensions(&self, _: &UI, width: isize) -> InternalWidgetDimensions {
        // TODO: Find a more efficient way to do this
        let stamp = Stamp::new();
        let brush = stamp.brush_at(rect(0, 0, width, isize::MAX));
        brush.putfs(&self.text);
        InternalWidgetDimensions {
            min: size2(8.min(self.text.len() as isize), 2),
            preferred: stamp.rect().size,
            // TODO: Better foundation for this number
            max: None,
            align_size_to: size2(1, 2),
            horizontal_spacer_count: 0,
            vertical_spacer_count: 0,
        }
    }

    fn clear_layout_cache(&self, _: &UI) { }

    fn layout_hacks(&self) -> LayoutHacks { self.layout_hacks }
}