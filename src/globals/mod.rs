mod cursor;

use super::reexports::*;
pub use cursor::*;

pub type Globals = Rc<GlobalState>;
pub struct GlobalState {
    // UI stuff
    pub ui: UI,
    pub graphics: RefCell<Graphics>,

    // game state
    pub player: RefCell<Player>,
    pub npcs: NPCs,
    pub terrain: Terrain, // all methods only require &Terrain currently
}

impl GlobalState {
    pub fn at(&self, point: GlobalPoint) -> Cursor<'_> {
        Cursor { 
            globals: self,
            point,
        }
    }
}