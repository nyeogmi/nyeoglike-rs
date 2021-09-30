use chiropterm::{Brush, Brushable, Keycode, Menu};

use super::{Widget, Widgetlike, widget::WidgetMenu};

pub type PromptBox = Widget<PromptBoxState>;

pub struct PromptBoxState {
    text: String,
    text_i: usize,
}

impl PromptBoxState {
    pub fn new() -> PromptBoxState {
        PromptBoxState {
            text: "fuck you lamers".to_string(), 
            text_i: 0,
        }
    }

    fn incr_i(&mut self, amt: isize) {
        let mut ti2 = self.text_i as isize + amt;
        if ti2 < 0 { ti2 = 0 }
        if ti2 >= self.text.len() as isize { ti2 = self.text.len() as isize }
        self.text_i = ti2 as usize;
    }
}
    
impl Widgetlike for PromptBoxState {
    fn draw<T: Brushable>(&self, brush: Brush<T>, menu: &WidgetMenu<PromptBoxState>) {
        let interactor = menu.on(Keycode::A, 
            |slf, _| slf.incr_i(1)
        );

        brush.interactor(interactor).putfs(&self.text[..self.text_i]);
    }
}