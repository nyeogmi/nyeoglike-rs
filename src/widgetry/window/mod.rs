use chiropterm::{Brush, FSem};
use euclid::size2;

use super::{UI, Widget, WidgetDimensions, WidgetMenu, Widgetlike, look_and_feel::WindowBorders, widget::AnyWidget};

pub type Window<'gamestate, Out> = Widget<'gamestate, WindowState<'gamestate, Out>, Out>;

// TODO: Support a w95-ish border type too

pub struct WindowState<'gamestate, Out> {
    widget: Option<AnyWidget<'gamestate, Out>>,
}

impl<'gamestate, Out> Default for WindowState<'gamestate, Out> {
    fn default() -> Self {
        WindowState { 
            widget: None,
        }
    }
}

impl<'gamestate, Out: 'gamestate> Widgetlike<'gamestate> for WindowState<'gamestate, Out> {
    type Out = Out;

    fn draw<'frame>(&self, _selected: bool, brush: Brush, menu: WidgetMenu<'gamestate, 'frame, Self, Self::Out>) {
        brush.fill(FSem::new().color(menu.ui.theme().window.color));

        let inner = match menu.ui.theme().window.borders {
            WindowBorders::W95 { bevel } => {
                brush.bevel_w95(bevel);
                brush.region(brush.rect().inflate(-1, -1))
            }
            WindowBorders::DOS {  } => {
                brush.draw_box(false);  // TODO. Box and box color in theme
                brush.region(brush.rect().inflate(-1, -2))
            }
        };

        match &self.widget {
            Some(x) => x.draw(inner, menu),
            None => {}
        }
    }

    fn estimate_dimensions(&self, ui: &UI, width: isize) -> WidgetDimensions {
        let ((pad_x, pad_y), (align_x, align_y)) = match ui.theme().window.borders {
            WindowBorders::W95 { bevel: _ } => {
                // TODO: Accommodate more if there's a title bar
                ((2, 2), (1, 1))
            }
            WindowBorders::DOS {  } => {
                ((2, 4), (1, 2))
            }
        };
        let mut d1 = if let Some(w) = self.widget.as_ref() {
            w.estimate_dimensions(ui, width)
        } else {
            WidgetDimensions::zero()
        };
        d1 = d1.shape_to_align(size2(align_x, align_y));
        d1 = d1.increase(size2(pad_x, pad_y));
        d1.max = size2(1600, 1600);  // pointlessly huge
        d1.align_size_to = size2(align_x, align_y);
        d1
    }

    fn clear_layout_cache(&self, ui: &UI) { 
        if let Some(w) = self.widget.as_ref() {
            w.clear_layout_cache_if_needed(ui)
        }
    }
}

impl<'gamestate, Out: 'gamestate> WindowState<'gamestate, Out> {
    pub fn set_widget<X: Widgetlike<'gamestate, Out=Out>>(&mut self, w: Widget<'gamestate, X, Out>) {
        self.widget = Some(AnyWidget::wrap(w))
    }
}