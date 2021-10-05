use chiropterm::*;
use euclid::{rect, size2};

use crate::widgetry::{InternalWidgetDimensions, UI, Widget, Widgetlike, widget::{LayoutHacks, WidgetMenu}};

pub type InputBox<'gamestate> = Widget<'gamestate, InputBoxState>;

pub struct InputBoxState {
    text: String,
    cursor_l: usize,
    cursor_r: usize,
    // TODO: Store left position of window

    pub layout_hacks: LayoutHacks,
}
    
impl<'gamestate> Widgetlike<'gamestate> for InputBoxState {
    fn create() -> Self {
        Self { 
            text: "".to_owned(),
            cursor_l: 0,
            cursor_r: 0,

            layout_hacks: LayoutHacks::new(),
        }
    }

    fn draw<'frame>(&self, selected: bool, brush: Brush, menu: WidgetMenu<'gamestate, 'frame, InputBoxState>) {
        if selected {
            menu.on_text_hprio( |_, this, character| { this.unique.type_character(character); Signal::Continue });
            menu.on_key_hprio( Keycode::Backspace, |_, this, _| {this.unique.backspace(); Signal::Continue });
            menu.on_key_hprio( Keycode::Delete, |_, this, _| {this.unique.delete(); Signal::Continue });
            menu.on_key_hprio( Keycode::Left, |_, this, _| {this.unique.move_cursor(-1); Signal::Continue });
            menu.on_key_hprio( Keycode::Right, |_, this, _| {this.unique.move_cursor(1); Signal::Continue });
            menu.on_key_hprio( Keycode::Home, |_, this, _| {this.unique.set_cursor(0); Signal::Continue });
            menu.on_key_hprio( Keycode::End, |_, this, _| {this.unique.set_cursor(this.unique.text.len()); Signal::Continue });
            menu.on_key_hprio(Keycode::Enter, |ui, this, _| {
                ui.deselect(this);
                Signal::Continue  // TODO: Make the UI hit the submit button
            })
        }

        let click_interactor = menu.on_click(move |ui, this, click: MouseEvent| {
            match click {
                MouseEvent::Click(MouseButton::Left, point, _) => {
                    ui.select(this);
                    this.unique.set_cursor(point.x as usize)
                },
                MouseEvent::Click(_, _, _) => {}
                MouseEvent::Up(_, _, _) => {}
                MouseEvent::Drag {
                    mouse_button: MouseButton::Left,
                    start_point, now_point, 
                    ..
                } => {
                    if start_point.x < 0 { return Signal::Continue; } // should be impossible
                    let now_x = now_point.x.max(0) as usize;

                    this.unique.highlight(start_point.x as usize, now_x);
                },
                MouseEvent::Drag {..} => {}
                MouseEvent::Scroll(_, _, _) => {}
            };
            Signal::Continue
        });

        let theme = menu.ui.theme().input_box;
        brush.fill(FSem::new().color(if selected { theme.selected } else { theme.deselected }));
        brush.bevel_w95(theme.bevel);
        brush.putfs(&self.text);  // TODO: Don't wrap?


        // make clickable
        brush.interactor(click_interactor, theme.preclick).fill(FSem::new());

        // draw cursor
        if selected {
            let cursor_region = brush.region(rect(
                self.cursor_l as isize, 0, 
                (self.cursor_r as isize - self.cursor_l as isize + 1).max(1), 2
            ));
            cursor_region.interactor(click_interactor, theme.cursor).fill(FSem::new().color(theme.cursor));
        }
    }

    fn estimate_dimensions(&self, _: &UI, _width: isize) -> InternalWidgetDimensions {
        InternalWidgetDimensions { 
            min: size2(8, 2),
            preferred: size2(8.max(self.text.len() as isize), 2),
            max: None,
            align_size_to: size2(1, 2),
            horizontal_spacer_count: 0,
            vertical_spacer_count: 0,
        }
    }

    fn clear_layout_cache(&self, _: &UI) { }

    fn layout_hacks(&self) -> LayoutHacks { self.layout_hacks }
}

impl InputBoxState {
    fn type_character(&mut self, character: char) {
        if self.cursor_l != self.cursor_r {
            self.text.drain(self.cursor_l..self.cursor_r + 1);
            self.cursor_r = self.cursor_l;
        }

        if self.cursor_l < self.text.len() {
            self.text.insert(self.cursor_l, character)
        } else {
            self.text.push(character)
        }
        self.cursor_l += 1;
        self.cursor_r += 1;
    }

    fn backspace(&mut self) {
        if self.cursor_l != self.cursor_r {
            self.text.drain(self.cursor_l..self.cursor_r + 1);
            self.cursor_r = self.cursor_l;
            return
        }

        if self.cursor_l == 0 {
            return;
        }
        else {
            self.text.remove(self.cursor_l - 1);
            self.cursor_l -= 1;
            self.cursor_r = self.cursor_l;
        }
    }

    fn delete(&mut self) {
        if self.cursor_l != self.cursor_r {
            self.text.drain(self.cursor_l..self.cursor_r + 1);
            self.cursor_r = self.cursor_l;
            return
        }

        if self.cursor_l >= self.text.len() {
            return;
        }
        else {
            self.text.remove(self.cursor_l);
        }
    }

    fn set_cursor(&mut self, value: usize) {
        self.cursor_l = value;
        self.cursor_r = value;
        self.cursor_fixup()
    }

    fn move_cursor(&mut self, amount: isize) {
        if amount > 0 {
            // move right, so start from right cursor
            self.cursor_l = self.cursor_r;
        }

        let mut cursor_2 = self.cursor_l as isize + amount;
        if cursor_2 < 0 { cursor_2 = 0; }
        if cursor_2 > self.text.len() as isize { cursor_2 = self.text.len() as isize; }
        self.cursor_l = cursor_2 as usize;
        self.cursor_r = self.cursor_l;
    }

    fn highlight(&mut self, i0: usize, i1: usize) {
        let l = i0.min(i1);
        let r = i0.max(i1);

        self.cursor_l = l;
        self.cursor_r = r;
        self.cursor_fixup()
    }

    fn cursor_fixup(&mut self) {
        if self.cursor_l > self.text.len() { self.cursor_l = self.text.len() }
        if self.cursor_r > self.text.len() { self.cursor_r = self.text.len() }
        if self.cursor_r != self.cursor_l && self.cursor_r == self.text.len() {
            self.cursor_r = self.text.len() - 1
        }
    }
}