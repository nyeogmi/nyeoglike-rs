mod selection;

use std::{cell::Cell, rc::Rc};

pub use self::selection::Selection;

use super::{WidgetCommon, Widgetlike};

pub struct UISource {
    selection: Cell<Selection>,
}

#[derive(Clone)]
pub struct UI {
    state: Rc<UISource>, 
}

impl UI {
    pub fn new() -> UI {
        UI {
            state: Rc::new(UISource { selection: Cell::new(Selection::none())})
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
}