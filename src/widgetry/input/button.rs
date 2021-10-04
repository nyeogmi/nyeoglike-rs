use std::{marker::PhantomData};

use chiropterm::{Brush, Brushable, MouseEvent, Signal, Stamp};
use euclid::{rect, size2};

use crate::widgetry::{InternalWidgetDimensions, UI, Widget, WidgetCommon, WidgetMenu, Widgetlike, widget::LayoutHacks};

pub type Button<'gamestate, Out> = Widget<'gamestate, ButtonState<'gamestate, Out>, Out>;

// TODO: Hotkeys
pub struct ButtonState<'gamestate, Out> {
    pub text: String,
    pub command: Option<Box<dyn 'gamestate+FnMut(UI, &mut WidgetCommon<ButtonState<'gamestate, Out>>, MouseEvent) -> Signal<Out>>>,

    pub layout_hacks: LayoutHacks,
    phantom: PhantomData<*const Out>,
}

impl<'gamestate, Out> Default for ButtonState<'gamestate, Out> {
    fn default() -> Self {
        Self {
            text: "".to_owned(),
            command: None,

            layout_hacks: LayoutHacks::new(),
            phantom: PhantomData,
        }
    }
}

impl<'gamestate, Out> ButtonState<'gamestate, Out> {
    pub fn set_command(&mut self, cmd: impl 'gamestate+FnMut(UI, &mut WidgetCommon<ButtonState<'gamestate, Out>>, MouseEvent) -> Signal<Out>) {
        self.command = Some(Box::new(cmd))
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
                MouseEvent::Scroll(_, _, _) => {}
            };
            Signal::Continue
        });

        let theme = menu.ui.theme().button;
        brush.bevel_w95(theme.bevel);
        brush.interactor(click_interactor, theme.preclick).putfs(&self.text);
    }

    fn estimate_dimensions(&self, _ui: &UI, width: isize) -> InternalWidgetDimensions {
        // TODO: Find a more efficient way to do this
        let stamp = Stamp::new();
        let brush = stamp.brush_at(rect(0, 0, width, isize::MAX));
        brush.putfs(&self.text);
        InternalWidgetDimensions {
            min: size2(8.min(self.text.len() as isize), 2),
            preferred: stamp.rect().size,
            // TODO: Better foundation for this number
            max: Some(size2(self.text.len() as isize, 2)),
            align_size_to: size2(1, 2),
            horizontal_spacer_count: 0,
            vertical_spacer_count: 0,
        }
    }

    fn clear_layout_cache(&self, _: &UI) { }

    fn layout_hacks(&self) -> LayoutHacks { self.layout_hacks }
}