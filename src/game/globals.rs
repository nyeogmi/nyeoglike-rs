use std::{cell::RefCell, rc::Rc};

use super::reexports::*;

pub struct Globals {
    // UI stuff
    pub ui: UI,

    // game state
    pub sitemode: Rc<RefCell<SiteMode>>,
}