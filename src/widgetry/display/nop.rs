use chiropterm::Brush;

use crate::widgetry::{InternalWidgetDimensions, UI, Widget, WidgetMenu, Widgetlike, widget::LayoutHacks};

pub type Nop<'gamestate> = Widget<'gamestate, NopState>;

pub struct NopState {
    pub layout_hacks: LayoutHacks,
}

impl <'gamestate> Widgetlike<'gamestate> for NopState {
    fn create() -> Self {
        Self {
            layout_hacks: LayoutHacks::new(),
        }
    }

    fn draw<'frame>(&self, _selected: bool, _brush: Brush, _menu: WidgetMenu<'gamestate, 'frame, Self>) { }

    fn estimate_dimensions(&self, _: &UI, _width: isize) -> InternalWidgetDimensions {
        InternalWidgetDimensions::zero()
    }

    fn clear_layout_cache(&self, _: &UI) { }

    fn layout_hacks(&self) -> LayoutHacks { self.layout_hacks }
}