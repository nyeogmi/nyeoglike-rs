use chiropterm::{Brush};

use crate::widgetry::{InternalWidgetDimensions, UI, Widget, WidgetMenu, Widgetlike, widget::{AnyWidget, LayoutHacks}};

// Smallvec size -- set this to "higher than most users will ever put in one column/row"
pub type Container<'gamestate> = Widget<'gamestate, ContainerState<'gamestate>>;

pub struct ContainerState<'gamestate> {
    widget: Option<AnyWidget<'gamestate>>,

    pub layout_hacks: LayoutHacks,
}

impl<'gamestate> Widgetlike<'gamestate> for ContainerState<'gamestate> {
    fn create() -> Self {
        ContainerState { 
            widget: None,
            layout_hacks: LayoutHacks::new(),
        }
    }

    fn draw<'frame>(&self, _: bool, brush: Brush, menu: WidgetMenu<'gamestate, 'frame, ContainerState<'gamestate>>) {
        if let Some(w) = &self.widget {
            w.draw(brush, menu);
        }
    }

    fn estimate_dimensions(&self, ui: &UI, width: isize) -> InternalWidgetDimensions {
        if let Some(w) = &self.widget {
            w.estimate_dimensions(ui, width).to_internal()
        } else {
            return InternalWidgetDimensions::zero();
        }
    }

    fn clear_layout_cache(&self, ui: &UI) {
        if let Some(w) = &self.widget {
            w.clear_layout_cache_if_needed(ui)
        }
    }

    fn layout_hacks(&self) -> LayoutHacks { 
        self.layout_hacks 
    }
}

impl<'gamestate> ContainerState<'gamestate> {
    pub fn set<X: Widgetlike<'gamestate>>(&mut self, w: Widget<'gamestate, X>) {
        self.widget = Some(AnyWidget::wrap(w))
    }
}
