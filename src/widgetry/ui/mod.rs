mod selection;

use std::{cell::Cell, rc::Rc};

pub use self::selection::Selection;

use super::{WidgetCommon, Widgetlike};

pub struct UISource {
    selection: Cell<Selection>,
    layout_token: Cell<u64>,
}

#[derive(Clone)]
pub struct UI {
    state: Rc<UISource>, 
}

impl UI {
    pub fn new() -> UI {
        UI {
            state: Rc::new(UISource { 
                selection: Cell::new(Selection::none()),
                layout_token: Cell::new(0),
            })
        }
    }
    pub fn share(&self) -> UI {
        UI { state: self.state.clone() }
    }

    pub fn select<'a, T: Widgetlike<'a, Out=Out>, Out>(&self, widg: &mut WidgetCommon<T>) {
        self.state.selection.replace(self.state.selection.get().advance());
        widg.selection = self.state.selection.get();
    }

    pub fn deselect<'a, T: Widgetlike<'a, Out=Out>, Out>(&self, _widg: &mut WidgetCommon<T>) {
        self.state.selection.replace(self.state.selection.get().advance());
    }

    pub fn is_selected(&self, other: Selection) -> bool {
        self.state.selection.get() == other
    }

    pub(crate) fn recompute_layout(&self) {
        self.state.layout_token.replace(self.state.layout_token.get() + 1);
    }

    pub(in crate::widgetry) fn layout_token(&self) -> u64 {
        self.state.layout_token.get()
    }
}