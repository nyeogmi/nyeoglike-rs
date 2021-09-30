mod common;
mod dimensions;
mod menu;
mod polymorphic;

use std::{cell::{Ref, RefCell, RefMut}, marker::PhantomData, rc::Rc};

use chiropterm::*;

pub use self::common::WidgetCommon;
pub use self::dimensions::WidgetDimensions;
pub use self::menu::WidgetMenu;
pub(in crate::widgetry) use self::polymorphic::AnyWidget;

use super::UI;

pub struct Widget<'draw, T: Widgetlike> {
    // TODO: Instead use a ref inside an arena allocator (not bump, we need drop)
    state: Rc<RefCell<WidgetCommon<T>>>,

    // TODO: Move this to state?
    phantom: PhantomData<&'draw ()>, 
}

impl<'draw, T: 'draw+Widgetlike> Widget<'draw, T> {
    pub fn new() -> Self {
        Widget { 
            state: Rc::new(RefCell::new(WidgetCommon::new(T::default()))),
            phantom: PhantomData,
        }
    }

    pub fn share(&self) -> Self {
        Widget { 
            state: self.state.clone(), 
            phantom: PhantomData,
        }
    }

    pub fn borrow(&self) -> Ref<WidgetCommon<T>> {
        self.state.borrow()
    }

    pub fn borrow_mut(&self) -> RefMut<WidgetCommon<T>> {
        self.state.borrow_mut()
    }

    pub fn draw(&self, ui: UI<'draw>, brush: Brush, menu: Menu<'draw, ()>) {
        let brush = self.estimate_dimensions(brush.rect().width()).tailor(brush);
        let offset = brush.cursor_offset();
        let widget_menu = WidgetMenu { 
            ui, state: self.state.clone(), menu, brush_offset: offset 
        };
        self.draw_internal(brush, widget_menu)
    }

    pub fn support_polydraw<X: Widgetlike>(&self, brush: Brush, menu: WidgetMenu<'draw, X>) {
        let brush = self.estimate_dimensions(brush.rect().width()).tailor(brush);
        let offset = brush.cursor_offset();
        let widget_menu = WidgetMenu { 
            ui: menu.ui, 
            state: self.state.clone(), 
            menu: menu.menu, 
            brush_offset: brush.cursor_offset() 
        };
        self.draw_internal(brush, widget_menu)
    }

    pub(in super) fn draw_internal(&self, brush: Brush, widget_menu: WidgetMenu<'draw, T>) {
        self.state.borrow().draw(brush, widget_menu);
    }

    pub fn estimate_dimensions(&self, mut width: isize) -> WidgetDimensions {
        self.state.borrow().estimate_dimensions(width)
    }
}

pub trait Widgetlike: Default+Sized {
    fn draw(&self, selected: bool, brush: Brush, menu: WidgetMenu<Self>);
    fn estimate_dimensions(&self, width: isize) -> WidgetDimensions;
}