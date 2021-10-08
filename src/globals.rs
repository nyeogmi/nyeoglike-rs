use std::{cell::RefCell, rc::Rc};

use super::reexports::*;

pub type Globals = Rc<GlobalState>;
pub struct GlobalState {
    // UI stuff
    pub ui: UI,
    pub graphics: RefCell<Graphics>,

    // game state
    pub player: RefCell<Player>,
    pub npcs: RefCell<NPCs>,
    pub terrain: RefCell<Terrain>,
}