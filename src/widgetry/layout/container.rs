use chiropterm::{Brush};

use crate::widgetry::{InternalWidgetDimensions, UI, Widget, WidgetMenu, Widgetlike, widget::{AnyWidget, LayoutHacks}};

// Smallvec size -- set this to "higher than most users will ever put in one column/row"
pub type Container<'gamestate, Out> = Widget<'gamestate, ContainerState<'gamestate, Out>, Out>;

pub struct ContainerState<'gamestate, Out> {
    widget: Option<AnyWidget<'gamestate, Out>>,

    pub layout_hacks: LayoutHacks,
}

impl<'gamestate, Out: 'gamestate> Widgetlike<'gamestate> for ContainerState<'gamestate, Out> {
    type Out = Out;

    fn create() -> Self {
        ContainerState { 
            widget: None,
            layout_hacks: LayoutHacks::new(),
        }
    }

    fn draw<'frame>(&self, _: bool, brush: Brush, menu: WidgetMenu<'gamestate, 'frame, ContainerState<'gamestate, Out>, Out>) {
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

impl<'gamestate, Out: 'gamestate> ContainerState<'gamestate, Out> {
    pub fn set<X: Widgetlike<'gamestate, Out=Out>>(&mut self, w: Widget<'gamestate, X, Out>) {
        self.widget = Some(AnyWidget::wrap(w))
    }
}
