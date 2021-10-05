use std::{cell::RefCell, marker::PhantomData, rc::Rc};

use chiropterm::*;

use crate::widgetry::ui::{UI, UIContext};

use super::{Widgetlike, common::WidgetCommon};

pub struct WidgetMenu<'gamestate: 'frame, 'frame, T: Widgetlike<'gamestate>> {
    pub ui: UI,
    pub(in super) state: Rc<RefCell<WidgetCommon<T>>>,
    pub menu: Menu<'frame>,
    pub(in super) brush_offset: CellVector,
    pub(in super) phantom: PhantomData<&'gamestate ()>,
}

impl<'gamestate: 'frame, 'frame, T: Widgetlike<'gamestate>> WidgetMenu<'gamestate, 'frame, T> {
    pub fn share(&self) -> WidgetMenu<'gamestate, 'frame, T> {
        WidgetMenu {
            ui: self.ui.share(),
            state: self.state.clone(),
            menu: self.menu.share(),
            brush_offset: self.brush_offset,
            phantom: PhantomData,
        }
    }

    pub fn on_key(&self, k: Keycode, cb: impl 'frame+Fn(UI, &mut WidgetCommon<T>, KeyEvent) -> Signal) {
        let state = self.state.clone();
        let ui = self.ui.share();
        self.menu.on_key(k, move |inp| {
            cb(ui.share(), &mut state.borrow_mut(), inp)
        })
    }

    pub fn on_key_hprio(&self, k: Keycode, cb: impl 'frame+Fn(UI, &mut WidgetCommon<T>, KeyEvent) -> Signal) {
        let state = self.state.clone();
        let ui = self.ui.share();
        self.menu.on_key_hprio(k, move |inp| {
            cb(ui.share(), &mut state.borrow_mut(), inp)
        })
    }

    pub fn on_click(&self, cb: impl 'frame+Fn(UI, &mut WidgetCommon<T>, MouseEvent) -> Signal) -> Interactor {
        let state = self.state.clone();
        let o = self.brush_offset;
        let ui = self.ui.share();
        self.menu.on_click(move |inp| {
            cb(ui.share(), &mut state.borrow_mut(), inp.offset(-o))
        })
    }

    pub fn on_text(&self, cb: impl 'frame+Fn(UI, &mut WidgetCommon<T>, char) -> Signal) {
        let state = self.state.clone();
        let ui = self.ui.share();
        self.menu.on_text(move |inp| {
            cb(ui.share(), &mut state.borrow_mut(), inp)
        })
    }

    pub(crate) fn on_text_hprio(&self, cb: impl 'frame+Fn(UI, &mut WidgetCommon<T>, char) -> Signal) {
        let state = self.state.clone();
        let ui = self.ui.share();
        self.menu.on_text_hprio(move |inp| {
            cb(ui.share(), &mut state.borrow_mut(), inp)
        })
    }

    pub(crate) fn with_context(mut self, on_ctx: impl FnOnce(&mut UIContext)) -> Self {
        self.ui = self.ui.with_context(on_ctx);
        self
    }
}