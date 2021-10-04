use std::{cell::Cell};

use chiropterm::{Brush, FSem, MouseButton, MouseEvent, Signal};
use euclid::{rect, vec2};

use crate::widgetry::{InternalWidgetDimensions, UI, Widget, WidgetMenu, Widgetlike, widget::{AnyWidget, LayoutHacks}};

// Smallvec size -- set this to "higher than most users will ever put in one column/row"
pub type Scrollable<'gamestate, Out> = Widget<'gamestate, ScrollableState<'gamestate, Out>, Out>;

pub struct ScrollableState<'gamestate, Out> {
    widget: Option<AnyWidget<'gamestate, Out>>,
    offset: Cell<f64>,

    pub layout_hacks: LayoutHacks,
}

impl<'gamestate, Out> Default for ScrollableState<'gamestate, Out> {
    fn default() -> Self {
        ScrollableState { 
            widget: None,
            offset: Cell::new(0.0),

            layout_hacks: LayoutHacks::new(),
        }
    }
}

impl<'gamestate, Out: 'gamestate> Widgetlike<'gamestate> for ScrollableState<'gamestate, Out> {
    type Out = Out;

    fn draw<'frame>(&self, _: bool, brush: Brush, menu: WidgetMenu<'gamestate, 'frame, ScrollableState<'gamestate, Out>, Out>) {
        if let Some(w) = &self.widget {
            let dims = w.estimate_dimensions(&menu.ui, brush.rect().width() - 2);

            let inner_height = dims.preferred.height;
            let brush_height = brush.rect().height();

            let offset_to_use = self.fix_offset(inner_height, brush_height);

            let space_to_adjust = (inner_height - brush_height).max(0);

            if space_to_adjust >= 0 {
                // TODO: Adjust by a different amount if the scrollbar is small

                let scrollbar = brush.region(rect(brush.rect().width() - 1, 0, 1, brush_height));

                let top_button = scrollbar.region(rect(0, 0, 1, 2));
                let btm_button = scrollbar.region(rect(0, scrollbar.rect().height() - 2, 1, 2));
                
                let position_top = if space_to_adjust == 0 { 0.0 } else { offset_to_use as f64 / inner_height as f64 };
                let position_bot = if space_to_adjust == 0 { 1.0 } else { 
                    (offset_to_use + brush.rect().height()) as f64 / inner_height as f64 
                };

                let scrollable_height = scrollbar.rect().height() - 4;
                let scroll_offset_for = move |dy: f32| {
                    if scrollable_height == 0 { return 0.0; }
                    let scrolls_per_cell = inner_height as f64 / scrollable_height as f64;
                    dy as f64 * scrolls_per_cell
                };

                let ix_top = (scrollable_height as f64 * position_top).floor() as isize;
                let mut ix_bot = (scrollable_height as f64 * position_bot).ceil() as isize;
                if ix_bot == ix_top { ix_bot += 1; }

                let scrollbar_rect = rect(0, ix_top + 2, 1, ix_bot - ix_top);

                let top_button_interactor = menu.on_click(move |_, w, me| {
                    match me {
                        MouseEvent::Click(MouseButton::Left, _, _) => { 
                            w.unique.set_offset(w.unique.offset.get() - scroll_offset_for(1 as f32).max(2.0), inner_height, brush_height); 
                        }
                        MouseEvent::Click(_, _, _) => {}
                        MouseEvent::Up(_, _, _) => {}
                        MouseEvent::Drag { .. } => {}
                        MouseEvent::Scroll(_, _, _) => {}
                    };
                    Signal::Continue
                });

                let btm_button_interactor = menu.on_click(move |_, w, me| {
                    match me {
                        MouseEvent::Click(MouseButton::Left, _, _) => { 
                            w.unique.set_offset(w.unique.offset.get() + scroll_offset_for(1 as f32).max(2.0), inner_height, brush_height); 
                        }
                        MouseEvent::Click(_, _, _) => {}
                        MouseEvent::Up(_, _, _) => {}
                        MouseEvent::Drag { .. } => {}
                        MouseEvent::Scroll(_, _, _) => {}
                    };
                    Signal::Continue
                });

                let bar_interactor = menu.on_click(move |_, w, me| {
                    // let me = me.offset(size2(scrollbar_rect.min_x() - brush.rect().min_x(), 0));
                    // may be no need, X doesn't matter
                    match me {
                        MouseEvent::Click(MouseButton::Left, point, _) => {
                            let scrollbar_center = (ix_top + ix_bot) / 2;
                            w.unique.set_offset(
                                w.unique.offset.get() + scroll_offset_for((point.y - scrollbar_center) as f32),
                                    inner_height, brush_height,
                            ); 
                        }
                        MouseEvent::Click(_, _, _) => {}
                        MouseEvent::Up(_, _, _) => {}
                        MouseEvent::Drag { 
                            mouse_button: MouseButton::Left,
                            last_point,
                            now_point,
                            .. 
                        } => {
                            w.unique.set_offset(
                                w.unique.offset.get() + scroll_offset_for((now_point.y - last_point.y) as f32),
                                inner_height, brush_height,
                            );
                        }
                        MouseEvent::Drag { .. } => {} 
                        MouseEvent::Scroll(amt, _, _) => {
                            w.unique.set_offset(
                                w.unique.offset.get() + scroll_offset_for(amt),
                                inner_height, brush_height,
                            );
                        }
                    }
                    Signal::Continue
                });

                let scrollbar_region = scrollbar.region(scrollbar_rect);
                scrollbar.interactor(bar_interactor, (255, 255)).fill(FSem::new().color(menu.ui.theme().input_box.deselected));
                scrollbar_region.interactor(bar_interactor, menu.ui.theme().button.preclick).fill(FSem::new().color(menu.ui.theme().input_box.cursor));

                top_button.interactor(top_button_interactor, menu.ui.theme().button.preclick).putch(0x1e);
                btm_button.interactor(btm_button_interactor, menu.ui.theme().button.preclick).putch(0x1f);
                brush.dont_interfere_with_interactor().scroll_interactor(bar_interactor).fill(FSem::new());
            }

            w.draw(
                brush.region(
                    rect(0, 0, brush.rect().width() - 2, dims.preferred.height.max(brush_height))
                ).offset_rect(vec2(0, -offset_to_use)), 
                menu.share()
            );
        } 
    }

    fn estimate_dimensions(&self, ui: &UI, width: isize) -> InternalWidgetDimensions {
        if let Some(w) = &self.widget {
            let mut dims = w.estimate_dimensions(ui, width - 2).to_internal();
            dims.min.height = 4; // smallest capable of rendering a scrollbar, for now. consider an even smaller scrollbar later
            dims
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

impl<'gamestate, Out: 'gamestate> ScrollableState<'gamestate, Out> {
    fn fix_offset(&self, inner_height: isize, brush_height: isize) -> isize {
        let space_to_adjust = (inner_height - brush_height).max(0);

        let new_offset = self.offset.get().max(0.0).min(space_to_adjust as f64);
        self.offset.replace(new_offset);
        let mut offset_to_use = self.offset.get() as isize;
        offset_to_use -= offset_to_use % 2;
        offset_to_use
    }

    fn set_offset(&self, new_value: f64, inner_height: isize, brush_height: isize) {
        self.offset.replace(new_value);
        self.fix_offset(inner_height, brush_height);
    }
}

impl<'gamestate, Out: 'gamestate> ScrollableState<'gamestate, Out> {
    pub fn set<X: Widgetlike<'gamestate, Out=Out>>(&mut self, w: Widget<'gamestate, X, Out>) {
        self.widget = Some(AnyWidget::wrap(w))
    }
}
