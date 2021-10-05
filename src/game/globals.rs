use std::{cell::RefCell, rc::Rc};

use crate::terrain::Terrain;

use super::reexports::*;

pub struct Globals {
    // UI stuff
    pub ui: UI,

    // game state
    pub sitemode: Rc<RefCell<SiteMode>>,
    pub terrain: Rc<RefCell<Terrain>>,
}