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

pub struct Widget<'draw, T: Widgetlike<'draw, Out=Out>, Out> {
    // TODO: Instead use a ref inside an arena allocator (not bump, we need drop)
    state: Rc<RefCell<WidgetCommon<T>>>,

    // TODO: Move this to state?
    phantom: PhantomData<&'draw ()>, 
}

impl<'gamestate, T: Widgetlike<'gamestate, Out=Out>, Out> Widget<'gamestate, T, Out> {
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

    pub fn draw<'frame>(&self, ui: UI, brush: Brush, menu: Menu<'frame, Out>) 
    where 'gamestate: 'frame {
        let brush = self.estimate_dimensions(brush.rect().width()).tailor(brush);
        let offset = brush.cursor_offset();
        let widget_menu = WidgetMenu { 
            ui, state: self.state.clone(), menu, brush_offset: offset, phantom: PhantomData
        };
        self.state.borrow().draw(brush, widget_menu);
    }

    pub fn estimate_dimensions(&self, width: isize) -> WidgetDimensions {
        self.state.borrow().estimate_dimensions(width)
    }
}

pub trait Widgetlike<'gamestate>: 'gamestate+Default+Sized {
    type Out: 'gamestate;

    fn draw<'frame>(&self, selected: bool, brush: Brush, menu: WidgetMenu<'gamestate, 'frame, Self, Self::Out>);
    fn estimate_dimensions(&self, width: isize) -> WidgetDimensions;
}