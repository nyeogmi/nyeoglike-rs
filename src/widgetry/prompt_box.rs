use chiropterm::*;
use euclid::size2;

use super::{Widget, WidgetDimensions, Widgetlike, widget::WidgetMenu};

pub type PromptBox = Widget<PromptBoxState>;

pub struct PromptBoxState {
    text: String,
}

impl Default for PromptBoxState {
    fn default() -> Self {
        Self { text: "".to_owned() }
    }
}
    
impl Widgetlike for PromptBoxState {
    fn draw<T: Brushable>(&self, brush: Brush<T>, menu: &WidgetMenu<PromptBoxState>) {
        menu.on_text( |slf, character| { slf.text.push(character); });
        menu.on_key( Keycode::Backspace, |slf, _| {slf.text.pop(); });

        brush.fill(FSem::new().bg(colors::Green[3]));
        brush.bevel_w95(colors::Dark[0], colors::Light[3]);
        brush.putfs(&self.text);
    }

    fn estimate_dimensions(&self, width: isize) -> super::WidgetDimensions {
        WidgetDimensions { 
            min: size2(8, 2),
            preferred: size2(8.max(self.text.len() as isize), 2),
            max: size2(width, 2),
        }
    }
}