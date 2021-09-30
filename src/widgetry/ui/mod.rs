mod selection;

use std::{cell::RefCell, rc::Rc};

pub use self::selection::Selection;

use super::{WidgetCommon, Widgetlike};

pub struct UI {
    selection: Selection,
}

impl UI {
    pub fn new() -> Rc<RefCell<UI>> {
        Rc::new(RefCell::new(UI { 
            selection: Selection::none(),
        }))
    }

    pub fn select<T: Widgetlike>(&mut self, widg: &mut WidgetCommon<T>) {
        self.selection = self.selection.advance();
        widg.selection = self.selection;
    }

    pub fn deselect<T: Widgetlike>(&mut self, _widg: &mut WidgetCommon<T>) {
        self.selection = self.selection.advance()
    }

    pub fn is_selected(&self, other: Selection) -> bool {
        self.selection == other
    }
}