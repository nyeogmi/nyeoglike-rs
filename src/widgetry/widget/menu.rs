use std::{cell::RefCell, rc::Rc};

use chiropterm::*;

use crate::widgetry::ui::{UI};

use super::{Widgetlike, common::WidgetCommon};

pub struct WidgetMenu<'draw, T: 'draw+Widgetlike> {
    pub(in super) ui: UI<'draw>,
    pub(in super) state: Rc<RefCell<WidgetCommon<T>>>,
    pub(in super) menu: Menu<'draw, ()>,
    pub(in super) brush_offset: CellVector,
}

impl<'draw, T: 'draw+Widgetlike> WidgetMenu<'draw, T> {
    pub fn on(&self, k: Keycode, cb: impl 'draw+Fn(UI, &mut WidgetCommon<T>, InputEvent)) -> Interactor {
        let state = self.state.clone();
        let o = self.brush_offset;
        let ui = self.ui;
        self.menu.on(k, move |inp| {
            cb(ui, &mut state.borrow_mut(), inp.offset(-o));
        })
    }

    pub fn on_key(&self, k: Keycode, cb: impl 'draw+Fn(UI, &mut WidgetCommon<T>, KeyEvent)) {
        let state = self.state.clone();
        let ui = self.ui;
        self.menu.on_key(k, move |inp| {
            cb(ui, &mut state.borrow_mut(), inp);
        })
    }

    pub fn on_click(&self, cb: impl 'draw+Fn(UI, &mut WidgetCommon<T>, MouseEvent)) -> Interactor {
        let state = self.state.clone();
        let o = self.brush_offset;
        let ui = self.ui;
        self.menu.on_click(move |inp| {
            cb(ui, &mut state.borrow_mut(), inp.offset(-o));
        })
    }

    pub fn on_text(&self, cb: impl 'draw+Fn(UI, &mut WidgetCommon<T>, char)) {
        let state = self.state.clone();
        let ui = self.ui;
        self.menu.on_text(move |inp| {
            cb(ui, &mut state.borrow_mut(), inp);
        })
    }
}