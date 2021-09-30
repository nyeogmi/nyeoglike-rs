use std::{cell::RefCell, rc::Rc};

use chiropterm::*;

pub struct WidgetMenu<'r, 'a, T: 'a> {
    pub(in super) state: Rc<RefCell<T>>,
    pub(in super) menu: &'r Menu<'a, ()>,
}

impl<'r, 'a, T> WidgetMenu<'r, 'a, T> {
    pub fn on(&self, k: Keycode, cb: impl 'a+Fn(&mut T, InputEvent)) -> Interactor {
        let state = self.state.clone();
        self.menu.on(k, move |inp| {
            cb(&mut state.borrow_mut(), inp);
        })
    }

    pub fn on_key(&self, k: Keycode, cb: impl 'a+Fn(&mut T, KeyEvent)) {
        let state = self.state.clone();
        self.menu.on_key(k, move |inp| {
            cb(&mut state.borrow_mut(), inp);
        })
    }

    pub fn on_click(&self, cb: impl 'a+Fn(&mut T, MouseEvent)) -> Interactor {
        let state = self.state.clone();
        self.menu.on_click(move |inp| {
            cb(&mut state.borrow_mut(), inp);
        })
    }

    pub fn on_text(&self, cb: impl 'a+Fn(&mut T, char)) {
        let state = self.state.clone();
        self.menu.on_text(move |inp| {
            cb(&mut state.borrow_mut(), inp);
        })
    }
}