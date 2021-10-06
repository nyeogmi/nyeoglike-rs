use crate::game::reexports::*;

use super::{graphics::{SCCELL_X, SCCELL_Y}};

impl SiteMode {
    pub fn on_loop(&mut self, globals: &Globals, screen_boundaries: CellRect) {
        // place player if possible
        if let None = self.player_xy {
            self.player_xy = globals.terrain.borrow().suggest_player_xy();
        }

        self.update_visibility(globals, screen_boundaries);
    }

    pub fn on_tick(&mut self, globals: &Globals) {
        self.handle_intent(globals);
    }
}