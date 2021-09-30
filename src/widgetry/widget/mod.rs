mod common;
mod dimensions;
mod menu;

use std::{cell::{Cell, RefCell}, rc::Rc};

use chiropterm::*;

pub use self::common::WidgetCommon;
pub use self::dimensions::WidgetDimensions;
pub use self::menu::WidgetMenu;

use super::UI;

pub struct Widget<T: Widgetlike> {
    state: Rc<RefCell<WidgetCommon<T>>>,
    last_dimensions: Cell<(isize, WidgetDimensions)>,
}

impl<T: 'static+Widgetlike> Widget<T> {
    pub fn new() -> Self {
        Widget { 
            state: Rc::new(RefCell::new(WidgetCommon::new(T::default()))),
            last_dimensions: Cell::new((-1, WidgetDimensions { 
                min: CellSize::zero(), 
                preferred: CellSize::zero(), 
                max: CellSize::zero() 
            })),
        }
    }

    pub fn draw<'draw, X: Brushable>(&self, ui: UI<'draw>, brush: Brush<X>, menu: Menu<'draw, ()>) {
        let brush = self.estimate_dimensions(brush.rect().width()).tailor(brush);
        let offset = brush.cursor_offset();
        let widget_menu = WidgetMenu { ui, state: self.state.clone(), menu, brush_offset: offset };
        self.state.borrow().draw(brush, &widget_menu);
    }

    pub fn estimate_dimensions(&self, mut width: isize) -> WidgetDimensions {
        if width < 0 { width = 0; }
        // TODO: If it's 0, provide a stock answer

        let (last_width, last_dims) = self.last_dimensions.get();
        if last_width == width {
            return last_dims;
        }

        let new_dims = self.state.borrow().estimate_dimensions(width).fixup();
        self.last_dimensions.replace((last_width, new_dims));
        new_dims
    }
}

pub trait Widgetlike: Default+Sized {
    fn draw<T: Brushable>(&self, selected: bool, brush: Brush<T>, menu: &WidgetMenu<Self>);
    fn estimate_dimensions(&self, width: isize) -> WidgetDimensions;
}