use std::any::Any;

use super::{Widget, Widgetlike};

pub struct AnyWidget {
    implementation: Box<dyn AWidget>
}

impl<T: Widgetlike> AWidget for Widget<T> {

}