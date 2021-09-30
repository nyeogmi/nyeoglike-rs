use chiropterm::{Brush, Brushable, Keycode, Menu};

use super::{Widget, Widgetlike, widget::WidgetMenu};

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

        brush.putfs(&self.text);
    }
}