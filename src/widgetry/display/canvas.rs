use chiropterm::{Brush};

use crate::widgetry::{InternalWidgetDimensions, UI, Widget, WidgetMenu, Widgetlike, widget::LayoutHacks};

pub type Canvas<'gamestate> = Widget<'gamestate, CanvasState<'gamestate>>;

pub struct CanvasState<'gamestate> {
    pub layout_hacks: LayoutHacks,
    pub draw: Option<Box<dyn 'gamestate+for<'frame> Fn(Brush, WidgetMenu<'gamestate, 'frame, CanvasState>)>>,
}

impl<'gamestate> CanvasState<'gamestate> {
    pub fn set_draw(&mut self, draw: impl 'gamestate+for<'frame> Fn(Brush, WidgetMenu<'gamestate, 'frame, CanvasState>)) {
        self.draw = Some(Box::new(draw))
    }
}

impl <'gamestate> Widgetlike<'gamestate> for CanvasState<'gamestate> {
    fn create() -> Self {
        Self {
            layout_hacks: LayoutHacks::new(),
            draw: None,
        }
    }

    fn draw<'frame>(&self, _selected: bool, brush: Brush, menu: WidgetMenu<'gamestate, 'frame, Self>) {
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