use std::{marker::PhantomData, mem};

use chiropterm::{Brush, Brushable, MouseEvent, Signal, Stamp, colors::*};
use euclid::{rect, size2};

use crate::widgetry::{UI, Widget, WidgetCommon, WidgetDimensions, WidgetMenu, Widgetlike};

pub type Button<'gamestate, Out> = Widget<'gamestate, ButtonState<'gamestate, Out>, Out>;

// TODO: Hotkeys
pub struct ButtonState<'gamestate, Out> {
    pub text: String,
    pub command: Option<Box<dyn 'gamestate+FnMut(UI, &mut WidgetCommon<ButtonState<'gamestate, Out>>, MouseEvent) -> Signal<Out>>>,

    phantom: PhantomData<*const Out>,
}

impl<'gamestate, Out> Default for ButtonState<'gamestate, Out> {
    fn default() -> Self {
        Self {
            text: "".to_owned(),
            command: None,

            phantom: PhantomData,
        }
    }
}

impl <'gamestate, Out: 'gamestate> Widgetlike<'gamestate> for ButtonState<'gamestate, Out> {
    type Out = Out;

    fn draw<'frame>(&self, _selected: bool, brush: Brush, menu: WidgetMenu<'gamestate, 'frame, Self, Self::Out>) {
        let click_interactor = menu.on_click(move |ui, this, click: MouseEvent| {
            match click {
                MouseEvent::Click(_, _, _) => {
                    ui.select(this); // this button can be selected, not that it matters
                    let command = this.unique.command.take();
                    if let Some(mut c) = command {
                        let result = c(ui, this, click);
                        this.unique.command.replace(c);
                        return result
                    }
                },
                MouseEvent::Up(_, _, _) => {}
                MouseEvent::Drag {..} => {}
            };
            Signal::Continue
        });

        let theme = menu.ui.theme().button;
        brush.bevel_w95(theme.bevel);
        brush.interactor(click_interactor, theme.preclick).putfs(&self.text);
    }

    fn estimate_dimensions(&self, ui: &UI, width: isize) -> WidgetDimensions {
        // TODO: Find a more efficient way to do this
        let stamp = Stamp::new();
        let brush = stamp.brush_at(rect(0, 0, width, isize::MAX));
        brush.putfs(&self.text);
        WidgetDimensions {
            min: size2(8.min(self.text.len() as isize), 2),
            preferred: stamp.rect().size,
            // TODO: Better foundation for this number
            max: size2(self.text.len() as isize, 2),
        }
    }

    fn clear_layout_cache(&self, _: &UI) { }
}