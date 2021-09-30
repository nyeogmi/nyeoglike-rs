use std::{cell::RefCell, rc::Rc};

use chiropterm::*;

pub struct Widget<T: Widgetlike> {
    state: Rc<RefCell<T>>,
}

pub struct Command<'a>(Box<dyn 'a+FnMut()>);

impl<'a> Command<'a> {
    pub fn run(mut self) {
        (self.0)();
    }
}

impl<T: 'static+Widgetlike> Widget<T> {
    pub fn new(state: T) -> Self {
        Widget { 
            state: Rc::new(RefCell::new(state))
        }
    }

    pub fn draw<X: Brushable>(&self, brush: Brush<X>, menu: &Menu<()>) {
        self.state.borrow().draw(brush, &WidgetMenu { state: self.state.clone(), menu });
    }
}

pub trait Widgetlike: Sized {
    fn draw<T: Brushable>(&self, brush: Brush<T>, menu: &WidgetMenu<Self>);
}

pub struct WidgetMenu<'r, 'a, T: 'a> {
    state: Rc<RefCell<T>>,
    menu: &'r Menu<'a, ()>,
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
}