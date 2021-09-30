mod selection;

use std::cell::Cell;

use chiropterm::{IO, Menu, Screen};

pub use self::selection::Selection;

use super::{WidgetCommon, Widgetlike};

struct UIState {
    selection: Cell<Selection>,
}

#[derive(Clone, Copy)]
pub struct UI<'gamestate> {
    state: &'gamestate UIState, 
}

impl<'gamestate> UI<'gamestate> {
    pub fn host(
        io: &mut IO, 
        mut body: impl for<'draw> FnMut(UI<'draw>, &Screen, Menu<'draw, ()>)
    ) {
        let ui_state = UIState {
            selection: Cell::new(Selection::none()),
        };
        let ui = UI { 
            state: &ui_state
        };

        loop {
            io.menu(|out, menu| {
                body(ui, out, menu)
            })
        }
    }

    pub fn select<T: Widgetlike>(&self, widg: &mut WidgetCommon<T>) {
        self.state.selection.replace(self.state.selection.get().advance());
        widg.selection = self.state.selection.get();
    }

    pub fn deselect<T: Widgetlike>(&self, _widg: &mut WidgetCommon<T>) {
        self.state.selection.replace(self.state.selection.get().advance());
    }

    pub fn is_selected(&self, other: Selection) -> bool {
        self.state.selection.get() == other
    }
}