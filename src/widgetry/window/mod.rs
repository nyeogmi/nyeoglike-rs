use chiropterm::*;
use euclid::{rect, size2};

use super::{UI, Widget, WidgetDimensions, WidgetMenu, Widgetlike, look_and_feel::WindowBorders, widget::AnyWidget};

pub type Window<'gamestate, Out> = Widget<'gamestate, WindowState<'gamestate, Out>, Out>;

// TODO: Support a w95-ish border type too

pub struct WindowState<'gamestate, Out> {
    pub title: Option<String>,
    widget: Option<AnyWidget<'gamestate, Out>>,
}

impl<'gamestate, Out> Default for WindowState<'gamestate, Out> {
    fn default() -> Self {
        WindowState { 
            title: None,
            widget: None,
        }
    }
}

impl<'gamestate, Out: 'gamestate> Widgetlike<'gamestate> for WindowState<'gamestate, Out> {
    type Out = Out;

    fn draw<'frame>(&self, _selected: bool, brush: Brush, menu: WidgetMenu<'gamestate, 'frame, Self, Self::Out>) {
        brush.fill(FSem::new().color(menu.ui.theme().window.color));

        let inner = match menu.ui.theme().window.borders {
            WindowBorders::W95 { bevel, title: title_color } => {
                brush.bevel_w95(bevel);

                if let Some(title) = &self.title {
                    let title_bar_outer = rect(0, 0, brush.rect().size.width, 2);
                    let title_bar_inner = rect(1, 0, brush.rect().size.width - 2, 2);

                    // get rid of w95 gradients around the title bar
                    brush.region(title_bar_outer).bevel_bottom(255);
                    brush.region(title_bar_outer).bevel_left(255);
                    brush.region(title_bar_outer).bevel_top(255);

                    draw_gradient(brush.region(title_bar_outer), [title_color.0[1], title_color.0[2]]);

                    brush.region(title_bar_inner).fg(title_color.1).putfs(title);

                    let inner = rect(1, 2, brush.rect().size.width - 2, brush.rect().size.height - 3);
                    brush.region(inner)
                }
                else {
                    brush.region(brush.rect().inflate(-1, -1))
                }

                // TODO: Use title
            }
            WindowBorders::DOS {  } => {
                brush.draw_box(false);  // TODO. Box and box color in theme
                if let Some(title) = &self.title {
                    brush.region(rect(2, 0, brush.rect().size.width - 4, 2)).putfs(title);
                }
                brush.region(brush.rect().inflate(-2, -2))
            }
        };

        match &self.widget {
            Some(x) => x.draw(inner, menu),
            None => {}
        }
    }

    fn estimate_dimensions(&self, ui: &UI, width: isize) -> WidgetDimensions {
        let ((pad_x, pad_y), (align_x, align_y)) = match ui.theme().window.borders {
            WindowBorders::W95 { .. } => {
                if self.title.is_some() {
                    ((2, 3), (1, 1))
                } else {
                    ((2, 2), (1, 1))
                }
            }
            WindowBorders::DOS {  } => {
                ((4, 4), (2, 2))
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
        // don't pass spacers through
        d1.horizontal_spacer_count = 0;
        d1.vertical_spacer_count = 0;
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

fn draw_gradient(brush: Brush, color_opts: [u8; 2]) {
    let glyph_opts = [(false, 0), (false, 0xb0), (false, 0xb1), /* (true, 0xb0) */];  // commented out for trypophobia reasons for now
    let n_opts = color_opts.len() * glyph_opts.len() - (glyph_opts.len() - 1);
    for (y0, h, x_offset) in [(0, 1, 0), (1, 1, -1)] {
        for i in 0..n_opts as isize {
            let x0 = (i * brush.rect().size.width) / n_opts as isize + x_offset;
            let x1 = ((i + 1) * brush.rect().size.width) / n_opts as isize; // + x_offset;
            let r = rect(x0, y0, x1 - x0, h);

            let color_ix = (i/glyph_opts.len() as isize) as usize;
            let last_color = color_opts[color_ix];
            let next_color = color_opts[(color_ix + 1).min(color_opts.len() - 1)];
            let (flip, c) = glyph_opts[i as usize % glyph_opts.len()];
            let glyph = SemanticContent::Small(c);

            let (bg, fg) = if flip { (next_color, last_color) } else { (last_color, next_color) };

            brush.region(r).fill(FSem::new().bg(bg).fg(fg).sem(glyph));
        }
    }
}