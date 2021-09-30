use chiropterm::*;
use euclid::{rect, size2};

use super::{Widget, WidgetDimensions, Widgetlike, widget::WidgetMenu};

pub type InputBox = Widget<InputBoxState>;

pub struct InputBoxState {
    text: String,
    cursor: usize,
    // TODO: Highlighting (click/drag etc)
}

impl Default for InputBoxState {
    fn default() -> Self {
        Self { 
            text: "".to_owned(),
            cursor: 0,
        }
    }
}
    
impl Widgetlike for InputBoxState {
    fn draw<T: Brushable>(&self, brush: Brush<T>, menu: &WidgetMenu<InputBoxState>) {
        menu.on_text( |slf, character| { slf.type_character(character); });
        menu.on_key( Keycode::Backspace, |slf, _| {slf.backspace(); });
        menu.on_key( Keycode::Delete, |slf, _| {slf.delete(); });
        menu.on_key( Keycode::Left, |slf, _| {slf.move_cursor(-1); });
        menu.on_key( Keycode::Right, |slf, _| {slf.move_cursor(1); });
        menu.on_key( Keycode::Home, |slf, _| {slf.set_cursor(0); });
        menu.on_key( Keycode::End, |slf, _| {slf.set_cursor(slf.text.len()); });

        let offset = brush.cursor_offset();
        let click_interactor = menu.on_click(move |slf, click: MouseEvent| {
            let rel = click.offset(-offset);
            match rel {
                MouseEvent::Click(MouseButton::Left, point, _) => {
                    slf.set_cursor(point.x as usize)
                },
                MouseEvent::Click(_, _, _) => {}
                MouseEvent::Up(_, _, _) => {}
            }
        });

        brush.fill(FSem::new().bg(colors::Green[3]));
        brush.bevel_w95(colors::Dark[0], colors::Light[3]);
        brush.putfs(&self.text);

        // draw cursor
        brush.region(rect(self.cursor as isize, 0, 1, 2)).fill(FSem::new().bg(colors::Orange[2]));

        // make clickable
        brush.interactor(click_interactor).fill(FSem::new())
    }

    fn estimate_dimensions(&self, width: isize) -> super::WidgetDimensions {
        WidgetDimensions { 
            min: size2(8, 2),
            preferred: size2(8.max(self.text.len() as isize), 2),
            max: size2(width, 2),
        }
    }
}
impl InputBoxState {
    fn type_character(&mut self, character: char) {
        if self.cursor < self.text.len() {
            self.text.insert(self.cursor, character)
        } else {
            self.text.push(character)
        }
        self.cursor += 1;
    }

    fn backspace(&mut self) {
        if self.cursor == 0 {
            return;
        }
        else {
            self.text.remove(self.cursor - 1);
            self.cursor -= 1;
        }
    }

    fn delete(&mut self) {
        if self.cursor >= self.text.len() {
            return;
        }
        else {
            self.text.remove(self.cursor);
        }
    }

    fn set_cursor(&mut self, value: usize) {
        self.move_cursor(value as isize - self.cursor as isize)
    }

    fn move_cursor(&mut self, amount: isize) {
        let mut cursor_2 = self.cursor as isize + amount;
        if cursor_2 < 0 { cursor_2 = 0; }
        if cursor_2 > self.text.len() as isize { cursor_2 = self.text.len() as isize; }
        self.cursor = cursor_2 as usize;
    }
}