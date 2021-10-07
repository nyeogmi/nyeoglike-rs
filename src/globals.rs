use std::{cell::RefCell, rc::Rc};

use crate::terrain::Terrain;

use super::reexports::*;

pub type Globals = Rc<GlobalState>;
pub struct GlobalState {
    // UI stuff
    pub ui: UI,

    // game state
    pub player: Rc<RefCell<Player>>,
    pub terrain: Rc<RefCell<Terrain>>,
}