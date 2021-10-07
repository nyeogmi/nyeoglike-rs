use crate::reexports::*;

impl Player {
    pub fn on_loop(&mut self, globals: &Globals, _screen_boundaries: CellRect) {
        // place player if possible
        if let None = self.xy {
            self.xy = globals.terrain.borrow().suggest_player_xy();
        }
    }

    pub fn on_tick_or_resize(&mut self, globals: &Globals, screen_boundaries: CellRect) {
       self.update_visibility(globals, screen_boundaries);
    }

    pub fn on_tick(&mut self, globals: &Globals) {
        self.handle_intent(globals);
    }
}