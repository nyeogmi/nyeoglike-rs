use std::{cell::RefCell, marker::PhantomData, rc::Rc};

use chiropterm::*;

use crate::widgetry::ui::{UI};

use super::{Widgetlike, common::WidgetCommon};

pub struct WidgetMenu<'gamestate: 'frame, 'frame, T: Widgetlike<'gamestate, Out=Out>, Out> {
    pub(in super) ui: UI,
    pub(in super) state: Rc<RefCell<WidgetCommon<T>>>,
    pub(in super) menu: Menu<'frame, Out>,
    pub(in super) brush_offset: CellVector,
    pub(in super) phantom: PhantomData<&'gamestate ()>,
}

impl<'gamestate: 'frame, 'frame, T: Widgetlike<'gamestate, Out=Out>, Out> WidgetMenu<'gamestate, 'frame, T, Out> {
    pub fn share(&self) -> WidgetMenu<'gamestate, 'frame, T, Out> {
        WidgetMenu {
            ui: self.ui.share(),
            state: self.state.clone(),
            menu: self.menu.share(),
            brush_offset: self.brush_offset,
            phantom: PhantomData,
        }
    }
    pub fn on(&self, k: Keycode, cb: impl 'frame+Fn(UI, &mut WidgetCommon<T>, InputEvent) -> Signal<Out>) -> Interactor {
        let state = self.state.clone();
        let o = self.brush_offset;
        let ui = self.ui.share();
        self.menu.on(k, move |inp| {
            cb(ui.share(), &mut state.borrow_mut(), inp.offset(-o))
        })
    }

    pub fn on_key(&self, k: Keycode, cb: impl 'frame+Fn(UI, &mut WidgetCommon<T>, KeyEvent) -> Signal<Out>) {
        let state = self.state.clone();
        let ui = self.ui.share();
        self.menu.on_key(k, move |inp| {
            cb(ui.share(), &mut state.borrow_mut(), inp)
        })
    }

    pub fn on_click(&self, cb: impl 'frame+Fn(UI, &mut WidgetCommon<T>, MouseEvent) -> Signal<Out>) -> Interactor {
        let state = self.state.clone();
        let o = self.brush_offset;
        let ui = self.ui.share();
        self.menu.on_click(move |inp| {
            cb(ui.share(), &mut state.borrow_mut(), inp.offset(-o))
        })
    }

    pub fn on_text(&self, cb: impl 'frame+Fn(UI, &mut WidgetCommon<T>, char) -> Signal<Out>) {
        let state = self.state.clone();
        let ui = self.ui.share();
        self.menu.on_text(move |inp| {
            cb(ui.share(), &mut state.borrow_mut(), inp)
        })
    }
}